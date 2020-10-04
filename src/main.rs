use std::io::stdin;
use std::io::BufRead;

mod lexer;

fn process(buf : String) {
    let mut lexer = lexer::Lexer::new(buf);

    while let Some(token) = lexer.next().unwrap() {
        println!("{:?}", token)
    }
}

fn main() {
    for line in stdin().lock().lines() {
        match line {
            Ok(line) => process(line),
            Err(e) => panic!("{}", e),
        }
    }
}
