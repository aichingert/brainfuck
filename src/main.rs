use std::path::Path;
use std::io::{self, Read, BufReader, BufRead};
use std::fs::File;

mod scanner;
mod token;

use scanner::Scanner;


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

    run(source.as_str());
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

            run(&line)
        } else {
            break;
        }
    }
}

fn run(source: &str) {
    let scanner: Scanner = Scanner::new(*source);
    let tokens = scanner.scanTokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

struct Brainfuck {
    had_error: bool,
}

impl Brainfuck {
    pub fn error(&self, line: i32, message: &String) {
        self.report(line, &"".to_string(), message)
    }
    
    pub fn report(&mut self, line: i32, location: &String, message: &String) {
        let report: String = format!("[line {line}] Error {location}: {message}");
        println!("{report}");
        self.had_error = true;
    }
}