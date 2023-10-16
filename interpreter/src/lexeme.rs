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
        self.source.chars()
            .map(|symbol|
                match symbol {
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
            .filter(|op| *op != OpCode::None)
            .collect::<Vec<OpCode>>()
    }
}

#[cfg(test)]
mod test {
    use crate::lexeme::Lexeme;
    use crate::opcode::OpCode;

    #[test]
    pub fn lexeme_created_correctly() {
        let path: &String = &"examples/parse_test.bf".to_string();
        let lex: Lexeme = Lexeme::new(std::fs::read_to_string(path).expect("unable to open file"));

        let result: Vec<OpCode> = lex.lex();
        let expect: Vec<OpCode> = vec![
            OpCode::LoopBegin, OpCode::LoopBegin, OpCode::Increment, OpCode::Increment, 
            OpCode::Decrement, OpCode::Decrement, OpCode::LoopEnd, OpCode::LoopEnd];

        assert!(result == expect)
    }
}
