use std::{fs::File, io::Read};

mod lexeme;
mod OpCode;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: Brainfuck: [file.bf]");
        std::process::exit(1);
    }

    let path: &String = &args[1];

    let source: &String = &std::fs::read_to_string(path)?;

    Ok(())
}