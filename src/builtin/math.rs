use crate::evaluator::Value;

pub fn pow(values: Vec<Value>) -> Result<Value, String> {
    let x : f64;
    let y : f64;

    if let Value::Number(num) = &values[0] {
        x = *num;
    } else {
        return Err("Invalid type parameter".to_owned());
    }

    if let Value::Number(num) = &values[1] {
        y = *num;
    } else {
        return Err("Invalid type parameter".to_owned());
    }

    Ok(Value::Number(x.powf(y)))
}
