pub struct Lexer {
    buffer: Vec<char>,
    pointer: usize,
}

#[derive(Debug)]
pub enum Token {
    Ident(String),
    RParen,
    LParen,

    Declare,
    Assign,
    
    If,
    Else,
    Invalid,
}

impl Lexer {
    pub fn new(buffer: Vec<char>) -> Self {
        Self { buffer, pointer: 0usize }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        while self.pointer < self.buffer.len() && self.buffer[self.pointer] == ' ' {
            self.pointer += 1;
        }

        if self.pointer >= self.buffer.len() {
            return None;
        }

        let token = match self.buffer[self.pointer] {
            '(' => Token::LParen,
            ')' => Token::RParen,
            ':' => Token::Declare,
            'a'..='z' | 'A'..='Z' => self.read_identifier(),
            _ => Token::Invalid,
        };
        self.pointer += 1;

        Some(token)
    }

    fn read_identifier(&mut self) -> Token {
        let mut acc = String::new();
        let mut cur: char = self.buffer[self.pointer];

        while self.pointer < self.buffer.len() && cur != ' ' && cur != '\n' {
            acc.push(cur);
            self.pointer += 1;
            cur = self.buffer[self.pointer];
        }

        match acc.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(acc)
        }
    }
}
