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
        if c.is_ascii_alphabetic() {
            return self.identifier();
        }
        if c.is_digit(10) {
            return self.number();
        }
        match c {
            '(' => return self.make_token(TokenKind::LeftParen),
            ')' => return self.make_token(TokenKind::RightParen),
            '[' => return self.make_token(TokenKind::LeftBracket),
            ']' => return self.make_token(TokenKind::RightBracket),
            '{' => return self.make_token(TokenKind::LeftBrace),
            '}' => return self.make_token(TokenKind::RightBrace),
            ';' => return self.make_token(TokenKind::Semicolon),
            ',' => return self.make_token(TokenKind::Comma),
            '.' => return self.make_token(TokenKind::Dot),
            '-' => {
                let token = if self.check_next('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                };
                return self.make_token(token);
            }
            '+' => {
                let token = if self.check_next('=') {
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                };
                return self.make_token(token);
            }
            '/' => return self.make_token(TokenKind::Slash),
            '*' => return self.make_token(TokenKind::Star),
            ':' => return self.make_token(TokenKind::Colon),
            '%' => return self.make_token(TokenKind::Percent),
            '!' => {
                let token = if self.check_next('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                return self.make_token(token);
            }
            '=' => {
                let token = if self.check_next('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                return self.make_token(token);
            }
            '<' => {
                let token = if self.check_next('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                return self.make_token(token);
            }
            '>' => {
                let token = if self.check_next('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                return self.make_token(token);
            }
            '"' => return self.string(),
            _ => {}
        }

        // Unexpected character
        Err(Error {
            line: self.line,
            kind: ErrorKind::UnexpectedCharacter,
            cols: Some((self.start, self.current)),
        })
        // Err(LexingError::UnexpectedCharacter(self.line, self.column, c))
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

    fn identifier_list(&self) -> Vec<(String, TokenKind)> {
        vec![
            ("let".to_string(), TokenKind::Let),
            ("true".to_string(), TokenKind::True),
            ("and".to_string(), TokenKind::And),
            ("else".to_string(), TokenKind::Else),
            ("if".to_string(), TokenKind::If),
            ("null".to_string(), TokenKind::Null),
            ("or".to_string(), TokenKind::Or),
            ("return".to_string(), TokenKind::Return),
            ("while".to_string(), TokenKind::While),
            ("false".to_string(), TokenKind::False),
            ("for".to_string(), TokenKind::For),
            ("fun".to_string(), TokenKind::Fun),
            ("in".to_string(), TokenKind::In),
            ("import".to_string(), TokenKind::Import),
        ]
    }

    fn identifier_kind(&self) -> TokenKind {
        for (s, kind) in self.identifier_list() {
            if self.start + s.len() < self.source.len()
                && &self.source[self.start..self.current] == &s
            {
                return kind;
            }
        }
        TokenKind::Identifier
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
        match kind {
            TokenKind::String => Ok(Token {
                kind,
                line: self.line,
                column: self.column,
                start: self.column - (self.current - self.start),
                value: self
                    .source
                    .get((self.start + 1)..(self.current - 1))
                    .ok_or(Error {
                        line: self.line,
                        kind: ErrorKind::Unknown,
                        cols: Some((self.start, self.current)),
                    })?
                    .to_string(),
            }),
            _ => Ok(Token {
                kind,
                line: self.line,
                column: self.column,
                start: self.column - (self.current - self.start),
                value: self
                    .source
                    .get(self.start..self.current)
                    .ok_or(Error {
                        line: self.line,
                        kind: ErrorKind::Unknown,
                        cols: Some((self.start, self.current)),
                    })?
                    .to_string(),
            }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {}
