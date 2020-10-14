use crate::parser::operators::*;
use crate::parser::*;

use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap};

use std::rc::Rc;

#[macro_use] mod errors;

#[derive(Debug, Clone)]
pub enum Value {
    Void,
    Number(f64),
    Bool(bool),
    Str(String),
    Function(Rc<LinkedHashSet<String>>, Rc<Vec<Expression>>),
    BuiltinFunction(usize, fn(Vec<Value>) -> Result<Value, String>),
}

mod basic;

mod boolean;
use boolean::*;

#[derive(Debug)]
pub struct Evaluator<'parent_scope> {
    parent_scope: Option<&'parent_scope Evaluator<'parent_scope>>,
    builtin_functions: &'parent_scope HashMap<String, Value>,
    variables: HashMap<String, Value>,
}

impl<'parent_scope, 'a> Evaluator<'parent_scope> {
    pub fn new(
        parent_scope: Option<&'parent_scope Evaluator>,
        builtin_functions: &'parent_scope HashMap<String, Value>,
    ) -> Self {
        Evaluator {
            variables: HashMap::new(),
            parent_scope,
            builtin_functions,
        }
    }

    fn get_value(&self, name: &String) -> Result<&Value, String> {
        if let Some(builtin) = self.builtin_functions.get(name) {
            return Ok(builtin);
        } 
     
        Ok(match self.variables.get(name) {
            Some(value) => value,
            None => {
                if let Some(ps) = self.parent_scope {
                    ps.get_value(name)?
                } else {
                    return cannot_find_var_err!(name);
                }
            }
        })
    }

    pub fn evaluate_block(&mut self, stmts: &'a Vec<Expression>) -> Result<Value, String> {
        if stmts.len() == 0 {
            return Ok(Value::Void);
        }

        for i in 0..stmts.len() - 1 {
            self.evaluate(&stmts[i])?;
        }

        return Ok(self.evaluate(&stmts[stmts.len() - 1])?);
    }

    pub fn evaluate(&mut self, expression: &'a Expression) -> Result<Value, String> {
        match expression {
            Expression::Infix(op, l, r) => match op {
                InfixOperator::Add => self.evaluate(*(&l))? + self.evaluate(*(&r))?,
                InfixOperator::Sub => self.evaluate(*(&l))? - self.evaluate(*(&r))?,
                InfixOperator::Mul => self.evaluate(*(&l))? * self.evaluate(*(&r))?,
                InfixOperator::Div => self.evaluate(*(&l))? / self.evaluate(*(&r))?,
                
                InfixOperator::Equals => equals(self.evaluate(*(&l))?, self.evaluate(*(&r))?),
                InfixOperator::GreaterThanOrEquals => { 
                    greater_than_equals(self.evaluate(*(&l))?, self.evaluate(*(&r))?)
                },
                InfixOperator::LessThanOrEquals => {
                    less_than_equals(self.evaluate(*(&l))?, self.evaluate(*(&r))?)
                },
                InfixOperator::LessThan => {
                    less_than(self.evaluate(*(&l))?, self.evaluate(*(&r))?)
                },
                InfixOperator::GreaterThan => {
                    greater_than(self.evaluate(*(&l))?, self.evaluate(*(&r))?)
                },
            },

            Expression::Prefix(op, l) => match op {
                PrefixOperator::Positive => self.evaluate(*(&l)),
                _ => unreachable!(),
            },

            Expression::Id(name) => Ok(self.get_value(&name)?.clone()),

            Expression::Assignment(name, expr) => {
                let value = self.evaluate(*(&expr))?;

                match value {
                    Value::Void => cannot_assign_void_to_var_err!(name),

                    _ => {
                        if !self.builtin_functions.get(name).is_none() {
                            return cannot_assign_to_builtin_err!(name)
                        }

                        self.variables.insert(name.clone(), value.clone());
                        Ok(value)
                    }
                }
            }

            Expression::Num(n) => Ok(Value::Number(n.clone())),
            Expression::Str(string) => Ok(Value::Str(string.clone())),
            Expression::Function(params, smts) => Ok(Value::Function(
                Rc::new(params.clone()),
                Rc::new(smts.clone()),
            )),

            Expression::If(expr, stmts, else_stmts) => {
                if let Value::Bool(result) = self.evaluate(*(&expr))? {
                    if result {
                        Ok(self.evaluate_block(stmts)?)
                    } else {
                        let else_stmts_ref = else_stmts.as_ref();

                        if else_stmts.is_none() {
                            Ok(Value::Void)
                        } else {
                            Ok(self.evaluate_block(else_stmts_ref.unwrap())?)
                        }
                    }
                } else {
                    unreachable!()
                }
            },


            Expression::While(expr, stmts) => {
                if let Value::Bool(result) = self.evaluate(*(&expr))? {
                    if !result {
                        return Ok(Value::Void)
                    } 
                    
                    loop {
                        self.evaluate_block(stmts)?;

                        match self.evaluate(*(&expr))? {
                            Value::Bool(v) => if !v { break; },

                            _ => unreachable!()
                        }
                    }

                    Ok(Value::Void)
                } else {
                    unreachable!()
                }
            }

            Expression::Void => Ok(Value::Void),
            Expression::Bool(val) => Ok(Value::Bool(val.clone())),

            Expression::FunctionCall(name, params) => {
                match self.get_value(&name)?.clone() {
                    Value::BuiltinFunction(param_count, func) => {
                        let mut resolved_params: Vec<Value> = vec![]; 
                        
                        if params.len() < param_count {
                            return not_enough_params_err!(name, param_count, params);
                        }

                        for param in params {
                            resolved_params.push(self.evaluate(param)?);
                        }
                        
                        return func(resolved_params);
                    },

                    Value::Function(t_params, t_stmts) => {
                        let param_count = t_params.len();
                        
                        if params.len() != param_count {
                            return not_enough_params_err!(name, param_count, params);
                        }
        
                        let mut subeval = Evaluator::new(self.parent_scope, self.builtin_functions);
                        for (i, t_param) in t_params.iter().enumerate() {
                            subeval
                                .variables
                                .insert(t_param.clone(), self.evaluate(&params[i])?);
                        }
        
                        if self.parent_scope.is_none() {
                            subeval.parent_scope = Some(self);
                        } else {
                            subeval.parent_scope = Some(self.parent_scope.unwrap());
                        }
        
                        return subeval.evaluate_block(&t_stmts)
                    },

                    _ => return not_a_function_err!(name),
                };

                
            }

            _ => unreachable!(),
        }
    }
}