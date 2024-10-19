use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub start: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Colon,
    Percent,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    PlusEqual,
    MinusEqual,
    // Literals.
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords.
    Let,
    And,
    Else,
    False,
    For,
    Fun,
    If,
    Null,
    Or,
    Return,
    True,
    In,
    While,
    Import,
    Eof,
    // comments
    Comment
}

pub struct ParseTokenKindError;

impl FromStr for TokenKind {
    type Err = ParseTokenKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "let" => Ok(TokenKind::Let),
            "true" => Ok(TokenKind::True),
            "and" => Ok(TokenKind::And),
            "else" => Ok(TokenKind::Else),
            "if" => Ok(TokenKind::If),
            "null" => Ok(TokenKind::Null),
            "or" => Ok(TokenKind::Or),
            "return" => Ok(TokenKind::Return),
            "while" => Ok(TokenKind::While),
            "false" => Ok(TokenKind::False),
            "for" => Ok(TokenKind::For),
            "fun" => Ok(TokenKind::Fun),
            "in" => Ok(TokenKind::In),
            "import" => Ok(TokenKind::Import),
            _ => Err(ParseTokenKindError),
        }
    }
}
