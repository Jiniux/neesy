use crate::parser::operators::*;
use crate::parser::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Value {
    Void,
    Number(f64),
    Bool(bool),
    Str(String),
    Function(HashSet<String>, Vec<Expression>)
}

mod basic; 
mod boolean; use boolean::*;

#[derive(Debug)]
pub struct Evaluator<'parent_scope> {
    parent_scope : Option<&'parent_scope Evaluator<'parent_scope>>,
    variables : HashMap<String, Value>
}

impl<'parent_scope> Evaluator<'parent_scope> {
    pub fn new (parent_scope: Option<&'parent_scope Evaluator>) -> Self {
        Evaluator { variables: HashMap::new(), parent_scope }
    }

    fn get_value(&self, name: &String) -> Result<Value, String> {
        Ok(match self.variables.get(name) {
            Some(value) => value.clone(),
            None => 
                if let Some(ps) = self.parent_scope { 
                    ps.get_value(name)?.clone() 
                } 
                else {  
                    return Err(format!("Cannot find variable {}", name))
                }
        })
    }
    
    pub fn evaluate_block(&mut self, stmts : Vec<Expression>) -> Result<Value, String> {
        
        for i in 0..stmts.len()-1 {
            self.evaluate(stmts[i].clone())?;
        }

        return Ok(self.evaluate(stmts[stmts.len()-1].clone())?)
    }

    pub fn evaluate(&mut self, expression : Expression) -> Result<Value, String> {
        match expression {
            Expression::Infix(op, l, r) =>
                match op {
                    InfixOperator::Add => self.evaluate(*l)? + self.evaluate(*r)?,
                    InfixOperator::Sub => self.evaluate(*l)? - self.evaluate(*r)?,
                    InfixOperator::Mul => self.evaluate(*l)? * self.evaluate(*r)?,
                    InfixOperator::Div => self.evaluate(*l)? / self.evaluate(*r)?,
                    InfixOperator::Equals => equals(self.evaluate(*l)?, self.evaluate(*r)?),

                    _ => unreachable!()
                },

            Expression::Prefix(op, l) => 
                match op {
                    PrefixOperator::Positive => self.evaluate(*l),
                    _=> unreachable!()
                },

            Expression::Id(name) => self.get_value(&name),

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

            Expression::If(expr, stmts, else_stmts) => {
                if let Value::Bool(result) = self.evaluate(*expr)? {
                    if result { Ok(self.evaluate_block(stmts)?) } 
                    else { 
                        if else_stmts.is_none() {
                            Ok(Value::Void) 
                        } else {
                            Ok(self.evaluate_block(else_stmts.unwrap())?)
                        }
                    }
                } else {
                    unreachable!()
                }
            },

            Expression::FunctionCall(name, params) => {
                let (t_params, t_exprs) = 
                        match self.get_value(&name)? {
                            Value::Function(params, exprs) => 
                                (params.clone(), exprs.clone()),

                            _ => return Err(format!("{} is not a function", name))
                        };
                
                if params.len() != t_params.len() {
                    return Err(format!("{} requires {} param(s), {} given", name, 
                        t_params.len(), params.len()));
                }

                let mut subeval = Evaluator::new(None);
                for (i, t_param) in t_params.iter().enumerate() {
                    subeval.variables.insert(t_param.clone(), self.evaluate(params[i].clone())?);
                }

                subeval.parent_scope = Some(self);
                
                Ok(subeval.evaluate_block(t_exprs)?)
            },

            _ => unreachable!()
        }
    }
}

// Just a demo.