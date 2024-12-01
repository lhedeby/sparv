use crate::lsp::logger;
use crate::token::Token;
use std::collections::HashMap;
use std::fmt::Display;
use std::mem::discriminant;

use crate::error::{Error, Result};
use crate::log;
use crate::parser::{Declaration, Expr, ExprKind, Statement};
use crate::TokenKind;

#[derive(Debug)]
struct Variables {
    envs: Vec<HashMap<String, Kind>>,
}

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Function(usize),
    String(String),
    Number,
    Bool,
    Null,
    Obj,
    List,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::Function(_) => "Fun",
                Kind::String(_) => "String",
                Kind::Number => "Number",
                Kind::Bool => "Bool",
                Kind::Null => "Null",
                Kind::Obj => "Object",
                Kind::List => "List",
            }
        )
    }
}

struct Analyzer {
    vars: Variables,
}

impl Variables {
    fn add(&mut self, s: &str, k: Kind) {
        self.envs.last_mut().unwrap().insert(s.to_string(), k);
    }
    fn get(&mut self, s: &str) -> Option<Kind> {
        match s {
            // native functions with 1 param
            "print" | "read_file" | "read_input" | "split_lines" | "len" | "parse" | "typeof"
            | "random" => Some(Kind::Function(1)),
            _ => {
                // self.envs.last().unwrap().get(s).cloned()
                for v in self.envs.iter().rev() {
                    if let Some(res) = v.get(s) {
                        return Some(res.clone());
                    }
                }
                None
            },
        }
    }
}

pub fn run(declarations: &Vec<Declaration>) -> Result<()> {
    // println!("running analysis");

    log!("running analyzer");
    let mut analyzer = Analyzer {
        vars: Variables {
            envs: vec![HashMap::new()],
        },
    };

    for decl in declarations {
        analyzer.decl(decl)?
        // decl.visit(&mut vars);
        // println!("decl: {:?}", decl);
    }
    Ok(())
}

impl Analyzer {
    fn decl(&mut self, decl: &Declaration) -> Result<()> {
        match decl {
            Declaration::Function(name, params, body) => {
                self.vars.add(&name, Kind::Function(params.len()));
                for stmt in body {
                    self.stmt(stmt)?;
                }
            }
            Declaration::Statement(s) => self.stmt(s)?,
            Declaration::Import(_) => todo!("Should maybe be resolved to the same tree in parser?"),
        }
        Ok(())
    }

    fn stmt(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Expression(expr) => {
                self.expr(expr)?;
            }
            Statement::Let(name, expr) => {
                let e = self.expr(expr)?;
                if let Some(_) = self.vars.get(&name) {
                    // println!("expr {:?}", expr);
                    // println!("LINE {}", expr.start_line);
                    return Err(Error {
                        line: expr.start_line,
                        start: 0,
                        end: expr.end_col,
                        msg: format!("Variable '{}' is already defined", &name)
                    })
                }
                self.vars.add(&name, e);
            }
            Statement::For(iter_name, iter, body) => {}
            Statement::If(condition, if_true_body, if_else_body) => {}
            Statement::Return(expr) => {}
            Statement::While(condition, body) => {}
            Statement::Assignment(..) => todo!("remove maybe"),
        };
        Ok(())
    }

    // return error if found
    fn expr(&mut self, expr: &Expr) -> Result<Kind> {
        // println!("lyze expr {:?}", expr);

        match &expr.expr {
            ExprKind::Prefix(_, e) => Ok(self.expr(&e)?),
            // TODO: How to handle this?
            ExprKind::Operator(e1, kind, e2) => {
                match kind {
                    TokenKind::Equal => {
                        // let expr1 = self.expr(e1)?;
                        let identifier = self.expr(&e1)?;
                        let k2 = self.expr(&e2)?;
                        match identifier {
                            Kind::String(s) => {
                                // println!("variable name: {}", s);
                                // TODO: Should we have type saftey?
                                if let Some(k1) = self.vars.get(&s) {
                                    if discriminant(&k1) != discriminant(&k2) {
                                        return Err(Error {
                                            line: e1.start_line,
                                            start: e1.start_col,
                                            end: e2.end_col,
                                            msg: format!(
                                                "'{}' is of type '{}' not '{}'",
                                                s, k1, k2
                                            ),
                                        });
                                    }
                                } else {
                                    println!("vars: {:?}", self.vars);
                                    return Err(Error {
                                        line: e1.start_line,
                                        start: e1.start_col,
                                        end: e1.end_col,
                                        msg: format!(
                                            "No variable with the name '{}' in current scope",
                                            s
                                        ),
                                    });
                                }
                            }
                            _ => panic!("lhs must be string?"),
                        }
                        // println!("looking for var {:?}", eval);
                    }
                    _ => {}
                }
                Ok(self.expr(&e1)?)
            }
            ExprKind::Number(_) => Ok(Kind::Number),
            ExprKind::String(s) => Ok(Kind::String(s.to_string())),
            ExprKind::Bool(_) => Ok(Kind::Bool),
            ExprKind::Null => Ok(Kind::Null),
            ExprKind::Function(params, _body) => Ok(Kind::Function(params.len())),
            ExprKind::Get(s, e) => Ok(Kind::Null), // TODO: Handle correctly
            ExprKind::Set(_, _, _) => Ok(Kind::Null),
            ExprKind::Call(params, e) => {
                let kind = self.expr(&e)?;
                match kind {
                    Kind::Function(param_count) => {
                        if param_count != params.len() {
                            return Err(Error {
                                line: expr.start_line,
                                start: expr.start_col,
                                end: expr.end_col,
                                msg: format!(
                                    "call has {} params but the function takes {}",
                                    params.len(),
                                    param_count
                                ),
                            });
                        }
                        Ok(kind)
                    }
                    _ => panic!("not a function"),
                }
            }
            ExprKind::Variable(name) => {
                if let Some(x) = self.vars.get(&name) {
                    Ok(x)
                } else {
                    println!("vars: {:?}", self.vars);
                    Err(Error {
                        line: expr.start_line,
                        start: expr.start_col,
                        end: expr.end_col,
                        msg: format!("No variable with the name '{}' in current scope", name),
                    })
                }
            }
            ExprKind::Object(_) => Ok(Kind::Obj),
            ExprKind::List(_) => Ok(Kind::List),
            // TODO: How to handle?
            ExprKind::Index(_, _) => Ok(Kind::Null),
            ExprKind::SetList(_, _, _) => Ok(Kind::Null),
        }
    }
}
