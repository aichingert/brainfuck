use crate::token::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![]
        }
    }

    pub fn scanTokens(&self) {
        let mut start: usize = 0;
        let mut current: usize = 0;
        let mut line: usize = 1;

        while !self.is_at_end(current) {
            start = current;

        }
    }

    pub fn is_at_end(&self, current: usize) -> bool {
        self.source.len() <= current
    }
}