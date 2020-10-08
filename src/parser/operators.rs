use crate::lexer::Operator;

#[derive(Debug)]
pub enum InfixOperator {
    Add, Sub, Mul, Div
}

impl Operator {
    pub fn to_infix(&self) -> Option<InfixOperator> {
        match self {
            Operator::Add => Some(InfixOperator::Add),
            Operator::Sub => Some(InfixOperator::Sub),
            Operator::Mul => Some(InfixOperator::Mul),
            Operator::Div => Some(InfixOperator::Div),

            _ => None
        }
    }
}

#[derive(Debug)]
pub enum PrefixOperator {
    Positive, Negative, Not
}


impl Operator {
    pub fn to_prefix(&self) -> Option<PrefixOperator> {
        match self {
            Operator::Add => Some(PrefixOperator::Positive),
            Operator::Sub => Some(PrefixOperator::Negative),

            _ => None
        }
    }
}
#[derive(Debug)]
pub enum PostfixOperator {
    FunctionCall
}