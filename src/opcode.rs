#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    Increment,
    Decrement,
    PointerInc,
    PointerDec,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
    None,
}