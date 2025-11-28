use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: word-counter <filename>");
        process::exit(1);
    }

    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(text) => println!("{} words", text.split_whitespace().count()),
        Err(err) => {
            eprintln!("Error reading filename {}: {}", filename, err);
            process::exit(1)
        }
    }
}
