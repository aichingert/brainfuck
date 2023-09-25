mod ast;
use ast::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new(vec!['a', 'A', 'b']);

    while let Some(token) = lexer.get_token() {
        println!("{:?}", token);
    }
}
