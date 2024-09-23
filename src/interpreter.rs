use std::collections::HashMap;
use std::io::{self};

use crate::parser::{Declaration, Expr, Statement};
use crate::token::TokenKind as TK;

#[derive(Debug)]
pub enum RuntimeError {
    MissingVariable,
    NoVariableEnvironment,
}

type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Variables {
    variables: Vec<HashMap<String, V>>,
    return_value: Option<V>,
}

impl Variables {
    fn add(&mut self, s: String, value: V) -> Result<()> {
        self.variables
            .last_mut()
            .ok_or(RuntimeError::NoVariableEnvironment)?
            .insert(s, value);
        Ok(())
    }
    fn begin_scope(&mut self) {
        self.variables.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.variables.pop();
    }
    fn new() -> Variables {
        Self {
            variables: vec![HashMap::new()],
            return_value: None,
        }
    }
    fn get(&mut self, name: &str) -> Result<&V> {
        for v in self.variables.iter().rev() {
            if let Some(res) = v.get(name) {
                return Ok(res);
            }
        }
        Err(RuntimeError::MissingVariable)
    }
    fn get_mut(&mut self, name: &str) -> Result<&mut V> {
        for v in self.variables.iter_mut().rev() {
            if let Some(res) = v.get_mut(name) {
                return Ok(res);
            }
        }
        Err(RuntimeError::MissingVariable)
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(declarations: Vec<Declaration>) -> Result<()> {
        let mut vars = Variables::new();
        for decl in declarations {
            println!("parsing: {:?}", decl);
            decl.interpret(&mut vars)?;
        }
        Ok(())
    }
}

impl Declaration {
    pub fn interpret(&self, vars: &mut Variables) -> Result<()> {
        match self {
            Declaration::Function(name, params, stmts) => {
                vars.add(name.to_string(), V::Func(params.to_vec(), stmts.to_vec()))
            }
            Declaration::Statement(stmt) => stmt.interpret(vars),
        }
    }
}

impl Statement {
    pub fn interpret(&self, vars: &mut Variables) -> Result<()> {
        match self {
            Statement::Expression(expr) => {
                _ = expr.interpret(vars)?;
            }
            Statement::Let(name, expr) => {
                let expr = expr.interpret(vars)?;
                vars.add(name.to_string(), expr)?;
            }
            Statement::For(i, expr, stmts) => {
                vars.begin_scope();
                let list = expr.interpret(vars)?.as_list();
                for item in list {
                    vars.add(i.to_string(), item)?;
                    for stmt in stmts {
                        stmt.interpret(vars)?;
                    }
                }
                vars.end_scope();
            },
            Statement::If(expr, if_stmts, else_stmts) => {
                vars.begin_scope();
                if expr.interpret(vars)?.as_bool() {
                    for stmt in if_stmts {
                        stmt.interpret(vars)?
                    }
                } else {
                    for stmt in else_stmts {
                        stmt.interpret(vars)?
                    }
                }
                vars.end_scope();
            }
            Statement::Print(expr) => {
                let value = expr.interpret(vars)?;
                match value {
                    V::String(s) => println!("{s}"),
                    V::Number(n) => println!("{n}"),
                    V::Bool(b) => println!("{b}"),
                    V::Obj(o) => println!("{:?}", o),
                    V::Func(..) => todo!("print function"),
                    V::Null => println!("NULL"),
                    V::List(..) => todo!("print list"),
                }
            }
            Statement::Return(expr) => {
                // set return value in variables?
                let v = expr.interpret(vars)?;
                vars.return_value = Some(v);
            }
            Statement::While(expr, stmts) => {
                vars.begin_scope();
                while expr.interpret(vars)?.as_bool() {
                    for stmt in stmts {
                        stmt.interpret(vars)?;
                    }
                }
                vars.end_scope();
            }
            Statement::Assignment(..) => todo!("maybe remove"),
        }
        Ok(())
    }
}

impl Expr {
    pub fn interpret(&self, vars: &mut Variables) -> Result<V> {
        let res = match self {
            Expr::Prefix(token_kind, expr) => match token_kind {
                TK::Minus => V::Number(-expr.interpret(vars)?.as_num()),
                TK::Bang => V::Bool(!expr.interpret(vars)?.as_bool()),
                _ => panic!("not valid prefixtoken '{:?}'", token_kind),
            },
            Expr::Operator(lhs, token_kind, rhs) => {
                let expr1 = lhs.interpret(vars)?;
                let expr2 = rhs.interpret(vars)?;
                match (expr1, token_kind, expr2) {
                    /*
                     *   Arithmetic
                     */
                    (V::Number(n1), TK::Plus, V::Number(n2)) => V::Number(n1 + n2),
                    (V::Number(n1), TK::Minus, V::Number(n2)) => V::Number(n1 - n2),
                    (V::Number(n1), TK::Slash, V::Number(n2)) => V::Number(n1 / n2),
                    (V::Number(n1), TK::Star, V::Number(n2)) => V::Number(n1 * n2),
                    // TODO
                    // (Value::Number(f1), TokenKind::Percent, Value::Number(f2)) => {
                    //     Value::Number(f1 % f2)
                    // }

                    /*
                     *   Concatenation
                     */
                    (V::String(s1), TK::Plus, V::String(s2)) => V::String(String::from(s1 + &s2)),
                    (V::String(s), TK::Plus, V::Bool(b)) => {
                        V::String(String::from(s + &b.to_string()))
                    }
                    (V::String(s), TK::Plus, V::Number(n)) => {
                        V::String(String::from(s + &n.to_string()))
                    }
                    (V::Number(n), TK::Plus, V::String(s)) => {
                        V::String(String::from(n.to_string() + &s))
                    }

                    /*
                     *   Compare
                     */
                    // Bools
                    (V::Bool(b1), TK::EqualEqual, V::Bool(b2)) => V::Bool(b1 == b2), // ==
                    (V::Bool(b1), TK::BangEqual, V::Bool(b2)) => V::Bool(b1 != b2),  // !=

                    // Strings
                    (V::String(s1), TK::EqualEqual, V::String(s2)) => V::Bool(s1 == s2), // ==
                    (V::String(s1), TK::BangEqual, V::String(s2)) => V::Bool(s1 != s2),  // !=

                    // Nubmers
                    (V::Number(n1), TK::EqualEqual, V::Number(n2)) => V::Bool(n1 == n2), // ==
                    (V::Number(n1), TK::BangEqual, V::Number(n2)) => V::Bool(n1 != n2),  // !=
                    (V::Number(n1), TK::Greater, V::Number(n2)) => V::Bool(n1 > n2),     // >
                    (V::Number(n1), TK::GreaterEqual, V::Number(n2)) => V::Bool(n1 >= n2), // >=
                    (V::Number(n1), TK::Less, V::Number(n2)) => V::Bool(n1 < n2),        // <
                    (V::Number(n1), TK::LessEqual, V::Number(n2)) => V::Bool(n1 <= n2),  // <=

                    /*
                     *   Logical
                     */
                    (V::Bool(b1), TK::And, V::Bool(b2)) => V::Bool(b1 && b2), // and
                    (V::Bool(b1), TK::Or, V::Bool(b2)) => V::Bool(b1 || b2),  // and

                    /*
                     *   Function composition
                     */
                    (V::Obj(o), TK::Dot, V::String(s)) => o.get(&s).unwrap().clone(),

                    /*
                     *   Reassignment
                     */
                    (V::String(s), TK::Equal, v) => {
                        vars.add(s, v)?;
                        V::Number(0.0)
                    }
                    (a1, a2, a3) => panic!("what is this got: '{:?}', '{:?}', '{:?}'", a1, a2, a3),
                }
            }
            Expr::Number(n) => V::Number(*n),
            Expr::String(s) => V::String(s.to_string()),
            Expr::Bool(b) => V::Bool(*b),
            Expr::Null => V::Null,
            Expr::Variable(s) => vars.get(s)?.clone(),
            Expr::Object(fields) => {
                let mut map = HashMap::new();
                for f in fields {
                    match f {
                        Expr::Operator(identifier, op, expr) => {
                            assert_eq!(op, &TK::Equal);
                            let key = identifier.interpret(vars)?.as_string();
                            map.insert(key, expr.interpret(vars)?);
                        }
                        _ => panic!("must be operator."),
                    }
                }
                V::Obj(map)
            }
            Expr::Get(s, expr) => expr
                .interpret(vars)?
                .as_mut_obj()
                .get_mut(s)
                .unwrap()
                .clone(),
            Expr::Set(s, g_expr, expr) => {
                let resolved = expr.interpret(vars)?;
                resolve_get(g_expr.clone(), vars)?
                    .as_mut_obj()
                    .insert(s.to_string(), resolved);
                V::Bool(true)
            }
            Expr::Call(params, expr) => {
                vars.begin_scope();
                match expr.interpret(vars)? {
                    V::Func(param_names, stmts) => {
                        if param_names.len() != params.len() {
                            panic!("wrong amount of parameters")
                        }
                        for (name, param_expr) in std::iter::zip(param_names, params) {
                            let res = param_expr.interpret(vars)?;
                            vars.add(name, res)?;
                        }
                        for stmt in stmts {
                            stmt.interpret(vars)?;

                            if vars.return_value.is_some() {
                                vars.end_scope();
                                let val = vars.return_value.as_mut().unwrap().clone();
                                vars.return_value = None;
                                return Ok(val);
                            }
                        }
                    }
                    _ => panic!("Expected function"),
                }
                vars.end_scope();
                V::Null
            }
            Expr::List(items) => V::List(items.iter().map(|x| x.interpret(vars).unwrap()).collect()),
            Expr::Index(list, index) => match list.interpret(vars)? {
                V::List(items) => items
                    .get(index.interpret(vars)?.as_num() as usize)
                    .unwrap()
                    .clone(),
                _ => panic!("cant index non-list"),
            },
            Expr::SetList(list, index, new) => {
                let new_val = new.interpret(vars)?;
                let idx = index.interpret(vars)?.as_num() as usize;
                let resolved = resolve_get(list.clone(), vars)?.as_mut_list();
                resolved[idx] = new_val;
                V::Null
            }
            Expr::ReadFile(expr) => {
                println!("trying to read file: {:?}", expr);
                let file_path = expr.interpret(vars)?.as_string();
                match std::fs::read_to_string(file_path.clone()) {
                    Ok(s) => V::String(s),
                    Err(_) => panic!("Error reading file: {}", file_path),
                }
            }
            Expr::ReadInput => {
                println!("input prompt: ");
                let mut buffer = String::new();
                match io::stdin().read_line(&mut buffer) {
                    Ok(_) => V::String(buffer),
                    Err(_) => panic!("Error getting input"),
                }
            }
        };
        Ok(res)
    }
}

fn resolve_get(expr: Box<Expr>, vars: &mut Variables) -> Result<&mut V> {
    println!("!resolve_get! {:?}", expr);
    match *expr.clone() {
        Expr::Get(s, expr) => Ok(resolve_get(expr, vars)?.as_mut_obj().get_mut(&s).unwrap()),
        Expr::Variable(s) => vars.get_mut(&s),
        _ => panic!("must be Get"),
    }
}

#[derive(Debug, Clone)]
pub enum V {
    String(String),
    Number(f64),
    Bool(bool),
    Obj(HashMap<String, V>),
    Func(Vec<String>, Vec<Statement>),
    Null,
    List(Vec<V>),
}

impl V {
    fn as_mut_list(&mut self) -> &mut Vec<V> {
        match self {
            V::List(l) => l,
            V::Obj(_) | V::Null | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }

    fn as_list(self) -> Vec<V> {
        match self {
            V::List(l) => l,
            V::Obj(_) | V::Null | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }

    fn as_mut_obj(&mut self) -> &mut HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            V::List(_) | V::Null | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }
    fn as_obj(self) -> HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            V::List(_) | V::Null | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }
    fn as_num(self) -> f64 {
        match self {
            V::Number(n) => n,
            V::List(_) | V::Null | V::String(_) | V::Bool(_) | V::Obj(_) | V::Func(..) => {
                panic!("not a number.")
            }
        }
    }
    fn as_bool(self) -> bool {
        match self {
            V::Bool(b) => b,
            V::List(_) | V::Null | V::String(_) | V::Number(_) | V::Obj(_) | V::Func(..) => {
                panic!("not a bool.")
            }
        }
    }

    fn as_string(self) -> String {
        match self {
            V::String(s) => s,
            V::List(_) | V::Null | V::Bool(_) | V::Number(_) | V::Obj(_) | V::Func(..) => {
                panic!("not a string.")
            }
        }
    }
}
