use std::io::stdin;
use std::io::BufRead;


mod lexer;
mod parser;
mod builtin;
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
        Ok(expr) => { 
            let test = expr.as_ref();
            println!("{:?}", evaluator.evaluate(test.unwrap())) 
        },
        Err(string) => println!("{}", string)
    }
}

use std::collections::HashMap;

use evaluator::Value;

fn main() {
    let mut builtin_functions: HashMap<String, Value> = HashMap::new();

    builtin_functions.insert(format!("puts_num"), Value::BuiltinFunction(1, builtin::puts_num));
    builtin_functions.insert(format!("puts_str"), Value::BuiltinFunction(1, builtin::puts_str));
    builtin_functions.insert(format!("pow"), Value::BuiltinFunction(2, builtin::math::pow));

    let mut eval = Evaluator::new(None, &builtin_functions);

    for line in stdin().lock().lines() {
        match line {
            Ok(line) => process(line, &mut eval),
            Err(e) => panic!("{}", e),
        }
    }
}
            