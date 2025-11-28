use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: word-counter <filename>");
        process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Error reading filename {}: {}", filename, err);
        process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut count: usize = 0;

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            count += line.split_whitespace().count();
        }
    }

    println!("{} words", count);
}
