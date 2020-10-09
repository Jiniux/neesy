use crate::lexer::{Operator, Token};

use core::slice::Iter;

use std::collections::HashSet;
use std::iter::Peekable;

pub mod operators;

use operators::*;
#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum Precedence {
    Lowest,
    Add,
    Mul,
    Prefix,
    Call,
}

impl InfixOperator {
    fn precedence(&self) -> Precedence {
        match self {
            InfixOperator::Add | InfixOperator::Sub => Precedence::Add,
            InfixOperator::Mul | InfixOperator::Div => Precedence::Mul,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Id(String),
    Assignment(String, Box<Expression>),
    Num(f64),
    Str(String),

    Function(HashSet<String>, Vec<Expression>),
    FunctionCall(String, Vec<Expression>),

    Infix(InfixOperator, Box<Expression>, Box<Expression>),
    Prefix(PrefixOperator, Box<Expression>),
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Peekable<Iter<'a, Token>>) -> Self {
        Parser { tokens }
    }

    fn parse_prefix_expression(&mut self, op: PrefixOperator) -> Result<Expression, String> {
        match self.parse_expression(Precedence::Prefix) {
            Ok(opt) => match opt {
                Some(expr) => Ok(Expression::Prefix(op, Box::new(expr))),
                None => Err(format!(
                    "Expected expression after prefix operator {:?}",
                    op
                )),
            },

            Err(err) => Err(err),
        }
    }

    fn parse_infix_expression(
        &mut self,
        lhs: Expression,
        op: InfixOperator,
        prec: Precedence,
    ) -> Result<Expression, String> {
        self.tokens.next();

        match self.parse_expression(prec) {
            Ok(result) => {
                if let Some(rhs) = result {
                    return Ok(Expression::Infix(op, Box::new(lhs), Box::new(rhs)));
                }

                Err(format!("Expected expression"))
            }

            Err(err) => Err(err),
        }
    }

    fn parse_assign(&mut self, lhs: Expression, prec: Precedence) -> Result<Expression, String> {
        if let Expression::Id(id) = lhs {
            self.tokens.next();
            Ok(Expression::Assignment(
                id,
                Box::new(match self.parse_expression(prec)? {
                    Some(expr) => expr,
                    None => return Err(format!("Expected expression")),
                }),
            ))
        } else {
            return Err(format!("{:?} is not an identifier", lhs));
        }
    }

    fn expect_next(&mut self, token: Token) -> Result<(), String> {
        match self.tokens.peek() {
            Some(next_token) => {
                if token == **next_token {
                    Ok(())
                } else {
                    Err(format!("Expected {:?}, got {:?}", token, next_token))
                }
            }

            None => Err(format!("Expected {:?}, got nothing", token)),
        }
    }

    fn parse_function_call(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.next() {
            let f_name = match token {
                Token::Id(id) => String::from(id),

                _ => return Err(format!("Expected identifier, got {:?}", token))
            };

            let mut params : Vec<Expression> = vec![];  

            loop {
                if let Some(next_token) = self.tokens.peek() {
                    match next_token {
                        Token::LBracket => { self.tokens.next(); break } , 
                        Token::EOS => { self.tokens.next(); },  
                        _ => {}
                    }
                } else {
                    return Err(format!("Expected LBrace, got nothing"));
                }
    
                match self.parse_expression(Precedence::Lowest)? {
                    Some(expr) => params.push(expr),
                    None => break,
                }
            }
            
            return Ok(Expression::FunctionCall(f_name, params))
        }
        
        Err("Expected function name, got nothing.".to_owned())
    }

    fn parse_function(&mut self) -> Result<Expression, String> {
        
        // Parse arguments
        let mut parameters: HashSet<String> = HashSet::new();

        loop {
            let next_token= self.tokens.next() ;
            
            if next_token.is_none() {
                return Err(format!("Expected function arguments."))
            }

            parameters.insert(match next_token.unwrap() {
                Token::Id(id) => { 
                    let n_id = String::from(id);
                    
                    if parameters.contains(&n_id) {
                        return Err(format!("Parameter {} was already specified", n_id));
                    }

                    n_id
                },

                _ => return Err(format!("Expected parameter name.")),
            });

            if let Some(token) = self.tokens.next() {
                match token {
                    Token::Comma => {
                        continue;
                    }
                    Token::VBar => {
                        break;
                    }

                    _ => return Err(format!("Unexpected {:?} in parameter list", token)),
                }
            } else {
                return Err(format!("Expected , or |, got nothing"));
            }
        }

        // Parse body

        self.expect_next(Token::RBrace)?;
        self.tokens.next();

        let mut expressions : Vec<Expression> = vec![];

        loop {
            if let Some(next_token) = self.tokens.peek() {
                match next_token {
                    Token::LBrace => { self.tokens.next(); break } , 
                    Token::EOS => { self.tokens.next(); }
                    _ => {}
                }
            } else {
                return Err(format!("Expected LBrace, got nothing"));
            }

            match self.parse_expression(Precedence::Lowest)? {
                Some(expr) => expressions.push(expr),
                None => break,
            }
        }

        Ok(Expression::Function(parameters, expressions))
    }

    pub fn parse_expression(&mut self, prec: Precedence) -> Result<Option<Expression>, String> {
        if let Some(token) = self.tokens.next() {
            let lhs_opt = match token {
                Token::Op(op) => {
                    let p_op = op.to_prefix();

                    if p_op.is_none() {
                        return Err(format!("{:?} is not a valid prefix operator", op));
                    }

                    Some(self.parse_prefix_expression(p_op.unwrap())?)
                }

                Token::VBar => Some(self.parse_function()?),
                Token::RBracket => Some(self.parse_function_call()?),

                Token::Num(num) => Some(Expression::Num(*num)),
                Token::Str(string) => Some(Expression::Str(String::from(string))),
                Token::Id(id) => Some(Expression::Id(String::from(id))),

                Token::RParenthesis => {
                    return match self.parse_expression(Precedence::Lowest)? {
                        Some(expr) => Ok(Some(expr)),
                        None => Ok(None),
                    }
                },

                Token::EOS => return Ok(None),

                _ => return Err(format!("Expected expression, got {:?}", token)),
            };

            if lhs_opt.is_none() {
                return Ok(None);
            }

            let mut lhs = lhs_opt.unwrap();

            loop {
                if let Some(next_token) = self.tokens.peek() {
                    lhs = match next_token {
                        Token::EOS | Token::LBrace | Token::LBracket => {
                            break;
                        },

                        Token::LParenthesis => {
                            self.tokens.next();
                            break;
                        }

                        // This is ugly.
                        Token::Assign => self.parse_assign(lhs, prec)?,

                        Token::Op(op) => {
                            let nop = match op.to_infix() {
                                Some(op) => op,
                                None => {
                                    return Err(format!("{:?} is not a valid infix operator", op))
                                }
                            };

                            let nop_prec = nop.precedence();

                            if prec >= nop_prec {
                                break;
                            }

                            self.parse_infix_expression(lhs, nop, nop_prec)?
                        }

                        _ => return Err(format!("Expected operator, got {:?}", next_token)),
                    }
                } else {
                    break;
                }
            }

            return Ok(Some(lhs));
        }

        Ok(None)
    }
}
