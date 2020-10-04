mod lexer;

use lexer::Token;

use std::iter::Peekable;

enum InfixOperator {

}

enum PrefixOperator {

}

enum Expression {
    Id(String),
    Assignment(Expression),
    Number(f64),
    Str(String),

    Infix(Expression, InfixOperator, Expression),
    Prefix(PrefixOperator, Expression)
}

struct Parser {
    tokens : Peekable<Token>
}

impl Parser {
    pub fn new(tokens : Peekable<Token>) -> Self {
        Parser { tokens: tokens }
    }

    fn parse_infix_expression(current : Token) -> Expression {
        match 
    }

    pub fn parse_expression(self) -> Option<Expression> {
        if let Some(t) = self.tokens.next() {
            match t {
                Token::Number | Token::Str | Token::Id => {
                },

                Token::RParenthesis => Some(self.parse_expression()),
                Token::LParenthesis => None
            }

        None
    }
}