use std::env;

mod lexer;

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
