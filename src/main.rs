use std::io::Write;

use crate::scanner::Scanner;

pub mod scanner;
pub mod token;
pub mod token_type;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.len() {
        len if len > 1 => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
        1 => {
            run_file(args[0].clone());
        }
        _ => {
            run_prompt();
        }
    }
}
fn run_file(path: String) {
    let bytes: Vec<u8> = std::fs::read(path).unwrap();
    run(String::from_utf8(bytes).unwrap());

    if unsafe { HAD_ERROR } {
        std::process::exit(65);
    }
}

fn run_prompt() {
    let mut reader = std::io::stdin().lines().map_while(Result::ok);
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let Some(line) = reader.next() else {
            break;
        };
        run(line);
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token}");
    }
}

fn error(line: usize, message: String) {
    report(line, "".into(), message);
}

static mut HAD_ERROR: bool = false;

fn report(line: usize, location: String, message: String) {
    println!("[line {line}] Error {location}: {message}");
    unsafe {
        HAD_ERROR = true;
    }
}
