#[derive(Debug, PartialEq)]
pub enum Operator {
    Add, Sub, Mul, Div, Not,

    Equals, 
    
    LessThan, 
    LessThanOrEquals, 

    GreaterThan,
    GreaterThanOrEquals,
}

pub struct Lexer {
    chars: Vec<char>,
    index: usize 
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(f64),
    Str(String),
    Id(String),

    Op(Operator),

    VBar,

    RBrace,
    LBrace,

    RParenthesis,
    LParenthesis,

    RBracket,
    LBracket, Not,

    Comma,

    Assign,
    
    If, 
    Else,

    EOS,
}

fn is_whitespace(c : char) -> bool {
    match c {
        ' ' | '\n' | '\r' | '\t' => true, 
        _ => false
    }
}

fn is_operator(c : char) -> bool {
    match c {
        '+' | '-' | '*' | '=' | '/' | '<' | '>' => true,
        _ => false
    }
}

impl Lexer {
    pub fn new(buf: String) -> Self {
        Lexer { chars: buf.chars().collect(), index : 0 }
    }
    
    fn step(&mut self) {
        self.index += 1;
    }

    fn current(&self) -> Option<char> {
        match self.chars.get(self.index) {
            Some(c) => Some(*c) ,
            None => None
        }
    }

    fn read_literal(&mut self) -> Token {
        let mut literal = String::new();

        while let Some(c) = self.current() {
            match c {
                _ if is_operator(c) || is_whitespace(c) => break,
                '(' | ')' | ',' | '|' | '{' | '}' | '[' | ']' | ';' => break,
                _ => literal.push(c)
            }

            self.step()
        }

        match &*literal {
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Id(literal)
        }
    }

    fn read_operator(&mut self) -> Result<Token, String> {
        let mut operator = String::new();

        while let Some(c) = self.current() {
            if !is_operator(c) { break; }
            operator.push(c);

            self.step()
        }

        match &*operator {
            "+" => Ok(Token::Op(Operator::Add)),
            "-" => Ok(Token::Op(Operator::Sub)),
            "*" => Ok(Token::Op(Operator::Mul)),
            "/" => Ok(Token::Op(Operator::Div)),

            "==" => Ok(Token::Op(Operator::Equals)),
            ">=" => Ok(Token::Op(Operator::GreaterThanOrEquals)),
            "<=" => Ok(Token::Op(Operator::LessThanOrEquals)),
            ">"  => Ok(Token::Op(Operator::GreaterThan)),
            "<"  => Ok(Token::Op(Operator::LessThan)),

            "<-" => Ok(Token::Assign),
            
            _ => Err(format!("Unknown operator: {}", operator))
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number       = 0.0;
        let mut decimal_part = false;
        let mut e : i64      = 0;

        while let Some(c) = self.current() {
            match c {
                '0'..='9' => {
                    let v =  (c as u8 - b'0') as f64;
                    number = (number * 10.0) + v;
                    if decimal_part { e-=1; }
                },
                '.' => {
                    if decimal_part {
                        return Err(format!("Unexpected . at {}", self.index - 1))
                    }

                    decimal_part = true 
                },
                _ => break
            }

            self.step()
        }

        number *= (10.0 as f64).powf(e as f64);
        if number.is_infinite() {
            return Err(format!("Too big number at {}", self.index - 1))
        }
        
        Ok(Token::Num(number))
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current() {
            if !is_whitespace(c) { break; }
            self.step();
        }
    

    fn read_string(&mut self) -> Result<Token, String> {
        self.step();
        let mut string = String::new();
        
        while let Some(c) = self.current() {
            match c {
                '"' => { 
                    self.step(); 
                    return Ok(Token::Str(string)); 
                },
                _ => string.push(c)
            }

            self.step()
        }

        Err(format!("String not terminated at {}", self.index - 1))
    }

    fn parse_token(&mut self) -> Result<Token, String> {
        let c = self.chars[self.index];

        match c {
            '(' => { self.step(); Ok(Token::RParenthesis) },
            ')' => { self.step(); Ok(Token::LParenthesis) },
            '[' => { self.step(); Ok(Token::RBracket) },
            ']' => { self.step(); Ok(Token::LBracket) },
            '{' => { self.step(); Ok(Token::RBrace) },
            '}' => { self.step(); Ok(Token::LBrace) },
            '|' => { self.step(); Ok(Token::VBar) },
            ',' => { self.step(); Ok(Token::Comma) },
            ';' => { self.step(); Ok(Token::EOS) }
            '"' => self.read_string(),
            '0'..='9' | '.' => self.read_number(),
            _ if is_operator(c) => self.read_operator(),
            _ => Ok(self.read_literal())
        }
    }

    pub fn collect(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens : Vec<Token> = vec![]; 

        loop {
            match self.next() {
                Ok(result) => {
                    if let Some(token) = result {
                        tokens.push(token); continue;
                    }

                    break
                }

                Err(err) => return Err(err)
            }
        }
        
        Ok(tokens)
    }

    pub fn next(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespaces();

        match self.chars.get(self.index) {
            Some(_) => match self.parse_token() {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(err)
            },
            None => Ok(None),
        }
    }
}
