#[derive(Debug)]
pub struct BrainfuckError {
    line: usize,
    log: String
}

pub fn error(line: usize, log: String) -> BrainfuckError {
    BrainfuckError { line, log }
}

pub fn report(err: BrainfuckError, location: String) {
    eprintln!("[line {}] Error: {}", err.line, err.log)
}