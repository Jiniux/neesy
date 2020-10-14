use crate::evaluator::Value;

use std::ops::{Add, Sub, Div, Mul};
impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Value) -> Result<Value, String> {
        match self {
            Value::Number(x) => {
                if let Value::Number(y) = other {
                    Ok(Value::Number(x + y))
                } else {
                    return invalid_operands_err!("sum", x, other);
                }
            },

            Value::Str(x) => {
                if let Value::Str(y) = other {
                    let mut new = String::new();

                    new.push_str(&x);
                    new.push_str(&y);
                    
                    Ok(Value::Str(new))
                } else {
                    return invalid_operands_err!("sum", x, other);
                }
            }

            _ => unreachable!()
        }
    }
}


impl Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, other: Value) -> Result<Value, String> {
        match self {
            Value::Number(x) => {
                if let Value::Number(y) = other {
                    Ok(Value::Number(x - y))
                } else {
                    return invalid_operands_err!("subtract", x, other);
                }
            }

            _ => unreachable!()
        }
    }
}


impl Mul for Value {
    type Output = Result<Value, String>;

    fn mul(self, other: Value) -> Result<Value, String> {
        match self {
            Value::Number(x) => {
                if let Value::Number(y) = other {
                    Ok(Value::Number(x * y))
                } else {
                    return invalid_operands_err!("multiply", x, other);
                }
            },

            Value::Str(x) => {
                if let Value::Number(y ) = other {
                    let mut new = String::new();

                    for _ in 0..(y as usize)  { new.push_str(&x); }

                    Ok(Value::Str(new))
                } else {
                    return invalid_operands_err!("multiply", x, other);
                }
            }

            _ => unreachable!()
        }
    }
}


impl Div for Value {
    type Output = Result<Value, String>;

    fn div(self, other: Value) -> Result<Value, String> {
        match self {
            Value::Number(x) => {
                if let Value::Number(y) = other {
                    Ok(Value::Number(x / y))
                } else {
                    return invalid_operands_err!("divide", x, other);
                }
            }

            _ => unreachable!()
        }
    }
}
