pub struct Lexer {
    chars: Vec<char>,
    index: usize 
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Literal(String),
    Str(String),
    If,

    Add, Sub, Mul, Div,
    Assign
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
                _ => literal.push(c)
            }

            self.step()
        }

        match &*literal {
            "if" => Token::If,
            _ => Token::Literal(literal) 
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
            "+" => Ok(Token::Add),
            "-" => Ok(Token::Sub),
            "*" => Ok(Token::Mul),
            "/" => Ok(Token::Div),
            "<-" => Ok(Token::Assign),
            _ => Err(format!("Unknown operator: {}", operator))
        }
    }

    fn read_number(&mut self) -> Token {
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
                '.' => decimal_part = true,
                _ => break
            }

            self.step()
        }

        number *= (10.0 as f64).powf(e as f64);

        Token::Number(number)
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current() {
            if !is_whitespace(c) { break; }
            self.step();
        }
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
            '"' => self.read_string(),
            '0'..='9' | '.' => Ok(self.read_number()),
            _ if is_operator(c) => self.read_operator(),
            _ => Ok(self.read_literal())
        }
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
