use crate::evaluator::Value;

pub mod math;

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