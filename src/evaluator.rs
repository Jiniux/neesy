use crate::parser::operators::*;
use crate::parser::*;

use std::collections::HashSet;

use std::ops::{Add, Sub, Div, Mul, Neg};
use std::rc::Rc;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
    Function(HashSet<String>, Vec<Expression>)
}

impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, other: Value) -> Result<Value, String> {
        match self {
            Value::Number(x) => {
                if let Value::Number(y) = other {
                    Ok(Value::Number(x + y))
                } else {
                    Err(format!("Cannot sum with {:?}", other))
                }
            },

            Value::Str(x) => {
                if let Value::Str(y) = other {
                    let mut  new = String::new();

                    new.push_str(&x);
                    new.push_str(&y);
                    
                    Ok(Value::Str(new))
                } else {
                    Err(format!("Cannot sum with {:?}", other))
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
                    Err(format!("Cannot sum with {:?}", other))
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
                    Err(format!("Cannot sum with {:?}", other))
                }
            },

            Value::Str(x) => {
                if let Value::Number(y ) = other {
                    let mut new = String::new();

                    for _ in 0..(y as usize)  { new.push_str(&x); }

                    Ok(Value::Str(new))
                } else {
                    Err(format!("Cannot sum with {:?}", other))
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
                    Err(format!("Cannot sum with {:?}", other))
                }
            }

            _ => unreachable!()
        }
    }
}



#[derive(Debug)]
pub struct Evaluator {
    variables : HashMap<String, Value>
}

impl Evaluator {
    pub fn new () -> Self {
        Evaluator { variables: HashMap::new() }
    }

    pub fn evaluate(&mut self, expression : Expression) -> Result<Value, String> {
        match expression {
            Expression::Infix(op, l, r) =>
                match op {
                    InfixOperator::Add => self.evaluate(*l)? + self.evaluate(*r)?,
                    InfixOperator::Sub => self.evaluate(*l)? - self.evaluate(*r)?,
                    InfixOperator::Mul => self.evaluate(*l)? * self.evaluate(*r)?,
                    InfixOperator::Div => self.evaluate(*l)? / self.evaluate(*r)?,
                },

            Expression::Prefix(op, l) => 
                match op {
                    PrefixOperator::Positive => self.evaluate(*l),
                    _=> unreachable!()
                },

            Expression::Id(name) => {
                match self.variables.get(&name) {
                    Some(value) => Ok(value.clone()),
                    None => return Err(format!("Cannot find variable {:?}", name))
                }
            },

            Expression::Assignment(name, expr) => {
                let value = self.evaluate(*expr)?;

                self.variables.insert(name, value.clone());
                Ok(value)
            },
            
            Expression::Num(n) => Ok(Value::Number(n)),
            Expression::Str(string) => Ok(Value::Str(string)),
            Expression::Function(params,smts) => {
                Ok(Value::Function(params, smts))
            },

            Expression::FunctionCall(name, params) => {
                let (t_params, t_exprs) = 
                    if let Some(var) = self.variables.get(&name) {
                        match var {
                            Value::Function(params, exprs) => 
                                (params.clone(), exprs.clone()),

                            _ => return Err(format!("{} is not a function", name))
                        }
                    } else { 
                        return Err("No".to_owned()) 
                    };
                
                if params.len() != t_params.len() {
                    return Err(format!("{} requires {} params, {} given", name, 
                        t_params.len(), params.len()));
                }

                let mut subeval = Evaluator::new();
                for (i, t_param) in t_params.iter().enumerate() {
                    subeval.variables.insert(t_param.clone(), self.evaluate(params[i].clone())?);
                }

                for i in 0..t_exprs.len()-1 {
                    subeval.evaluate(t_exprs[i].clone())?;
                }

                return Ok(subeval.evaluate(t_exprs[t_exprs.len()-1].clone())?)
            },

            _ => unreachable!()
        }
    }
}

// Just a demo.