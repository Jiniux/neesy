
use crate::evaluator::Value;

macro_rules! declare_boolean_op {
    ($name:ident, $op:tt) => { 
        pub fn $name(x : Value, other: Value) -> Result<Value, String> {
            match x {
                Value::Number(n) => {
                    if let Value::Number(y) = other {
                        Ok(Value::Bool(n $op y))
                    } else {
                        return invalid_operands_err!("compare", n, other);
                    }
                },

                Value::Str(s) => {
                    if let Value::Str(y) = other {
                        Ok(Value::Bool(s $op y))
                    } else {
                        return invalid_operands_err!("compare", s, other);
                    }
                }

                _ => unreachable!()
            }
        }
    };
}

declare_boolean_op!(equals, ==);
declare_boolean_op!(less_than_equals, <=);
declare_boolean_op!(greater_than_equals, >=);
declare_boolean_op!(greater_than, >);
declare_boolean_op!(less_than, <);