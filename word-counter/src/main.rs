use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use word_counter::split;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: word-counter <filename>");
        process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = 0;
    for line in reader.lines() {
        res += split(&line.unwrap()).count()
    }
    println!("{} words in {}", res, filename);
}
