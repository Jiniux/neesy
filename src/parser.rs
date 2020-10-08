use crate::lexer::{Operator, Token};

use core::slice::Iter;
use std::iter::Peekable;

mod operators;

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

#[derive(Debug)]
pub enum Expression {
    Id(String),
    Assignment(String, Box<Expression>),
    Num(f64),
    Str(String),

    Infix(InfixOperator, Box<Expression>, Box<Expression>),
    Prefix(PrefixOperator, Box<Expression>),
    Postfix(PostfixOperator, Box<Expression>),
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
        prec : Precedence
    ) -> Result<Expression, String> {
        self.tokens.next();

        match self.parse_expression(prec) {
            Ok(result) => {
                if let Some(rhs) = result {
                    return Ok(Expression::Infix(op, Box::new(lhs), Box::new(rhs)))
                } 
                
                Err(format!("Expected expression"))
            },

            Err(err) => Err(err)
        }
    }

    pub fn parse_expression(&mut self, prec: Precedence) -> Result<Option<Expression>, String> {
        if let Some(token) = self.tokens.next() {
            let lhs_opt = match token {
                Token::Op(op) => {
                    let p_op = op.to_prefix();

                    if p_op.is_none() {
                        return Err(format!("{:?} is not a valid prefix operator", op));
                    }

                    match self.parse_prefix_expression(p_op.unwrap()) {
                        Ok(expr) => Some(expr),
                        Err(err) => return Err(err),
                    }
                }

                Token::Num(num) => Some(Expression::Num(*num)),
                Token::Str(string) => Some(Expression::Str(String::from(string))),
                Token::Id(id) => Some(Expression::Id(String::from(id))),

                Token::RParenthesis => return match self.parse_expression(Precedence::Lowest) {
                    Ok(result) => {
                        match result {
                            Some(expr) => Ok(Some(expr)),
                            None => Ok(None)
                        } 
                    },
                    Err(err) => return Err(err),
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
                        Token::EOS => break,
                        Token::LParenthesis => {
                            self.tokens.next();
                            break;
                        },

                        Token::Op(op) => {
                            let nop = match op.to_infix() {
                                Some(op) => op,
                                None => return Err(format!("{:?} is not a valid infix operator", op))
                            };

                            let nop_prec = nop.precedence();

                            if prec >= nop_prec {
                                break;
                            } 
                            
                            match self.parse_infix_expression(lhs, nop, nop_prec) {
                                Ok(expr) => expr,
                                Err(err) => return Err(err)
                            }
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
