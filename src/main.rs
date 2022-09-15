use std::path::Path;
use std::io::{self, Read, BufReader, BufRead};
use std::fs::File;

mod scanner;
mod error;
mod token;

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: brafu [script]");
        std::process::exit(0);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("unable to open location");
    } else {
        run_prompt();
    }
}

fn run_file<T: AsRef<Path>>(path: T) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf: Vec<u8> = vec![];

    reader.read_to_end(&mut buf)?;

    run(&buf);
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    
}

fn run(source: &[u8]) {

}