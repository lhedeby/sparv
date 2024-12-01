use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub start: usize,
    pub end: usize,
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
    Comment,
}

pub struct ParseTokenKindError;

impl TokenKind {
    pub fn to_hover_text(&self) -> String {
        match self {
            TokenKind::LeftParen => "".to_string(),
            TokenKind::RightParen => "".to_string(),
            TokenKind::LeftBracket => "".to_string(),
            TokenKind::RightBracket => "".to_string(),
            TokenKind::LeftBrace => "".to_string(),
            TokenKind::RightBrace => "".to_string(),
            TokenKind::Comma => "".to_string(),
            TokenKind::Dot => "".to_string(),
            TokenKind::Minus => "Subtracts two numbers".to_string(),
            TokenKind::Plus => "Adds two numbers or concatenates strings".to_string(),
            TokenKind::Semicolon => "".to_string(),
            TokenKind::Slash => "Divides two numbers".to_string(),
            TokenKind::Star => "Multiplies tow numbers".to_string(),
            TokenKind::Colon => "".to_string(),
            TokenKind::Percent => "Remainder/modulo".to_string(),
            TokenKind::Bang => "Negates the next boolean".to_string(),
            TokenKind::BangEqual => "Check if two expressions are NOT EQUAL to each other".to_string(),
            TokenKind::Equal => "Assign the right hand side expression to the left hand side variable".to_string(),
            TokenKind::EqualEqual => "Check if two expressions are EQUAL to each other".to_string(),
            TokenKind::Greater => "Check if the the left hand side is GREATER than the left hand side".to_string(),
            TokenKind::GreaterEqual => "Check if the the left hand side is GREATER OR EQUAL to the left hand side".to_string(),
            TokenKind::Less => "Check if the the left hand side is LESS than the left hand side".to_string(),
            TokenKind::LessEqual => "Check if the the left hand side is LESS OR EQUAL to the left hand side".to_string(),
            TokenKind::Arrow => "Passes the result of the left hand side as a parameter to the right hand side function".to_string(),
            TokenKind::PlusEqual => "".to_string(),
            TokenKind::MinusEqual => "".to_string(),
            TokenKind::Identifier(i) => format!("Identifier: '{}'", i),
            TokenKind::String(s) => format!("String: '{}'", s),
            TokenKind::Number(n) => format!("Number: '{}'", n),
            TokenKind::Let => "Declares a variable".to_string(),
            TokenKind::And => "and".to_string(),
            TokenKind::Else => "else".to_string(),
            TokenKind::False => "The boolean value 'false'".to_string(),
            TokenKind::For => "A for loop".to_string(),
            TokenKind::Fun => "Function declaration".to_string(),
            TokenKind::If => "If statement".to_string(),
            TokenKind::Null => "Nulljust nothing".to_string(),
            TokenKind::Or => "or".to_string(),
            TokenKind::Return => "Return the following expression from the function".to_string(),
            TokenKind::True => "The boolean value 'true'".to_string(),
            TokenKind::In => "".to_string(),
            TokenKind::While => "A while loop".to_string(),
            TokenKind::Import => "Import a file to be used".to_string(),
            TokenKind::Eof => "".to_string(),
            TokenKind::Comment => "Just a comment".to_string(),
        }
    }
}

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
