use std::{
    env,
    fs::{self, ReadDir},
    path::Path,
    process,
};

fn read_dir_safe(path: &Path) -> Option<ReadDir> {
    if let Ok(kind) = fs::metadata(path).map(|x| x.file_type()) {
        if kind.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                return Some(entries);
            }
        }
    }
    None
}

const BLACKLISTED: [&'static str; 2] = ["target", ".git"];

fn print_tree(path: &Path, depth: usize) {
    if let Some(basename) = path.file_name().map(|x| x.to_string_lossy()) {
        if BLACKLISTED.contains(&basename.as_ref()) {
            return;
        }
        for _ in 0..(depth + 1) {
            print!("|    ");
        }
        println!("{}", basename);
    }

    if let Some(entries) = read_dir_safe(path) {
        for entry in entries.filter_map(Result::ok) {
            print_tree(&entry.path(), depth + 1);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Expect exactly one argument, got {}", args.len());
        println!("Usage: tree <directory-path>");
        process::exit(1)
    }

    let root = args[1].clone();
    print_tree(Path::new(&root), 0);
}
