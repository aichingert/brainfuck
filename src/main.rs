use std::path::Path;

mod scanner;
mod error;
mod token;

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: brafu");
        std::process::exit(0);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("unable to open location");
    } else {
        println!("Provide a path to the source");
    }
}

fn run_file<T: AsRef<Path>>(path: T) -> std::io::Result<()> {
    let source: String = std::fs::read_to_string(path)?;

    Ok(())
}

fn run(source: &String) {

}