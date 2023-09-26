mod ast;
use ast::Lexer;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run [file.bs]");
        std::process::exit(1);
    }

    let source: Vec<char> = std::fs::read_to_string(&args[1])?
        .trim()
        .to_lowercase()
        .chars().collect::<Vec<char>>();

    let mut lexer: Lexer = Lexer::new(source);

    while let Some(token) = lexer.get_token() {
        print!("{:?} ", token);
    }

    Ok(())
}
