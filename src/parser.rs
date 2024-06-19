// parser.rs - Parser for the Clarice programming language

use crate::lexer::{Token, Lexer};
use std::vec::Vec;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<Statement>),
}

#[derive(Debug)]
pub enum Statement {
    With(Box<WithStatement>),
    Set(Box<SetStatement>),
    As(Box<AsStatement>),
    To(Box<ToStatement>),
    Then(Box<ThenStatement>),
    Do(Box<DoStatement>),
    Print(Box<PrintStatement>),
    Where(Box<WhereStatement>),
    Loop(Box<LoopStatement>),
    Iter(Box<IterStatement>),
}

#[derive(Debug)]
pub struct WithStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct SetStatement {
    pub variable: String,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct AsStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct ToStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct ThenStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct DoStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct WhereStatement {
    pub condition: Box<Expression>,
    pub true_branch: Vec<Statement>,
    pub false_branch: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct LoopStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct IterStatement {
    pub variable: String,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.get_next_token();
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
        }
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token {
            Token::Keyword(ref keyword) => {
                match keyword.as_str() {
                    "with" => self.parse_with_statement(),
                    "set" => self.parse_set_statement(),
                    "as" => self.parse_as_statement(),
                    "to" => self.parse_to_statement(),
                    "then" => self.parse_then_statement(),
                    "do" => self.parse_do_statement(),
                    "print" => self.parse_print_statement(),
                    "where" => self.parse_where_statement(),
                    "loop" => self.parse_loop_statement(),
                    "iter" => self.parse_iter_statement(),
                    _ => {
                        println!("Clarice doesn't recognize the keyword \"{}\".", keyword);
                        self.advance();
                        // Placeholder error handling
                        Statement::Print(Box::new(PrintStatement {
                            expression: Box::new(Expression::StringLiteral("Unknown Keyword.".to_string())),
                        }))
                    }
                }
            },
            _ => {
                println!("Expected a statement, got {:?}", self.current_token);
                self.advance();
                // Placeholder error handling
                Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Statement.".to_string())),
                }))
            }
        }
    }

    fn parse_with_statement(&mut self) -> Statement {
        self.advance(); // Skip "with"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'with' keyword, got {:?}", self.current_token);
                return Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Identifier (With).".to_string())),
                }));
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Statement::With(Box::new(WithStatement {
            identifier,
            expression: Box::new(expression),
        }))
    }

    fn parse_set_statement(&mut self) -> Statement {
        self.advance(); // Skip "set"
        let variable = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'set' keyword, got {:?}", self.current_token);
                return Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Identifier (Set).".to_string())),
                }));
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Statement::Set(Box::new(SetStatement {
            variable,
            expression: Box::new(expression),
        }))
    }

    fn parse_as_statement(&mut self) -> Statement {
        self.advance(); // Skip "as"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'as' keyword, got {:?}", self.current_token);
                return Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Identifier (As).".to_string())),
                }));
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Statement::As(Box::new(AsStatement {
            identifier,
            expression: Box::new(expression),
        }))
    }

    fn parse_to_statement(&mut self) -> Statement {
        self.advance(); // Skip "to"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'to' keyword, got {:?}", self.current_token);
                return Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Identifier (To).".to_string())),
                }));
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Statement::To(Box::new(ToStatement {
            identifier,
            expression: Box::new(expression),
        }))
    }

    fn parse_then_statement(&mut self) -> Statement {
        self.advance(); // Skip "then"
        let expression = self.parse_expression();
        Statement::Then(Box::new(ThenStatement {
            expression: Box::new(expression),
        }))
    }

    fn parse_do_statement(&mut self) -> Statement {
        self.advance(); // Skip "do"
        let expression = self.parse_expression();
        Statement::Do(Box::new(DoStatement {
            expression: Box::new(expression),
        }))
    }

    fn parse_print_statement(&mut self) -> Statement {
        self.advance(); // Skip "print"
        let expression = self.parse_expression();
        Statement::Print(Box::new(PrintStatement {
            expression: Box::new(expression),
        }))
    }

    fn parse_where_statement(&mut self) -> Statement {
        self.advance(); // Skip "where"
        let condition = self.parse_expression();
        let true_branch = self.parse_program();
        let false_branch = if self.current_token == Token::Keyword("otherwise".to_string()) {
            self.advance(); // Skip "otherwise"
            Some(self.parse_program())
        } else {
            None
        };
        Statement::Where(Box::new(WhereStatement {
            condition: Box::new(condition),
            true_branch,
            false_branch,
        }))
    }

    fn parse_loop_statement(&mut self) -> Statement {
        self.advance(); // Skip "loop"
        let expression = self.parse_expression();
        Statement::Loop(Box::new(LoopStatement {
            expression: Box::new(expression),
        }))
    }

    fn parse_iter_statement(&mut self) -> Statement {
        self.advance(); // Skip "iter"
        let variable = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'iter' keyword, got {:?}", self.current_token);
                return Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Identifier (Iter).".to_string())),
                }));
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Statement::Iter(Box::new(IterStatement {
            variable,
            expression: Box::new(expression),
        }))
    }

    fn parse_expression(&mut self) -> Expression {
        let token = self.current_token.clone();
        match token {
            Token::Identifier(ref id) => {
                self.advance(); // Advance past the identifier
                Expression::Identifier(id.clone())
            },
            Token::IntegerLiteral(num) => {
                self.advance(); // Advance past the integer literal
                Expression::IntegerLiteral(num)
            },
            Token::StringLiteral(ref s) => {
                self.advance(); // Advance past the string literal
                Expression::StringLiteral(s.clone())
            },
            _ => {
                println!("Expected an expression, got {:?}", self.current_token);
                self.advance();
                Expression::StringLiteral("No Expression.".to_string())
            }
        }
    }
    
    pub fn parse(&mut self) -> ASTNode {
        let program = self.parse_program();
        ASTNode::Program(program)
    }
}
