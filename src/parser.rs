use std::fs;

use crate::{
    error::{Error, ErrorKind},
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub struct Parser {
    p: usize,
    tokens: Vec<Token>,
}

type Result<T> = std::result::Result<T, Error>;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Declaration>> {
        let mut parser = Parser { p: 0, tokens };
        let mut decls: Vec<Declaration> = vec![];
        while parser.tokens[parser.p].kind != TokenKind::Eof {
            let decl = parser.parse_decl()?;
            match decl {
                Declaration::Import(imported_decls) => decls.extend(imported_decls),
                rest => decls.push(rest),
            }
        }
        // hoist functions
        decls.sort_by(|a, _| match a {
            Declaration::Function(..) => std::cmp::Ordering::Less,
            Declaration::Statement(_) => std::cmp::Ordering::Greater,
            Declaration::Import(_) => {
                unreachable!("All import declarations are resolved by this point")
            }
        });
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
        self.consume(TokenKind::Identifier)?;
        let fun_identifier = self.tokens[self.p - 1].value.to_string();
        self.consume(TokenKind::LeftParen)?;
        let mut params: Vec<String> = vec![];
        let mut stmts: Vec<Statement> = vec![];
        // params
        loop {
            match self.get_kind() {
                TokenKind::Comma => self.p += 1,
                TokenKind::Identifier => {
                    let param_identifier = self.tokens[self.p].value.to_string();
                    params.push(param_identifier);
                    self.p += 1;
                }
                TokenKind::RightParen => break,
                _ => {
                    return Err(Error {
                        line: self.get_token().line,
                        kind: ErrorKind::UnexpectedToken(None, self.get_kind()),
                        cols: self.get_cols(),
                    })
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
                stmts.push(self.parse_stmt()?);
            }
        }

        Ok(Declaration::Function(fun_identifier, params, stmts))
    }

    fn import(&mut self) -> Result<Declaration> {
        self.p += 1;
        let path = self.tokens[self.p].value.to_string();
        match fs::read_to_string(path.to_string()) {
            Ok(source) => {
                let tokens = Scanner::get_tokens(source)?;
                let tree = Parser::parse(tokens)?;
                Ok(Declaration::Import(tree))
            }
            Err(_) => Err(Error {
                line: self.get_token().line,
                kind: ErrorKind::Import(path),
                cols: self.get_cols(),
            }),
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
            TokenKind::Identifier => self.identifier_stmt(),
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
        self.consume(TokenKind::Identifier)?;
        let identifier = self.tokens[self.p - 1].value.to_string();
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
            Expr::Null
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
        let i = self.tokens[self.p].value.to_string();
        self.consume(TokenKind::Identifier)?;
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

        let mut left = self.parse_prefix(kind)?.clone();

        while precedence < infix_precedence(&self.tokens[self.p].kind) {
            let token_kind = self.tokens[self.p].kind;
            self.p += 1;
            left = self.parse_infix(&left, token_kind)?;
        }
        Ok(left)
    }

    // this is the parselets?
    fn parse_prefix(&mut self, token_kind: &TokenKind) -> Result<Expr> {
        match token_kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Bang => {
                let right = self.parse_expr(prefix_precedence(&token_kind))?;
                Ok(Expr::Prefix(token_kind.clone(), Box::new(right.clone())))
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
                Ok(Expr::List(items))
            }
            TokenKind::Identifier => {
                let identifier = self.tokens[self.p - 1].value.clone();
                let mut expr = Expr::Variable(identifier);
                loop {
                    match self.get_kind() {
                        TokenKind::Dot => {
                            self.p += 1;
                            let i = self.tokens[self.p].value.clone();
                            self.consume(TokenKind::Identifier)?;
                            expr = Expr::Get(i, Box::new(expr))
                        }
                        TokenKind::LeftParen => {
                            self.p += 1;
                            let mut params = vec![];

                            loop {
                                match self.get_kind() {
                                    TokenKind::Comma => self.p += 1,
                                    TokenKind::RightParen => break,
                                    _ => params.push(self.parse_expr(0)?),
                                }
                            }

                            self.consume(TokenKind::RightParen)?;
                            expr = Expr::Call(params, Box::new(expr))
                        }
                        TokenKind::LeftBracket => {
                            self.p += 1;
                            expr = Expr::Index(Box::new(expr), Box::new(self.parse_expr(0)?));
                            self.consume(TokenKind::RightBracket)?;
                        }
                        _ => break,
                    }
                }
                Ok(expr)
            }
            TokenKind::Number => Ok(Expr::Number(self.tokens[self.p - 1].value.parse().unwrap())),
            TokenKind::String => Ok(Expr::String(self.tokens[self.p - 1].value.clone())),
            TokenKind::True => Ok(Expr::Bool(true)),
            TokenKind::False => Ok(Expr::Bool(false)),
            TokenKind::Null => Ok(Expr::Null),
            TokenKind::Fun => {
                self.consume(TokenKind::LeftParen)?;
                let mut params: Vec<String> = vec![];
                let mut stmts: Vec<Statement> = vec![];
                // params
                loop {
                    match self.get_kind() {
                        TokenKind::Comma => self.p += 1,
                        TokenKind::Identifier => {
                            let param_identifier = self.tokens[self.p].value.to_string();
                            params.push(param_identifier);
                            self.p += 1;
                        }
                        TokenKind::RightParen => break,
                        actual => {
                            return Err(Error {
                                line: self.get_token().line,
                                kind: ErrorKind::UnexpectedToken(None, actual),
                                cols: self.get_cols(),
                            })
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
                        stmts.push(self.parse_stmt()?);
                    }
                }

                Ok(Expr::Function(params, stmts))
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
                Ok(Expr::Object(res))
            }
            actual => Err(Error {
                line: self.get_token().line,
                kind: ErrorKind::UnexpectedToken(None, *actual),
                cols: self.get_cols(),
            }),
        }
    }

    fn parse_infix(&mut self, expr: &Expr, token_kind: TokenKind) -> Result<Expr> {
        let right = self.parse_expr(infix_precedence(&token_kind))?;

        match token_kind {
            TokenKind::Equal => match expr {
                Expr::Variable(s) => Ok(Expr::Operator(
                    Box::new(Expr::String(s.to_string())),
                    token_kind,
                    Box::new(right),
                )),
                Expr::Get(s, g_expr) => {
                    Ok(Expr::Set(s.to_string(), g_expr.clone(), Box::new(right)))
                }
                Expr::Index(e1, e2) => Ok(Expr::SetList(e1.clone(), e2.clone(), Box::new(right))),
                _ => Err(Error {
                    line: self.get_token().line,
                    kind: ErrorKind::Assignment,
                    cols: self.get_cols(),
                }),
            },
            _ => Ok(Expr::Operator(
                Box::new(expr.clone()),
                token_kind,
                Box::new(right),
            )),
        }
    }

    fn consume(&mut self, expected_token: TokenKind) -> Result<()> {
        let actual = self.tokens[self.p].kind;
        if expected_token != actual {
            return Err(Error {
                line: self.tokens[self.p].line,
                kind: ErrorKind::UnexpectedToken(Some(expected_token), actual),
                cols: self.get_cols(),
            });
        }
        self.p += 1;
        Ok(())
    }

    fn get_cols(&mut self) -> Option<(usize, usize)> {
        Some((self.tokens[self.p].start, self.tokens[self.p].column))
    }

    fn get_token(&mut self) -> &Token {
        &self.tokens[self.p]
    }

    fn get_kind(&mut self) -> TokenKind {
        self.tokens[self.p].kind.clone()
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
        TokenKind::Equal => 1,
        TokenKind::Arrow => 2,
        TokenKind::Or => 3,
        TokenKind::And => 4,
        TokenKind::BangEqual | TokenKind::EqualEqual => 5,
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => 6,
        TokenKind::Plus | TokenKind::Minus => 7,
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 8,
        TokenKind::Dot | TokenKind::Colon => 9,
        _ => 0,
    }
}
fn prefix_precedence(_kind: &TokenKind) -> usize {
    8
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Declaration {
    Function(String, Vec<String>, Vec<Statement>),
    // Let(String, Expr),
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
pub enum Expr {
    Prefix(TokenKind, Box<Expr>),
    Operator(Box<Expr>, TokenKind, Box<Expr>),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Function(Vec<String>, Vec<Statement>),
    Get(String, Box<Expr>),
    Set(String, Box<Expr>, Box<Expr>),
    Call(Vec<Expr>, Box<Expr>),
    Variable(String),
    Object(Vec<Expr>),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    SetList(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[cfg(test)]
mod tests {}
