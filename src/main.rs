mod scanner;

use scanner::Scanner;
use std::io::Write;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: zero or one argument needed (filename)");
    } else if args.len() == 2 {
        let source_code = fs::read_to_string(&args[1]).expect("Failed to read file");
        run(&source_code);
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.pop();
        if line.is_empty() {
            break;
        }
        run(&line);
    }
}

fn run(source: &str) {
    let mut s = Scanner::new(source);
    if let Ok(tokens) = s.scan_tokens() {
        for token in tokens {
            println!("{:?},", token);
        }
    }
}
