use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use word_counter::{count_words, split};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: word-counter <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let word_iter = reader.lines().filter_map(Result::ok).flat_map(|line| {
        split(&line)
            .map(|word| word.to_string())
            .collect::<Vec<_>>()
    });

    for (word, count) in count_words(word_iter) {
        println!("{}: {}", word, count)
    }
}
