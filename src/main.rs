mod lexer;
mod parser;
mod builtin;
mod evaluator;

use parser::Parser;
use parser::Precedence;

use evaluator::Evaluator;

use lexer::Lexer;

fn process(buf : String, evaluator: &mut Evaluator) -> Result<(), String> {
    let mut lexer = Lexer::new(buf);

    let tokens = match lexer.collect() {
        Ok(tokens) => tokens,
        Err(err) => { return Err(err) }
    };
    
    let mut parser = Parser::new(tokens.iter().peekable());
    
    loop {
        match parser.parse_expression(Precedence::Lowest) {
            Ok(result) => { 
                if let Some(expr) = result {
                    evaluator.evaluate(&expr)?;
                } 
                else {
                    break;
                }
            },

            Err(string) => return Err(string)
        }
    };

    Ok(())
}

use std::collections::HashMap;

use evaluator::Value;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut builtin_functions: HashMap<String, Value> = HashMap::new();

    builtin_functions.insert(format!("puts_num"), Value::BuiltinFunction(1, builtin::puts_num));
    builtin_functions.insert(format!("puts_str"), Value::BuiltinFunction(1, builtin::puts_str));
    builtin_functions.insert(format!("read_line"), Value::BuiltinFunction(0, builtin::read_line));
    builtin_functions.insert(format!("pow"), Value::BuiltinFunction(2, builtin::math::pow));

    builtin_functions.insert(format!("to_string"), Value::BuiltinFunction(1, builtin::convert::to_string));
    builtin_functions.insert(format!("to_number"), Value::BuiltinFunction(1, builtin::convert::to_number));

    let mut eval = Evaluator::new(None, &builtin_functions);

    match std::env::args().skip(1).next() {
        Some(arg) => {
            let mut buf = String::new();

            let mut file = File::open(arg).unwrap();
            file.read_to_string(&mut buf).unwrap();

            match process(buf, &mut eval) {
                Ok(()) => {},

                Err(err) => println!("{}", err)
            }
        },

        None => println!("Usage: neesy <source.nsy>")
    }
}
            