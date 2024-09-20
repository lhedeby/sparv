use std::collections::HashMap;

use crate::parser::{Declaration, Expr, Statement};
use crate::scanner::TokenKind as TK;

pub struct Interpreter {
    variables: HashMap<String, V>,
}

impl Interpreter {
    pub fn interpret(declarations: Vec<Declaration>) {
        let mut interpreter = Interpreter {
            variables: HashMap::new(),
        };
        for decl in declarations {
            println!("parsing: {:?}", decl);
            decl.interpret(&mut interpreter);
        }
    }
}

impl Declaration {
    pub fn interpret(&self, interpreter: &mut Interpreter) {
        match self {
            Declaration::Function(name, params, stmts) => {
                println!("DECLARING FUNCTION: {name}, params: {:?}", params);
                interpreter
                    .variables
                    .insert(name.to_string(), V::Func(params.to_vec(), stmts.to_vec()));
            }
            Declaration::Statement(stmt) => stmt.interpret(interpreter),
        }
    }
}

impl Statement {
    pub fn interpret(&self, interpreter: &mut Interpreter) {
        match self {
            Statement::Expression(expr) => {
                _ = expr.interpret(interpreter);
            }
            Statement::Let(name, expr) => {
                let expr = expr.interpret(interpreter);
                interpreter.variables.insert(name.to_string(), expr);
            }
            Statement::For => todo!(),
            Statement::If(expr, if_stmts, else_stmts) => {
                if expr.interpret(interpreter).as_bool() {
                    for stmt in if_stmts {
                        stmt.interpret(interpreter)
                    }
                } else {
                    for stmt in else_stmts {
                        stmt.interpret(interpreter)
                    }
                }
            }
            Statement::Print(expr) => {
                let value = expr.interpret(interpreter);
                match value {
                    V::String(s) => println!("{s}"),
                    V::Number(n) => println!("{n}"),
                    V::Bool(b) => println!("{b}"),
                    V::Obj(o) => println!("{:?}", o),
                    V::Func(..) => todo!("print function"),
                    V::Nil => println!("NIL"),
                }
            }
            Statement::Return(_) => todo!(),
            Statement::While(expr, stmts) => {
                while expr.interpret(interpreter).as_bool() {
                    for stmt in stmts {
                        stmt.interpret(interpreter);
                    }
                }
            }
            Statement::Assignment(..) => todo!("maybe remove"),
        }
    }
}

impl Expr {
    pub fn interpret(&self, interpreter: &mut Interpreter) -> V {
        match self {
            Expr::Prefix(token_kind, expr) => match token_kind {
                TK::Minus => V::Number(-expr.interpret(interpreter).as_num()),
                TK::Bang => V::Bool(!expr.interpret(interpreter).as_bool()),
                _ => panic!("not valid prefixtoken '{:?}'", token_kind),
            },
            Expr::Operator(lhs, token_kind, rhs) => {
                let expr1 = lhs.interpret(interpreter);
                let expr2 = rhs.interpret(interpreter);
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
                        interpreter.variables.insert(s, v);
                        V::Number(0.0)
                    }
                    (a1, a2, a3) => panic!("what is this got: '{:?}', '{:?}', '{:?}'", a1, a2, a3),
                }
            }
            Expr::Number(n) => V::Number(*n),
            Expr::String(s) => V::String(s.to_string()),
            Expr::Bool(b) => V::Bool(*b),
            Expr::Variable(s) => interpreter.variables.get(s).unwrap().clone(),
            Expr::Object(fields) => {
                let mut map = HashMap::new();
                for f in fields {
                    match f {
                        Expr::Operator(identifier, op, expr) => {
                            assert_eq!(op, &TK::Equal);
                            let test = identifier.interpret(interpreter).as_string();
                            map.insert(test, expr.interpret(interpreter));
                        }
                        _ => panic!("must be operator."),
                    }
                }
                V::Obj(map)
            }
            Expr::Get(s, expr) => expr
                .interpret(interpreter)
                .as_mut_obj()
                .get_mut(s)
                .unwrap()
                .clone(),
            Expr::Set(s, g_expr, expr) => {
                let resolved = expr.interpret(interpreter);
                resolve_get(g_expr.clone(), interpreter)
                    .as_mut_obj()
                    .insert(s.to_string(), resolved);
                V::Bool(true)
            }
            Expr::Call(params, expr) => {
                println!("call!");
                println!("expr {:?}", expr);
                println!("{:?}", interpreter.variables);
                println!("params {:?}", params);
                match expr.interpret(interpreter) {
                    V::Func(param_names, stmts) => {
                        if param_names.len() != params.len() {
                            panic!("wrong amount of parameters")
                        }
                        for (name, param_expr) in std::iter::zip(param_names, params) {
                            let res = param_expr.interpret(interpreter);
                            interpreter.variables.insert(name, res);
                        }
                        for stmt in stmts {
                            stmt.interpret(interpreter)
                        }
                    }
                    _ => panic!("Expected function"),
                }
                V::Nil
            }
        }
    }
}

fn resolve_get(expr: Box<Expr>, interpreter: &mut Interpreter) -> &mut V {
    println!("!resolve_get! {:?}", expr);
    match *expr.clone() {
        Expr::Get(s, expr) => resolve_get(expr, interpreter)
            .as_mut_obj()
            .get_mut(&s)
            .unwrap(),
        Expr::Variable(s) => interpreter.variables.get_mut(&s).unwrap(),
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
    Nil,
}

impl V {
    fn as_mut_obj(&mut self) -> &mut HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            V::Nil | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }
    fn as_obj(self) -> HashMap<String, V> {
        match self {
            V::Obj(o) => o,
            V::Nil | V::String(_) | V::Bool(_) | V::Number(_) | V::Func(..) => {
                panic!("not a object.")
            }
        }
    }
    fn as_num(self) -> f64 {
        match self {
            V::Number(n) => n,
            V::Nil | V::String(_) | V::Bool(_) | V::Obj(_) | V::Func(..) => panic!("not a number."),
        }
    }
    fn as_bool(self) -> bool {
        match self {
            V::Bool(b) => b,
            V::Nil | V::String(_) | V::Number(_) | V::Obj(_) | V::Func(..) => panic!("not a bool."),
        }
    }

    fn as_string(self) -> String {
        match self {
            V::String(s) => s,
            V::Nil | V::Bool(_) | V::Number(_) | V::Obj(_) | V::Func(..) => panic!("not a string."),
        }
    }
}
