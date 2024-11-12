use crate::lsp::logger;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::log;
use crate::parser::{Declaration, Expr, Statement};
use crate::TokenKind;

#[derive(Debug)]
struct Variables {
    envs: Vec<HashMap<String, Kind>>,
}

#[derive(Debug, Clone)]
enum Kind {
    Function(usize),
    String(String),
    Number,
    Bool,
    Null,
    Obj,
    List,
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
            _ => self.envs.last().unwrap().get(s).cloned(),
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
    // println!("decls {:?}", declarations);

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
                // todo!( params )
                self.vars.add(&name, Kind::Function(params.len()));
                for b in body {
                    // b.visit(vars)?
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

        match expr {
            Expr::Prefix(_, e) => Ok(self.expr(e)?),
            // TODO: How to handle this?
            Expr::Operator(e1, kind, e2) => {
                match kind {
                    TokenKind::Equal => {
                        // let expr1 = self.expr(e1)?;
                        let eval = self.expr(e1)?;
                        match eval {
                            Kind::String(s) => {
                                // println!("variable name: {}", s);
                                if let None = self.vars.get(&s) {
                                    return Err(Error {
                                        line: 1,
                                        start: 1,
                                        end: 5,
                                        msg: format!(
                                            "No variable with the name {} in current scope",
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
                Ok(self.expr(e1)?)
            }
            Expr::Number(_) => Ok(Kind::Number),
            Expr::String(s) => Ok(Kind::String(s.to_string())),
            Expr::Bool(_) => Ok(Kind::Bool),
            Expr::Null => Ok(Kind::Null),
            Expr::Function(params, _body) => Ok(Kind::Function(params.len())),
            Expr::Get(s, e) => todo!("dunno how to handle?"),
            Expr::Set(_, _, _) => Ok(Kind::Null),
            Expr::Call(params, expr, pos) => {
                let kind = self.expr(expr)?;
                match kind {
                    Kind::Function(param_count) => {
                        if param_count != params.len() {
                            return Err(Error {
                                line: pos.0,
                                start: pos.1,
                                end: pos.2,
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
            Expr::Variable(name) => {
                if let Some(x) = self.vars.get(&name) {
                    Ok(x)
                } else {
                    Err(Error {
                        line: 0,
                        start: 0,
                        end: 5,
                        msg: format!("No variable with the name {} in current scope", name),
                    })
                }
            }
            Expr::Object(_) => Ok(Kind::Obj),
            Expr::List(_) => Ok(Kind::List),
            // TODO: How to handle?
            Expr::Index(_, _) => Ok(Kind::Null),
            Expr::SetList(_, _, _) => Ok(Kind::Null),
        }
    }
}
