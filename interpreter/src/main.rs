mod lexeme;
mod opcode;
mod brainfuck;

use lexeme::Lexeme;
use opcode::OpCode;
use brainfuck::{Brainfuck as bf, parse, Instruction};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: Brainfuck: [file.bf]");
        std::process::exit(1);
    }

    let path: &String = &args[1];

    let source: &String = &std::fs::read_to_string(path)?;
    let lexame: Lexeme = Lexeme::new(source.clone());

    let opcodes: Vec<OpCode> = lexame.lex();
    let program: Vec<Instruction> = parse(&opcodes);

    let mut bf: bf = bf::new();
    bf.run(&program);
    
    Ok(())
}
