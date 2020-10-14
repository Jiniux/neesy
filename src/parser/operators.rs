use crate::lexer::Operator;

#[derive(Debug, Clone)]
pub enum InfixOperator {
    Add, Sub, Mul, Div, 
    
    Equals, 
    
    LessThan, 
    LessThanOrEquals, 

    GreaterThan,
    GreaterThanOrEquals,
}

impl Operator {
    pub fn to_infix(&self) -> Option<InfixOperator> {
        match self {
            Operator::Add => Some(InfixOperator::Add),
            Operator::Sub => Some(InfixOperator::Sub),
            Operator::Mul => Some(InfixOperator::Mul),
            Operator::Div => Some(InfixOperator::Div),
            
            Operator::Equals => Some(InfixOperator::Equals),
            Operator::GreaterThan => Some(InfixOperator::GreaterThan),
            Operator::GreaterThanOrEquals => Some(InfixOperator::GreaterThanOrEquals),
            Operator::LessThan => Some(InfixOperator::LessThan),
            Operator::LessThanOrEquals => Some(InfixOperator::LessThanOrEquals),

            _ => None
        }
    }
}

#[derive(Debug, Clone)]
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
