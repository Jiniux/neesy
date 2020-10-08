use std::io::stdin;
use std::io::BufRead;

mod lexer;
mod parser;

use parser::Parser;
use parser::Precedence;

use lexer::Lexer;

fn process(buf : String) {
    let mut lexer = Lexer::new(buf);

    let tokens = match lexer.collect() {
        Ok(tokens) => tokens,
        Err(err) => return println!("{}", err) 
    };


    let mut parser = Parser::new(tokens.iter().peekable());
    match parser.parse_expression(Precedence::Lowest) {
        Ok(expr) => println!("{:?}", expr),
        Err(string) => println!("{}", string)
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
