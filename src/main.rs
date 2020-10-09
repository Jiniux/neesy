use std::io::stdin;
use std::io::BufRead;

mod lexer;
mod parser;
mod evaluator;

use parser::Parser;
use parser::Precedence;

use evaluator::Evaluator;

use lexer::Lexer;

fn process(buf : String, evaluator: &mut Evaluator) {
    let mut lexer = Lexer::new(buf);

    let tokens = match lexer.collect() {
        Ok(tokens) => tokens,
        Err(err) => return println!("{}", err) 
    };
    
    let mut parser = Parser::new(tokens.iter().peekable());
    match parser.parse_expression(Precedence::Lowest) {
        Ok(expr) => println!("{:?}", evaluator.evaluate(expr.unwrap())),
        Err(string) => println!("{}", string)
    }
}

fn main() {
    let mut eval = Evaluator::new();

    for line in stdin().lock().lines() {
        match line {
            Ok(line) => process(line, &mut eval),
            Err(e) => panic!("{}", e),
        }
    }
}
