pub struct Lexer {
    buffer: Vec<char>,
    pointer: usize,
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Num(u32),

    RParen,
    LParen,

    Not,
    Equals,

    Declare,
    
    If,
    Else,
    Invalid,
}

impl Lexer {
    pub fn new(buffer: Vec<char>) -> Self {
        Self { buffer, pointer: 0usize }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        while self.pointer < self.buffer.len() && self.is_position_skipable() {
            self.pointer += 1;
        }

        if self.pointer >= self.buffer.len() {
            return None;
        }

        let token = match self.buffer[self.pointer] {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '!' => Token::Not,
            '=' => Token::Equals,
            ':' if self.pointer + 1 < self.buffer.len() && self.buffer[self.pointer + 1] == '=' => {
                self.pointer += 1;
                Token::Declare
            }
            'a'..='z' | 'A'..='Z' => self.read_identifier(),
            '0'..='9' => self.read_number(),
            _ => Token::Invalid,
        };
        self.pointer += 1;

        Some(token)
    }

    fn read_identifier(&mut self) -> Token {
        let mut acc = String::new();

        while self.pointer < self.buffer.len() && !self.is_position_skipable() {
            acc.push(self.buffer[self.pointer]);
            self.pointer += 1;
        }

        match acc.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(acc)
        }
    }

    fn read_number(&mut self) -> Token {
        let mut acc = String::new();

        while self.pointer < self.buffer.len() && !self.is_position_skipable() {
            acc.push(self.buffer[self.pointer]);
            self.pointer += 1;
        }

        Token::Num(acc.parse().expect("a number"))
    }

    fn is_position_skipable(&self) -> bool {
        self.buffer[self.pointer] == ' ' || self.buffer[self.pointer] == '\t' || self.buffer[self.pointer] == '\n'
    }
}
