use std::str::FromStr;

use crate::{
    error::{Error, ErrorKind, Result},
    token::{Token, TokenKind},
};

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    source: String,
}

impl Scanner {
    pub fn get_tokens(source: String) -> Result<Vec<Token>> {
        let mut scanner = Scanner {
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            source,
        };
        let mut res = vec![];
        loop {
            let token = scanner.next_token()?;
            // println!("token: {:?}", token);
            let kind = token.kind;
            res.push(token);
            match kind {
                TokenKind::Eof => break,
                _ => {}
            }
        }
        Ok(res)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace()?;
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance().unwrap();

        match c {
            'a'..='z' | 'A'..='Z' => self.identifier(),
            '0'..='9' => self.number(),
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '[' => self.make_token(TokenKind::LeftBracket),
            ']' => self.make_token(TokenKind::RightBracket),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '-' => {
                let token = if self.check_next('>') {
                    TokenKind::Arrow
                } else if self.check_next('=') {
                    TokenKind::MinusEqual
                } else {
                    TokenKind::Minus
                };
                self.make_token(token)
            }
            '+' => {
                let token = if self.check_next('=') {
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                };
                self.make_token(token)
            }
            '/' => self.make_token(TokenKind::Slash),
            '*' => self.make_token(TokenKind::Star),
            ':' => self.make_token(TokenKind::Colon),
            '%' => self.make_token(TokenKind::Percent),
            '!' => {
                let token = if self.check_next('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.make_token(token)
            }
            '=' => {
                let token = if self.check_next('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.make_token(token)
            }
            '<' => {
                let token = if self.check_next('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.make_token(token)
            }
            '>' => {
                let token = if self.check_next('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.make_token(token)
            }
            '"' => self.string(),
            _ => Err(Error {
                line: self.line,
                kind: ErrorKind::UnexpectedCharacter,
                cols: Some((self.start, self.current)),
            }),
        }
    }

    fn identifier(&mut self) -> Result<Token> {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphabetic() || c.is_digit(10) || c == '_')
        {
            self.advance();
        }
        self.make_token(self.identifier_kind())
    }

    fn identifier_kind(&self) -> TokenKind {
        match TokenKind::from_str(&self.source[self.start..self.current]) {
            Ok(kind) => kind,
            Err(_) => TokenKind::Identifier,
        }
    }

    fn number(&mut self) -> Result<Token> {
        while self.peek().is_some()
            && (self.peek().unwrap().is_digit(10) || self.peek().unwrap().eq(&'.'))
        {
            self.advance();
        }
        self.make_token(TokenKind::Number)
    }

    fn string(&mut self) -> Result<Token> {
        let line = self.line;
        while self.peek().is_some() && self.peek().unwrap() != '"' && !self.is_at_end() {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
                self.advance();
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(Error {
                line,
                kind: ErrorKind::UnterminatedString,
                cols: Some((self.start, self.current)),
            });
        }
        self.advance();
        self.make_token(TokenKind::String)
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        //self.start+=1;
                        _ = self.advance()
                    }
                    '\n' => {
                        self.line += 1;
                        self.column = 0;
                        self.advance();
                    }
                    '/' => {
                        if self.peek_next().is_some() && self.peek_next().unwrap() == '/' {
                            while self.peek().is_some()
                                && self.peek().unwrap() != '\n'
                                && !self.is_at_end()
                            {
                                self.advance();
                            }
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.source.chars().nth(self.current + 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn check_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> Option<char> {
        self.column += 1;
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn make_token(&self, kind: TokenKind) -> Result<Token> {
        let value_range = match kind {
            TokenKind::String => (self.start + 1)..(self.current - 1),
            _ => self.start..self.current,
        };
        Ok(Token {
            kind,
            line: self.line,
            column: self.column,
            start: self.column - (self.current - self.start),
            value: self
                .source
                .get(value_range)
                .ok_or(Error {
                    line: self.line,
                    kind: ErrorKind::Unknown,
                    cols: Some((self.start, self.current)),
                })?
                .to_string(),
        })
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
