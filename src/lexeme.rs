use crate::OpCode::OpCode;

pub struct Lexeme {
    source: String
}

impl Lexeme {
    pub fn new(source: String) -> Self {
        Self {
            source
        }
    }

    pub fn lex(&self) -> Vec<OpCode> {
        let mut operations: Vec<OpCode> = Vec::new();

        for symbol in self.source.chars() {
            operations.push(match symbol {
                '>' => OpCode::PointerInc,
                '<' => OpCode::PointerDec,
                '+' => OpCode::Increment,
                '-' => OpCode::Decrement,
                '.' => OpCode::Write,
                ',' => OpCode::Read,
                '[' => OpCode::LoopBegin,
                ']' => OpCode::LoopEnd,
                _ => OpCode::None,
            })
        }

        let mut offset: usize = 0;
        for i in 0..operations.len() {
            if operations[i] == None {

            }
        }

        operations
    }
}