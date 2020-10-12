
use crate::evaluator::Value;

pub fn equals(x : Value, other: Value) -> Result<Value, String> {
    match x {
        Value::Number(n) => {
            if let Value::Number(y) = other {
                Ok(Value::Bool(n == y))
            } else {
                Err(format!("Can't compare"))
            }
        },

        Value::Str(s) => {
            if let Value::Str(y) = other {
                Ok(Value::Bool(s == y))
            } else {
                Err(format!("Can't compare"))
            }
        }

        _ => unreachable!()
    }
}


pub fn less_than_equals(x : Value, other: Value) -> Result<Value, String> {
    match x {
        Value::Number(n) => {
            if let Value::Number(y) = other {
                Ok(Value::Bool(n <= y))
            } else {
                Err(format!("Can't compare"))
            }
        },

        Value::Str(s) => {
            if let Value::Str(y) = other {
                Ok(Value::Bool(s <= y))
            } else {
                Err(format!("Can't compare"))
            }
        }

        _ => unreachable!()
    }
}
