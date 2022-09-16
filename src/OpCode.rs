#[derive(Debug, Partial)]
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