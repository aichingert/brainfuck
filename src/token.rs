use std::fmt;

pub enum TokenType {
    Increment,
    Decrement,
    CellIncrement,
    CellDecrement,
    Print(u8),
    Get(u8),
    Open,
    Close,
    Eof
}

pub struct Token {
    token: TokenType,
    lexeme: String,
    line: i32
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, line: i32) -> Self {
        Self {
            token,
            lexeme,
            line
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token, self.lexeme, self.line)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TokenType::Increment => "Increment",
            TokenType::Decrement => "Decrement",
            TokenType::CellIncrement => "CellIncrement",
            TokenType::CellDecrement => "CellDecrement",
            TokenType::Print(_) => "Print",
            TokenType::Get(_) => "Get",
            TokenType::Open => "Open",
            TokenType::Close => "Close",
            TokenType::Eof => "End of line",
        })
    }
}