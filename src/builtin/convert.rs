use crate::evaluator::Value;

pub fn to_string(values: Vec<Value>) -> Result<Value, String> {
    match &values[0] {
        Value::Number(num) => Ok(Value::Str(format!("{}", *num))),
        Value::Bool(b) => 
            Ok(Value::Str(if *b { "true".to_string() } else { "false".to_string() })),
        
        Value::Str(string) => Ok(Value::Str(string.clone())),

        _ => Err("Bad argument.".to_string())
    }
}

pub fn to_number(values: Vec<Value>) -> Result<Value, String> {
    match &values[0] {
        Value::Str(s) => Ok(
            Value::Number(s.parse::<f64>().unwrap_or(0.0).to_owned())),

        _ => Err("Bad argument.".to_string())
    }
}
