use std::{env, io::{self, BufRead, Write}};

mod lexer;
mod ast;
mod parser;
mod stmt;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        println!("args 3");
    } else if args.len() == 2 {
        println!("args 2");
    } else if args.len() == 1 {
        println!("args 1");
    }
}

#[allow(dead_code)]
fn run_prompt() -> Result<(), String> {
    loop {
        print!(">>");
        
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush stdout".to_string())
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    return Ok(())
                } else if n == 1 {
                    continue;
                }
            },
            Err(_) => return Err("Could not read line".to_string())
        }

        println!("<< {}", buffer);

    }
}