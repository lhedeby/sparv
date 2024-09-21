use crate::scanner::{Token, TokenKind};

pub struct Parser {
    p: usize,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ParserError {}

type Result<T> = std::result::Result<T, ParserError>;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Declaration>> {
        println!("Started parsing...");
        println!("tokens: {:?}", tokens);
        let mut parser = Parser { p: 0, tokens };
        let mut decls: Vec<Declaration> = vec![];
        while parser.tokens[parser.p].kind != TokenKind::Eof {
            let decl = parser.parse_decl()?;
            decls.push(decl);
        }
        println!("Done parsing...");
        println!("=== DECLARATIONS ===");
        for d in &decls {
            println!("{:?}", d);
        }
        println!("\n");

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
            _ => Ok(Declaration::Statement(self.parse_stmt()?)),
        }
    }

    fn fun_decl(&mut self) -> Result<Declaration> {
        self.p += 1;
        self.consume(TokenKind::Identifier);
        let fun_identifier = self.tokens[self.p - 1].value.to_string();
        self.consume(TokenKind::LeftParen);
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
                _ => panic!("Expected Identifier, Comma or Right paren"),
            }
        }
        // while self.get_kind() != TokenKind::RightParen {
        //     println!("loop, kind: {:?}", self.get_kind());
        //     let param_identifier = self.tokens[self.p].value.to_string();
        //     params.push(param_identifier);
        //     self.p += 2;
        // }
        self.consume(TokenKind::RightParen);
        self.consume(TokenKind::LeftBrace);
        while self.get_kind() != TokenKind::RightBrace {
            stmts.push(self.parse_stmt()?);
        }
        self.consume(TokenKind::RightBrace);

        Ok(Declaration::Function(fun_identifier, params, stmts))
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
            TokenKind::Print => self.print_stmt(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Identifier => self.identifier_stmt(),
            TokenKind::For => self.for_stmt(),
            _ => panic!("Unexpected token '{:?}'.", self.get_kind()),
        }
    }

    fn let_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        self.consume(TokenKind::Identifier);
        let identifier = self.tokens[self.p - 1].value.to_string();
        self.consume(TokenKind::Equal);

        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::Semicolon);
        Ok(Statement::Let(identifier, expr))
    }

    fn while_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let mut stmts: Vec<Statement> = vec![];
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::LeftBrace);

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt();
            stmts.push(stmt?);
        }
        self.consume(TokenKind::RightBrace);
        Ok(Statement::While(expr, stmts))
    }

    fn if_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let mut if_stmts: Vec<Statement> = vec![];
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::LeftBrace);

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt();
            if_stmts.push(stmt?);
        }
        self.consume(TokenKind::RightBrace);

        if self.get_kind() != TokenKind::Else {
            return Ok(Statement::If(expr, if_stmts, vec![]));
        }
        self.p += 1;

        let mut else_stmts: Vec<Statement> = vec![];
        self.consume(TokenKind::LeftBrace);

        while self.get_kind() != TokenKind::RightBrace {
            let stmt = self.parse_stmt();
            else_stmts.push(stmt?);
        }
        self.consume(TokenKind::RightBrace);

        Ok(Statement::If(expr, if_stmts, else_stmts))
    }
    fn print_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let expr = self.parse_expr(0)?;
        self.consume(TokenKind::Semicolon);
        Ok(Statement::Print(expr))
    }
    fn return_stmt(&mut self) -> Result<Statement> {
        self.p += 1;
        let expr = if self.tokens[self.p].kind != TokenKind::Semicolon {
            self.parse_expr(0)?
        } else {
            Expr::Null
        };
        self.consume(TokenKind::Semicolon);
        Ok(Statement::Return(expr))
    }
    fn identifier_stmt(&mut self) -> Result<Statement> {
        let res = Ok(Statement::Expression(self.parse_expr(0)?));
        self.consume(TokenKind::Semicolon);
        res
    }
    fn for_stmt(&mut self) -> Result<Statement> {
        todo!()
    }

    /*
     *
     *   === EXPRESSIONS ===
     *
     */

    fn parse_expr(&mut self, precedence: usize) -> Result<Expr> {
        let kind = &self.tokens[self.p].kind.clone();
        self.p += 1;

        let mut left = self.parse_prefix(kind).unwrap().clone();

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
                self.consume(TokenKind::RightBracket);
                Ok(Expr::List(items))
            }
            TokenKind::Identifier => {
                let identifier = self.tokens[self.p - 1].value.clone();
                println!("kind: {:?}", self.get_kind());
                let mut expr = Expr::Variable(identifier);
                loop {
                    match self.get_kind() {
                        TokenKind::Dot => {
                            self.p += 1;
                            let i = self.tokens[self.p].value.clone();
                            self.consume(TokenKind::Identifier);
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

                            self.consume(TokenKind::RightParen);
                            expr = Expr::Call(params, Box::new(expr))
                        }
                        TokenKind::LeftBracket => {
                            self.p += 1;
                            expr = Expr::Index(Box::new(expr), Box::new(self.parse_expr(0)?));
                            self.consume(TokenKind::RightBracket);
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
            TokenKind::LeftParen => {
                let expr = self.parse_expr(0);
                self.consume(TokenKind::RightParen);
                expr
            }
            TokenKind::LeftBrace => {
                let mut res = vec![];
                while self.get_kind() != TokenKind::RightBrace {
                    let expr = self.parse_expr(0);
                    res.push(expr?);
                    self.consume(TokenKind::Comma);
                }
                self.consume(TokenKind::RightBrace);
                Ok(Expr::Object(res))
            }

            /*
                        BUILT INS
            */
            TokenKind::ReadFile => Ok(Expr::ReadFile(Box::new(self.parse_expr(0)?))),
            TokenKind::ReadInput => Ok(Expr::ReadInput),
            _ => panic!("token is: {:?}", token_kind),
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
                _ => {
                    panic!("Must be variable or get? {:?}", expr)
                }
            },
            _ => Ok(Expr::Operator(
                Box::new(expr.clone()),
                token_kind,
                Box::new(right),
            )),
        }
    }

    fn consume(&mut self, expected_token: TokenKind) {
        if expected_token != self.tokens[self.p].kind {
            panic!(
                "Unexpected token. Expected '{:?}' but got '{:?}'",
                expected_token, self.tokens[self.p].kind
            );
        }
        self.p += 1;
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
        TokenKind::Or => 2,
        TokenKind::And => 3,
        TokenKind::BangEqual | TokenKind::EqualEqual => 4,
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => 5,
        TokenKind::Plus | TokenKind::Minus => 6,
        TokenKind::Star | TokenKind::Slash => 7,
        TokenKind::Dot => 9,
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
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expr),
    Let(String, Expr),
    For,
    If(Expr, Vec<Statement>, Vec<Statement>),
    Print(Expr),
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
    Get(String, Box<Expr>),
    Set(String, Box<Expr>, Box<Expr>),
    Call(Vec<Expr>, Box<Expr>),
    Variable(String),
    Object(Vec<Expr>),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    SetList(Box<Expr>, Box<Expr>, Box<Expr>),
    ReadFile(Box<Expr>),
    ReadInput,
}
