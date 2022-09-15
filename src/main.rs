use std::path::Path;
use std::io::{self, BufRead};

mod scanner;
mod token;
mod error;

use scanner::Scanner;
use error::*;

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
    let source: &String = &std::fs::read_to_string(path)?;

    match run(source) {
        Ok(_) => {},
        Err(error) => {
            report(error, "".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }

            match run(&line) {
                Ok(_) => {},
                Err(error) => {
                    report(error, "".to_string());
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: &str) -> Result<(), BrainfuckError> {
    let scanner: Scanner = Scanner::new(*source);
    let tokens = scanner.scanTokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}