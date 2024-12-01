use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, Write};

use crate::error::Error;
use crate::parser::{Expr, Declaration, ExprKind, Statement};
use crate::token::TokenKind as TK;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Variables {
    variables: Vec<HashMap<String, V>>,
    return_value: Option<V>,
}

fn gen_err(s: String) -> Error {
    Error {
        line: 0,
        start: 0,
        end: 0,
        msg: s
    }
}

impl Variables {
    fn re_assign(&mut self, s: String, value: V) -> Result<V> {
        for v in &mut self.variables {
            if v.contains_key(&s) {
                v.insert(s.to_string(), value.clone());
                return Ok(value);
            }
        }
        Err(gen_err(format!("Could not find variable: {s}")))
    }
    fn get_scope(&self) -> Result<HashMap<String, V>> {
        Ok(self
            .variables
            .last()
            .ok_or(gen_err(format!("Found no Variable Environment")))?
            .clone())
    }
    fn add_scope(&mut self, env: Option<HashMap<String, V>>) -> Result<()> {
        if let Some(e) = env {
            for (k, v) in e {
                self.add(k, v)?;
            }
        }
        Ok(())
    }
    fn add(&mut self, s: String, value: V) -> Result<()> {
        self.variables
            .last_mut()
            .ok_or(gen_err(format!("Found no Variable Environment")))?
            .insert(s, value);
        Ok(())
    }
    fn begin_scope(&mut self) {
        self.variables.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.variables.pop();
    }
    fn new_with_native_fns() -> Variables {
        Self {
            variables: vec![HashMap::from([
                ("print".to_string(), V::NativeFunc(1, NativeFunction::Print)),
                (
                    "read_file".to_string(),
                    V::NativeFunc(1, NativeFunction::ReadFile),
                ),
                (
                    "read_input".to_string(),
                    V::NativeFunc(1, NativeFunction::ReadInput),
                ),
                (
                    "append".to_string(),
                    V::NativeFunc(2, NativeFunction::Append),
                ),
                ("split".to_string(), V::NativeFunc(2, NativeFunction::Split)),
                (
                    "split_lines".to_string(),
                    V::NativeFunc(1, NativeFunction::SplitLines),
                ),
                ("len".to_string(), V::NativeFunc(1, NativeFunction::Len)),
                ("parse".to_string(), V::NativeFunc(1, NativeFunction::Parse)),
                (
                    "typeof".to_string(),
                    V::NativeFunc(1, NativeFunction::Typeof),
                ),
                (
                    "random".to_string(),
                    V::NativeFunc(1, NativeFunction::Random),
                ),
            ])],
            return_value: None,
        }
    }
    fn get(&mut self, name: &str) -> Result<&V> {
        for v in self.variables.iter().rev() {
            if let Some(res) = v.get(name) {
                return Ok(res);
            }
        }
        Err(gen_err(format!("Could not find variable '{}'", name)))
    }
    fn get_mut(&mut self, name: &str) -> Result<&mut V> {
        for v in self.variables.iter_mut().rev() {
            if let Some(res) = v.get_mut(name) {
                return Ok(res);
            }
        }
        Err(gen_err(format!("Could not find variable '{}'", name)))
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(declarations: Vec<Declaration>) -> Result<()> {
        let mut vars = Variables::new_with_native_fns();

        for decl in declarations {
            // println!("parsing: {:?}", decl);
            decl.interpret(&mut vars)?;
        }
        Ok(())
    }
}

impl Declaration {
    fn interpret(&self, vars: &mut Variables) -> Result<()> {
        match self {
            Declaration::Function(name, params, stmts) => vars.add(
                name.to_string(),
                V::Func(params.to_vec(), stmts.to_vec(), None),
            ),
            Declaration::Statement(stmt) => stmt.interpret(vars),
            Declaration::Import(_) => unreachable!("Imports are handled during parsing"),
        }
    }
}

impl Statement {
    fn interpret(&self, vars: &mut Variables) -> Result<()> {
        match self {
            Statement::Expression(expr) => {
                _ = expr.expr.interpret(vars)?;
            }
            Statement::Let(name, expr) => {
                let expr = expr.expr.interpret(vars)?;
                vars.add(name.to_string(), expr)?;
            }
            Statement::For(i, expr, stmts) => {
                let list = expr.expr.interpret(vars)?.as_list();
                for item in list {
                    vars.begin_scope();
                    vars.add(i.to_string(), item)?;
                    for stmt in stmts {
                        stmt.interpret(vars)?;
                    }
                    vars.end_scope();
                }
            }
            Statement::If(expr, if_stmts, else_stmts) => {
                vars.begin_scope();
                if expr.expr.interpret(vars)?.as_bool() {
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
            Statement::Return(expr) => {
                // set return value in variables?
                let v = expr.expr.interpret(vars)?;
                vars.return_value = Some(v);
            }
            Statement::While(expr, stmts) => {
                vars.begin_scope();
                while expr.expr.interpret(vars)?.as_bool() {
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

impl ExprKind {
    fn interpret(&self, vars: &mut Variables) -> Result<V> {
        let res = match self {
            ExprKind::Prefix(token_kind, expr) => match token_kind {
                TK::Minus => V::Number(-expr.expr.interpret(vars)?.as_num()),
                TK::Bang => V::Bool(!expr.expr.interpret(vars)?.as_bool()),
                _ => panic!("not valid prefixtoken '{:?}'", token_kind),
            },
            ExprKind::Operator(lhs, token_kind, rhs) => {
                let expr1 = lhs.expr.interpret(vars)?;
                let expr2 = rhs.expr.interpret(vars)?;
                match (expr1, token_kind, expr2) {
                    /*
                     *   Arithmetic
                     */
                    (V::Number(n1), TK::Plus, V::Number(n2)) => V::Number(n1 + n2),
                    (V::Number(n1), TK::Minus, V::Number(n2)) => V::Number(n1 - n2),
                    (V::Number(n1), TK::Slash, V::Number(n2)) => V::Number(n1 / n2),
                    (V::Number(n1), TK::Star, V::Number(n2)) => V::Number(n1 * n2),
                    (V::Number(f1), TK::Percent, V::Number(f2)) => V::Number(f1 % f2),

                    /*
                     *   List Concatenation
                     */
                    (V::List(l1), TK::Plus, V::List(l2)) => V::List([l1, l2].concat()),
                    /*
                     *   Concatenation
                     */
                    (e1, TK::Plus, e2) => V::String(format!("{}{}", e1, e2)),

                    /*
                     *   Compare
                     */
                    // Bools
                    (V::Bool(b1), TK::EqualEqual, V::Bool(b2)) => V::Bool(b1 == b2), // ==
                    (V::Bool(b1), TK::BangEqual, V::Bool(b2)) => V::Bool(b1 != b2),  // !=

                    // Strings
                    (V::String(s1), TK::EqualEqual, V::String(s2)) => V::Bool(s1 == s2), // ==
                    (V::String(s1), TK::BangEqual, V::String(s2)) => V::Bool(s1 != s2),  // !=

                    // Numbers
                    (V::Number(n1), TK::EqualEqual, V::Number(n2)) => V::Bool(n1 == n2), // ==
                    (V::Number(n1), TK::BangEqual, V::Number(n2)) => V::Bool(n1 != n2),  // !=
                    (V::Number(n1), TK::Greater, V::Number(n2)) => V::Bool(n1 > n2),     // >
                    (V::Number(n1), TK::GreaterEqual, V::Number(n2)) => V::Bool(n1 >= n2), // >=
                    (V::Number(n1), TK::Less, V::Number(n2)) => V::Bool(n1 < n2),        // <
                    (V::Number(n1), TK::LessEqual, V::Number(n2)) => V::Bool(n1 <= n2),  // <=
                    
                    // Null
                    (V::Null, TK::EqualEqual, V::Null) => V::Bool(true),

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
                    (V::String(s), TK::Equal, v) => vars.re_assign(s, v)?,
                    (V::String(s), TK::PlusEqual, v) => {
                        let new_value = match vars.get(&s)? {
                            V::String(ss) => V::String(format!("{}{}", ss, v.as_string())),
                            V::Number(n) => V::Number(n + v.as_num()),
                            V::Bool(_) => {
                                return Err(gen_err(format!("Cant concatenate bool {s}")))
                            }
                            V::Obj(_) => {
                                return Err(gen_err(format!("Cant concatenate object {s}")))
                            }
                            V::Func(_, _, _) => {
                                return Err(gen_err(format!("Cant concatenate function {s}")))
                            }
                            V::NativeFunc(_, _) => {
                                return Err(gen_err(format!("Cant concatenate function {s}")))
                            }
                            V::Null => return Err(gen_err(format!("Cant concatenate null {s}"))),
                            V::List(l) => V::List([l.clone(), v.as_list()].concat()),
                        };
                        vars.re_assign(s, new_value)?
                    }
                    (V::String(s), TK::MinusEqual, v) => {
                        let new_value = match vars.get(&s)? {
                            V::Number(n) => V::Number(n - v.as_num()),
                            _ => {
                                return Err(gen_err(format!(
                                    "Only numbers can be used with the '-=' operator"
                                )))
                            }
                        };
                        vars.re_assign(s, new_value)?
                    }

                    (V::Number(n1), TK::Colon, V::Number(n2)) => V::List(
                        ((n1 as usize)..(n2 as usize))
                            .map(|x| V::Number(x as f64))
                            .collect(),
                    ),
                    (e1, TK::Arrow, V::Func(param_names, stmts, env)) => {
                        if param_names.len() != 1 {
                            return Err(gen_err(format!(
                                "Function can only have 1 parameter when chaining with '->'"
                            )));
                        }
                        vars.begin_scope();
                        vars.add(param_names[0].to_string(), e1)?;
                        vars.add_scope(env)?;
                        for stmt in stmts {
                            stmt.interpret(vars)?;
                            if vars.return_value.is_some() {
                                let val = vars.return_value.as_mut().unwrap().clone();
                                vars.return_value = None;
                                vars.end_scope();
                                return Ok(val);
                            }
                        }
                        vars.end_scope();
                        V::Null
                    }
                    (e1, TK::Arrow, V::NativeFunc(_, kind)) => exec_native_fn(kind, vec![e1])?,
                    (a1, a2, a3) => panic!(
                        "Unknown operator expression: '{:?}', '{:?}', '{:?}'",
                        a1, a2, a3
                    ),
                }
            }
            ExprKind::Number(n) => V::Number(*n),
            ExprKind::String(s) => V::String(s.to_string()),
            ExprKind::Bool(b) => V::Bool(*b),
            ExprKind::Null => V::Null,
            ExprKind::Variable(s) => vars.get(s)?.clone(),
            ExprKind::Object(fields) => {
                let mut map = HashMap::new();
                for f in fields {
                    match &f.expr {
                        ExprKind::Operator(identifier, op, expr) => {
                            assert_eq!(op, &TK::Equal);
                            let key = identifier.expr.interpret(vars)?.as_string();
                            map.insert(key, expr.expr.interpret(vars)?);
                        }
                        _ => panic!("must be operator."),
                    }
                }
                V::Obj(map)
            }
            ExprKind::Get(s, expr) => expr.expr
                .interpret(vars)?
                .as_mut_obj()
                .get_mut(s)
                .unwrap()
                .clone(),
            ExprKind::Set(s, g_expr, expr) => {
                let resolved = expr.expr.interpret(vars)?;
                resolve_get(g_expr.clone(), vars)?
                    .as_mut_obj()
                    .insert(s.to_string(), resolved);
                V::Bool(true)
            }
            ExprKind::Call(params, expr) => {
                vars.begin_scope();
                match expr.expr.interpret(vars)? {
                    V::Func(param_names, stmts, env) => {
                        if param_names.len() != params.len() {
                            return Err(Error {
                                line: 0, 
                                start: 0, 
                                    end: 0, 
                                msg: format!("Trying to call a function with {} parameters with {} paramaters", param_names.len(), params.len()),
                            });
                        }
                        vars.add_scope(env)?;

                        for (name, param_expr) in std::iter::zip(param_names, params) {
                            let res = param_expr.expr.interpret(vars)?;
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
                    V::NativeFunc(arity, native_fn) => {
                        if params.len() != arity {
                            panic!("wrong amount of parameters");
                        }
                        let resolved_params: Vec<V> = params
                            .iter()
                            .map(|x| x.expr.interpret(vars))
                            .collect::<Result<Vec<V>>>()?;
                        // TODO: naming
                        let res = exec_native_fn(native_fn, resolved_params);
                        vars.end_scope();
                        return res;
                    }
                    a => panic!("Expected function {:?}", a),
                }
                vars.end_scope();

                V::Null
            }
            ExprKind::List(items) => V::List(
                items
                    .iter()
                    .map(|x| x.expr.interpret(vars))
                    .collect::<Result<Vec<V>>>()?,
            ),
            ExprKind::Index(list, index) => match list.expr.interpret(vars)? {
                V::List(items) => items
                    .get(index.expr.interpret(vars)?.as_num() as usize)
                    .unwrap()
                    .clone(),
                V::String(s) => V::String(
                    s.chars()
                        .nth(index.expr.interpret(vars)?.as_num() as usize)
                        .unwrap()
                        .to_string(),
                ),

                _ => panic!("cant index non-list"),
            },
            ExprKind::SetList(list, index, new) => {
                let new_val = new.expr.interpret(vars)?;
                let idx = index.expr.interpret(vars)?.as_num() as usize;
                let resolved = resolve_get(list.clone(), vars)?.as_mut_list();
                resolved[idx] = new_val;
                V::Null
            }
            ExprKind::Function(params, stmts) => {
                V::Func(params.to_vec(), stmts.to_vec(), Some(vars.get_scope()?))
            }
        };
        Ok(res)
    }
}

fn exec_native_fn(kind: NativeFunction, resolved_params: Vec<V>) -> Result<V> {
    match kind {
        NativeFunction::Print => {
            println!("{}", resolved_params[0]);
            Ok(resolved_params[0].clone())
        }
        NativeFunction::ReadFile => {
            // TODO: remove clone
            let file_path = resolved_params.get(0).unwrap().clone().as_string();
            match std::fs::read_to_string(file_path.clone()) {
                Ok(s) => Ok(V::String(s)),
                Err(_) => panic!("Error reading file: {}", file_path),
            }
        }
        NativeFunction::ReadInput => {
            print!("{}", resolved_params[0]);
            io::stdout().flush().expect("Should not happend");

            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => Ok(V::String(buffer)),
                Err(_) => panic!("Error getting input"),
            }
        }
        NativeFunction::Split => {
            let s = resolved_params[0].clone().as_string();
            let delim = resolved_params[1]
                .clone()
                .as_string()
                .replace("\\n", "\n")
                .replace("\\r", "\r");
            let split: Vec<V> = s
                .split_terminator(&delim)
                .map(|x| V::String(x.to_string().replace("\\r", "")))
                .collect();
            Ok(V::List(split))
        }
        NativeFunction::SplitLines => Ok(V::List(
            resolved_params[0]
                .clone()
                .as_string()
                .lines()
                .map(|x| V::String(x.to_string()))
                .collect(),
        )),
        NativeFunction::Append => {
            let mut new_list = resolved_params[0].clone().as_list();
            new_list.push(resolved_params[1].clone());
            Ok(V::List(new_list))
        }
        NativeFunction::Len => Ok(V::Number(match &resolved_params[0] {
            V::String(s) => s.len() as f64,
            V::Number(_) => todo!(),
            V::Bool(_) => todo!(),
            V::Obj(_) => todo!(),
            V::Func(..) => todo!(),
            V::Null => todo!(),
            V::List(v) => v.len() as f64,
            V::NativeFunc(..) => todo!(),
        })),
        NativeFunction::Parse => match &resolved_params[0] {
            V::String(s) => match s.parse() {
                Ok(v) => Ok(V::Number(v)),
                Err(_) => Ok(V::Null),
            },
            _ => panic!("not a valid arg"),
        },
        NativeFunction::Typeof => match &resolved_params[0] {
            V::String(_) => Ok(V::String("<str>".to_string())),
            V::Number(_) => Ok(V::String("<number>".to_string())),
            V::Bool(_) => Ok(V::String("<bool>".to_string())),
            V::Obj(_) => Ok(V::String("<object>".to_string())),
            V::Func(_, _, _) => Ok(V::String("<function>".to_string())),
            V::NativeFunc(_, _) => Ok(V::String("<function>".to_string())),
            V::Null => Ok(V::String("<null>".to_string())),
            V::List(_) => Ok(V::String("<list>".to_string())),
        },
        NativeFunction::Random => match &resolved_params[0] {
            V::List(list) => {
                let micros = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time travel is not allowed")
                    .subsec_micros() as usize;
                Ok(list[micros % list.len()].clone())
            }
            _ => panic!("expected list"),
        },
    }
}

fn resolve_get(expr: Box<Expr>, vars: &mut Variables) -> Result<&mut V> {
    match expr.expr.clone() {
        ExprKind::Get(s, expr) => Ok(resolve_get(expr, vars)?.as_mut_obj().get_mut(&s).unwrap()),
        // TODO: Err if not found
        ExprKind::Variable(s) => vars.get_mut(&s),
        _ => panic!("must be Get"),
    }
}

#[derive(Debug, Clone)]
enum V {
    String(String),
    Number(f64),
    Bool(bool),
    Obj(HashMap<String, V>),
    Func(Vec<String>, Vec<Statement>, Option<HashMap<String, V>>),
    NativeFunc(usize, NativeFunction),
    Null,
    List(Vec<V>),
}

#[derive(Debug, Clone)]
enum NativeFunction {
    Print,
    ReadFile,
    ReadInput,
    Len,
    Split,
    SplitLines,
    Append,
    Parse,
    Typeof,
    Random,
}

impl Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            V::String(s) => write!(f, "{}", s),
            V::Number(n) => write!(f, "{}", n),
            V::Bool(b) => write!(f, "{}", b),
            V::Obj(o) => write!(
                f,
                "{{{}}}",
                o.iter()
                    .map(|kvp| format!("{}: {}", kvp.0, kvp.1))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            V::Func(..) => write!(f, "<function>"),
            V::Null => write!(f, "null"),
            V::List(items) => {
                write!(
                    f,
                    "[{}]",
                    items
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            V::NativeFunc(_, name) => write!(f, "<native fn {:?}>", name),
        }
    }
}

impl V {
    fn as_mut_list(&mut self) -> &mut Vec<V> {
        match self {
            V::List(l) => l,
            _ => panic!("not a list."),
        }
    }

    fn as_list(self) -> Vec<V> {
        match self {
            V::List(l) => l,
            _ => panic!("not a list."),
        }
    }

    fn as_mut_obj(&mut self) -> &mut HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            _ => panic!("not an object."),
        }
    }
    fn _as_obj(self) -> HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            _ => panic!("not an object."),
        }
    }
    fn as_num(self) -> f64 {
        match self {
            V::Number(n) => n,
            _ => panic!("not a number."),
        }
    }
    fn as_bool(self) -> bool {
        match self {
            V::Bool(b) => b,
            _ => panic!("not a bool."),
        }
    }

    fn as_string(self) -> String {
        match self {
            V::String(s) => s,
            _ => panic!("not a string."),
        }
    }
}
