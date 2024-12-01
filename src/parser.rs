use std::fs;

use crate::{
    analysis,
    error::{Error, Result},
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub struct Parser<'a> {
    p: usize,
    tokens: Vec<Token>,
    root: &'a str,
}

impl Parser<'_> {
    pub fn parse<'a>(source: &'a str, root: &'a str) -> Result<Vec<Declaration>> {
        let mut tokens = Scanner::get_tokens(source)?;
        tokens.retain(|token| token.kind != TokenKind::Comment);
        let mut parser = Parser { p: 0, tokens, root };
        let mut decls: Vec<Declaration> = vec![];
        while parser.tokens[parser.p].kind != TokenKind::Eof {
            let decl = parser.parse_decl()?;
            match decl {
                Declaration::Import(imported_decls) => decls.extend(imported_decls),
                rest => decls.push(rest),
            }
        }
        // hoist functions
        decls.sort_by(|a, b| match (a, b) {
            (Declaration::Function(..), Declaration::Function(..)) => std::cmp::Ordering::Equal,
            (Declaration::Function(..), _) => std::cmp::Ordering::Less,
            (..) => std::cmp::Ordering::Greater,
        });

        analysis::run(&decls)?;
        // println!("=== DECLARATIONS ===");
        // for d in &decls {
        //     println!("{:?}", d);
        // }
        // println!("\n");

        Ok(decls)
    }

    /*
     *
     *   === DECLARATIONS ===
     *
     */

    fn parse_decl(&mut self) -> Result<Declaration> {
        match self.get_kind() {
            TokenKind::Fun => self.fun_decl(),
            TokenKind::Import => self.import(),
            _ => Ok(Declaration::Statement(self.parse_stmt()?)),
        }
    }

    fn fun_decl(&mut self) -> Result<Declaration> {
        self.p += 1;
        let fun_identifier = self.consume_identifier()?;
        self.consume(TokenKind::LeftParen)?;
        let mut params: Vec<String> = vec![];
        let mut stmts: Vec<Statement> = vec![];
        // params
        loop {
            match self.get_kind() {
                TokenKind::Comma => self.p += 1,
                TokenKind::Identifier(param_identifier) => {
                    params.push(param_identifier);
                    self.p += 1;
                }
                TokenKind::RightParen => break,
                _ => {
                    let t = self.get_token();
                    return Err(Error {
                        line: t.line,
                        start: t.start,
                        end: t.end,
                        msg: format!("Unexpected token"),
                    });
                }
            }
        }
        self.consume(TokenKind::RightParen)?;
        match self.get_kind() {
            TokenKind::LeftBrace => {
                self.consume(TokenKind::LeftBrace)?;
                while self.get_kind() != TokenKind::RightBrace {
                    stmts.push(self.parse_stmt()?);
                }
                self.consume(TokenKind::RightBrace)?;
            }
            _ => {
                stmts.push(Statement::Return(self.parse_expr(0)?));
                self.consume(TokenKind::Semicolon)?;
            }
        }

        Ok(Declaration::Function(fun_identifier, params, stmts))
    }

    fn import(&mut self) -> Result<Declaration> {
        self.p += 1;
        let kind = self.tokens[self.p].kind.clone();
        let path = match kind {
            TokenKind::String(s) => s,
            k => {
                return Err(self.err(format!("Could not import file: '{:?}'", k)));
            }
        };

        if path.split('.').last().unwrap() != "sparv" {
            return Err(self.err(format!("Could not import file: '{}'", path)));
        }
        let root_path = std::path::Path::new(self.root);
        let new_path = root_path
            .parent()
            .expect("Should fail before if the path is not valid")
            .to_str()
            .unwrap_or("")
            .to_string()
            + &path;
        match fs::read_to_string(new_path.clone()) {
            Ok(source) => {
                let tree = Parser::parse(&source, &new_path)?;
                Ok(Declaration::Import(tree))
            }
            Err(_) => Err(self.err(format!("Could not import file: '{}'", path))),
        }
    }

    /*
     *
     *   === STATEMENTS ===
     *
     */

    fn parse_stmt(&mut self) -> Result<Statement> {
        match self.get_kind() {
            TokenKind::While => self.while_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Identifier(_) => self.identifier_stmt(),
            TokenKind::For => self.for_stmt(),
            // TODO: Think about if this is actually correct
            _ => self.expression_stmt(),
        }
    }

    fn expression_stmt(&mut self) -> Result<Statement> {
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Statement::Expression(expr))
    }

    fn let_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let identifier = self.consume_identifier()?;
        self.consume(TokenKind::Equal)?;

        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Statement::Let(identifier, expr))
    }

    fn while_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let mut stmts: Vec<Statement> = vec![];
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::LeftBrace)?;

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        self.consume(TokenKind::RightBrace)?;
        Ok(Statement::While(expr, stmts))
    }

    fn if_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let mut if_stmts: Vec<Statement> = vec![];
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::LeftBrace)?;

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt()?;
            if_stmts.push(stmt);
        }
        self.consume(TokenKind::RightBrace)?;

        if self.get_kind() != TokenKind::Else {
            return Ok(Statement::If(expr, if_stmts, vec![]));
        }
        self.p += 1;

        let mut else_stmts: Vec<Statement> = vec![];
        self.consume(TokenKind::LeftBrace)?;

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt()?;
            else_stmts.push(stmt);
        }
        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::If(expr, if_stmts, else_stmts))
    }
    fn return_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let expr = if self.tokens[self.p].kind != TokenKind::Semicolon {
            self.parse_expr(0)?
        } else {
            Expr {
                expr: ExprKind::Null,
                start_line: 1,
                start_col: 0,
                end_line: 1,
                end_col: 0,
            }
        };
        self.consume(TokenKind::Semicolon)?;
        Ok(Statement::Return(expr))
    }
    fn identifier_stmt(&mut self) -> Result<Statement> {
        let res = Ok(Statement::Expression(self.parse_expr(0)?));
        self.consume(TokenKind::Semicolon)?;
        res
    }
    fn for_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let mut stmts = vec![];
        let i = self.consume_identifier()?;
        self.consume(TokenKind::In)?;
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::LeftBrace)?;

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        self.consume(TokenKind::RightBrace)?;

        Ok(Statement::For(i, expr, stmts))
    }

    /*
     *
     *   === EXPRESSIONS ===
     *
     */

    fn parse_expr(&mut self, precedence: usize) -> Result<Expr> {
        let kind = &self.tokens[self.p].kind.clone();
        self.p += 1;

        let mut left = self
            .parse_prefix(kind, self.tokens[self.p - 1].clone())?
            .clone();

        while precedence < infix_precedence(&self.tokens[self.p].kind) {
            let token_kind = self.tokens[self.p].kind.clone();
            self.p += 1;
            left = self.parse_infix(&left, token_kind)?;
        }
        Ok(left)
    }

    fn parse_prefix(&mut self, token_kind: &TokenKind, token: Token) -> Result<Expr> {
        match token_kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Bang => {
                let right = self.parse_expr(prefix_precedence(&token_kind))?;
                Ok(Expr {
                    expr: ExprKind::Prefix(token_kind.clone(), Box::new(right.clone())),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                })
            }
            TokenKind::LeftBracket => {
                let mut items = vec![];
                loop {
                    match self.get_kind() {
                        TokenKind::Comma => self.p += 1,
                        TokenKind::RightBracket => break,
                        _ => items.push(self.parse_expr(0)?),
                    }
                }
                self.consume(TokenKind::RightBracket)?;
                Ok(Expr {
                    expr: ExprKind::List(items),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                })
            }
            TokenKind::Identifier(identifier) => {
                let mut expr = ExprKind::Variable(identifier.to_string());
                let expr_line = token.line;
                let expr_start = token.start;
                loop {
                    match self.get_kind() {
                        TokenKind::Dot => {
                            self.p += 1;
                            let i = self.consume_identifier()?;
                            expr = ExprKind::Get(
                                i,
                                Box::new(Expr {
                                    expr,
                                    start_line: 1,
                                    start_col: 0,
                                    end_line: 1,
                                    end_col: 0,
                                }),
                            )
                        }
                        TokenKind::LeftParen => {
                            let line = self.get_token().line;
                            let start = self.get_token().start;
                            self.p += 1;
                            let mut params = vec![];

                            loop {
                                match self.get_kind() {
                                    TokenKind::Comma => self.p += 1,
                                    TokenKind::RightParen => break,
                                    _ => params.push(self.parse_expr(0)?),
                                }
                            }

                            let end = self.get_token().end;

                            self.consume(TokenKind::RightParen)?;
                            expr = ExprKind::Call(
                                params,
                                Box::new(Expr {
                                    expr,
                                    start_line: line,
                                    start_col: start,
                                    end_line: line,
                                    end_col: end,
                                }),
                            )
                        }
                        TokenKind::LeftBracket => {
                            self.p += 1;
                            expr = ExprKind::Index(
                                Box::new(Expr {
                                    expr,
                                    start_line: 1,
                                    start_col: 0,
                                    end_line: 1,
                                    end_col: 0,
                                }),
                                Box::new(self.parse_expr(0)?),
                            );
                            self.consume(TokenKind::RightBracket)?;
                        }
                        _ => break,
                    }
                }
                Ok(Expr {
                    expr,
                    start_line: expr_line,
                    start_col: expr_start,
                    end_line: expr_line,
                    end_col: self.tokens[self.p-1].end,
                })
            }
            TokenKind::Number(f) => {
                // println!("token {:?}", token);
                Ok(Expr {
                            expr: ExprKind::Number(*f),
                            start_line: token.line,
                            start_col: token.start,
                            end_line: token.line,
                            end_col: token.end,
                        })
            },
            TokenKind::String(s) => Ok(Expr {
                expr: ExprKind::String(s.to_string()),
                start_line: token.line,
                start_col: token.start,
                end_line: token.line,
                end_col: token.end,
            }),
            TokenKind::True => Ok(Expr {
                expr: ExprKind::Bool(true),
                start_line: token.line,
                start_col: token.start,
                end_line: token.line,
                end_col: token.end,
            }),
            TokenKind::False => Ok(Expr {
                expr: ExprKind::Bool(false),
                start_line: token.line,
                start_col: token.start,
                end_line: token.line,
                end_col: token.end,
            }),
            TokenKind::Null => Ok(Expr {
                expr: ExprKind::Null,
                start_line: token.line,
                start_col: token.start,
                end_line: token.line,
                end_col: token.end,
            }),
            TokenKind::Fun => {
                self.consume(TokenKind::LeftParen)?;
                let mut params: Vec<String> = vec![];
                let mut stmts: Vec<Statement> = vec![];
                // params
                loop {
                    match self.get_kind() {
                        TokenKind::Comma => self.p += 1,
                        TokenKind::Identifier(param_identifier) => {
                            params.push(param_identifier);
                            self.p += 1;
                        }
                        TokenKind::RightParen => break,
                        actual => return Err(self.err(format!("Unexpected token '{:?}'", actual))),
                    }
                }
                self.consume(TokenKind::RightParen)?;
                match self.get_kind() {
                    TokenKind::LeftBrace => {
                        self.consume(TokenKind::LeftBrace)?;
                        while self.get_kind() != TokenKind::RightBrace {
                            stmts.push(self.parse_stmt()?);
                        }
                        self.consume(TokenKind::RightBrace)?;
                    }
                    _ => {
                        stmts.push(Statement::Return(self.parse_expr(0)?));
                    }
                }

                Ok(Expr {
                    expr: ExprKind::Function(params, stmts),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                })
            }
            TokenKind::LeftParen => {
                let expr = self.parse_expr(0);
                self.consume(TokenKind::RightParen)?;
                expr
            }
            TokenKind::LeftBrace => {
                let mut res = vec![];
                while self.get_kind() != TokenKind::RightBrace {
                    let expr = self.parse_expr(0);
                    res.push(expr?);
                    self.consume(TokenKind::Comma)?;
                }
                self.consume(TokenKind::RightBrace)?;
                Ok(Expr {
                    expr: ExprKind::Object(res),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                })
            }
            actual => Err(Error {
                line: self.tokens[self.p - 1].line,
                start: self.tokens[self.p - 1].start,
                end: self.tokens[self.p - 1].end,
                msg: format!("Unexpected token '{:?}'", actual),
            }),
        }
    }

    fn parse_infix(
        &mut self,
        expr: &Expr,
        token_kind: TokenKind,
    ) -> Result<Expr> {
        let right = self.parse_expr(infix_precedence(&token_kind))?;

        match token_kind {
            TokenKind::Equal | TokenKind::PlusEqual | TokenKind::MinusEqual => match &expr.expr {
                ExprKind::Variable(s) => Ok(Expr {
                    expr: ExprKind::Operator(
                        Box::new(Expr {
                            expr: ExprKind::String(s.to_string()),
                            start_line: expr.start_line,
                            start_col: expr.start_col,
                            end_line: expr.end_line,
                            end_col: expr.end_col,
                        }),
                        token_kind,
                        Box::new(right),
                    ),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                }),
                ExprKind::Get(s, g_expr) => Ok(Expr {
                    expr: ExprKind::Set(s.to_string(), g_expr.clone(), Box::new(right)),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                }),
                ExprKind::Index(e1, e2) => Ok(Expr {
                    expr: ExprKind::SetList(e1.clone(), e2.clone(), Box::new(right)),
                    start_line: 1,
                    start_col: 0,
                    end_line: 1,
                    end_col: 0,
                }),
                _ => Err(self.err("Invalid assignment".to_string())),
            },
            _ => Ok(Expr {
                expr: ExprKind::Operator(Box::new(expr.clone()), token_kind, Box::new(right)),
                start_line: 1,
                start_col: 0,
                end_line: 1,
                end_col: 0,
            }),
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        self.p += 1;
        match &self.tokens[self.p - 1].kind {
            TokenKind::Identifier(s) => Ok(s.to_string()),
            actual => Err(self.err(format!(
                "Unexpected token '{:?}', expected 'identifier'",
                actual
            ))),
        }
    }

    fn consume(&mut self, expected_token: TokenKind) -> Result<()> {
        let actual = self.tokens[self.p].kind.clone();
        if expected_token != actual {
            return Err(self.err(format!(
                "Unexpected token '{:?}', expected '{:?}'",
                actual, expected_token
            )));
        }
        self.p += 1;
        Ok(())
    }

    fn get_token(&mut self) -> &Token {
        &self.tokens[self.p]
    }

    fn get_kind(&mut self) -> TokenKind {
        self.tokens[self.p].kind.clone()
    }

    fn err(&mut self, msg: String) -> Error {
        let token = self.get_token();
        Error {
            line: token.line,
            start: token.start,
            end: token.end,
            msg: msg.to_string(),
        }
    }
}
// PRECEDENCE
// ASSIGNMENT, =
// or
// and
// EQUALITY, == !=
// COMPARISON, < > <= >=
// TERM, + -
// FACTOR, * /
// UNARY, ! -
// CALL, . ()
// PRIMARY

