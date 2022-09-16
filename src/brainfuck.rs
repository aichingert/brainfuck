pub struct Brainfuck {
    buffer: Vec<u8>,
    pointer: usize
}

pub enum Instructions {
    Increment,
    Decrement,
    PointerInc,
    PointerDec,
    Write,
    Read,
    Loop(Vec<Instructions>)
}

impl Brainfuck {
    pub fn run(&mut self) {

    }

    fn parse() -> Vec<Instructions> {
        vec![]
    }
}