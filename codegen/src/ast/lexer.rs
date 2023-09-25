pub struct Lexer {
    buffer: Vec<char>,
    pointer: usize,
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Invalid,
}

impl Lexer {
    pub fn new(buffer: Vec<char>) -> Self {
        Self { buffer, pointer: 0usize }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        self.pointer += 1;
        if self.pointer >= self.buffer.len() {
            return None;
        }

        match self.buffer[self.pointer - 1] {
            'A' => Some(Token::Ident(String::from("Hello"))),
            _ => Some(Token::Invalid),
        }
    }
}
