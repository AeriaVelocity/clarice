// lexer.rs - Lexer for the Clarice programming language

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String), // "with", "as", "to", "set", "where", "otherwise", "iter", etc.
    Identifier(String), // "x", "y", "z", "tomato", "celery", "avocado", etc.
    IntegerLiteral(i64), // 1, 2, 3
    StringLiteral(String), // "hello, world!"
    Operator(String), // "+", "-", "*", "/", etc.
    Separator(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.chars().nth(self.position);
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            }
            else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            
            if c.is_digit(10) {
                return self.tokenize_integer();
            }
            else if c.is_alphabetic() {
                return self.tokenize_identifier_or_keyword();
            }
            else {
                let token = match c {
                    '+' | '-' | '*' | '/' | '=' => {
                        self.advance();
                        Token::Operator(c.to_string())
                    },
                    '(' | ')' | '{' | '}' | '[' | ']' | ':' | ';' | ',' | '.' => {
                        self.advance();
                        Token::Separator(c.to_string())
                    },
                    '"' => self.tokenize_string_literal(),
                    _ => {
                        self.advance();
                        continue;
                    }
                };
                return token;
            }

        }
        Token::EOF
    }

    fn tokenize_integer(&mut self) -> Token {
        let mut num_str = String::new();
        
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                num_str.push(c);
                self.advance();
            }
            else {
                break;
            }
        }

        let num: i64 = num_str.parse().unwrap_or(0);
        Token::IntegerLiteral(num)
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            }
            else {
                break;
            }
        }
        
        match identifier.as_str() {
            "with" | "set" | "as" | "to" | "then" | "do" | "print" | "where" | "loop" | "iter" => {
                Token::Keyword(identifier)
            },
            _ => Token::Identifier(identifier),
        }
    }

    fn tokenize_string_literal(&mut self) -> Token {
        let mut string_literal = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            }
            else {
                string_literal.push(c);
                self.advance();
            }
        }
        Token::StringLiteral(string_literal)
    }
}

pub fn test(input: &str) {
    let mut lexer = Lexer::new(input);
    let mut token = lexer.get_next_token();
    while token != Token::EOF {
        println!("{:?}", token);
        token = lexer.get_next_token();
    }
}