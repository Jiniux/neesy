use crate::evaluator::Value;

use std::io::{stdin, Read};

pub mod convert;
pub mod math;

pub fn read_line(values : Vec<Value>) -> Result<Value, String> {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    buf = buf[..buf.len() - 1].to_string();

    Ok(Value::Str(buf))
}

pub fn puts_num(values : Vec<Value>) -> Result<Value, String> {
    if let Value::Number(num) = &values[0] {
        println!("{}", *num);

        Ok(Value::Void)
    } else {
        Err("Invalid type parameter".to_owned())
    }
}

pub fn puts_str(values : Vec<Value>) -> Result<Value, String> {
    if let Value::Str(num) = &values[0] {
        println!("{}", num);

        Ok(Value::Void)
    } else {
        Err("Invalid type parameter".to_owned())
    }
}