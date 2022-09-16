use crate::opcode::OpCode;

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
            match operations[i - offset] {
                OpCode::None => {
                    operations.remove(i - offset);
                    offset += 1;
                },
                _ => {}
            }
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use crate::lexeme::Lexeme;
    use crate::opcode::OpCode;

    #[test]
    pub fn parse_correctly() {
        let path: &String = &"examples/parse_test.bf".to_string();
        let lex: Lexeme = Lexeme::new(std::fs::read_to_string(path).expect("unable to open file"));

        let result: Vec<OpCode> = lex.lex();
        let expect: Vec<OpCode> = vec![
            OpCode::LoopBegin, OpCode::LoopBegin, OpCode::Increment, OpCode::Increment, 
            OpCode::Decrement, OpCode::Decrement, OpCode::LoopEnd, OpCode::LoopEnd];

        assert!(result == expect)
    }
}