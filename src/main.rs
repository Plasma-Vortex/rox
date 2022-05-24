mod scanner;

use std::{env, fs, io};
use std::io::Write;
use scanner::Scanner;

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
        io::stdout().flush();
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
    let s = Scanner::new(source);
    let tokens = s.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}