fn infix_precedence(kind: &TokenKind) -> usize {
    match kind {
        TokenKind::Equal | TokenKind::PlusEqual | TokenKind::MinusEqual => 1,
        TokenKind::Arrow => 2,
        TokenKind::Or => 3,
        TokenKind::And => 4,
        TokenKind::BangEqual | TokenKind::EqualEqual => 5,
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => 6,
        TokenKind::Plus | TokenKind::Minus => 7,
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 8,
        TokenKind::Dot | TokenKind::Colon => 10,
        _ => 0,
    }
}
fn prefix_precedence(_kind: &TokenKind) -> usize {
    9
}

#[derive(Debug)]
pub enum Declaration {
    Function(String, Vec<String>, Vec<Statement>),
    Statement(Statement),
    Import(Vec<Declaration>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expr),
    Let(String, Expr),
    For(String, Expr, Vec<Statement>),
    If(Expr, Vec<Statement>, Vec<Statement>),
    Return(Expr),
    While(Expr, Vec<Statement>),
    Assignment(String, Vec<String>, Expr),
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub expr: ExprKind,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Prefix(TokenKind, Box<Expr>),
    Operator(Box<Expr>, TokenKind, Box<Expr>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Function(Vec<String>, Vec<Statement>),
    Get(String, Box<Expr>),
    Set(String, Box<Expr>, Box<Expr>),
    Call(
        Vec<Expr>,
        Box<Expr>,
    ),
    Variable(String),
    Object(Vec<Expr>),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    SetList(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[cfg(test)]
mod tests {}
