// parser.rs - Parser for the Clarice programming language

use crate::lexer::{Token, Lexer};
use crate::symbol_table::{SymbolTable, Type};
use crate::type_checker::{self, TypeChecker};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<Statement>),
}

#[derive(Debug, Clone)]
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
    Expression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct WithStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct SetStatement {
    pub variable: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct AsStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct ToStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct ThenStatement {
    pub statement: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct DoStatement {
    pub expression: Box<Expression>,
    pub statement: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct WhereStatement {
    pub condition: Box<Expression>,
    pub true_branch: ASTNode,
    pub false_branch: Option<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct LoopStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct IterStatement {
    pub variable: String,
    pub iterable: Box<Expression>,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    BooleanLiteral(bool),
    ListLiteral(Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
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

    fn parse_program(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
        }
        ASTNode::Program(statements)
    }

    fn parse_statement(&mut self) -> Statement {
        match &self.current_token {
            Token::Keyword(ref keyword) => {
                match keyword.as_str() {
                    "with" => Statement::With(self.parse_with_statement()),
                    "set" => Statement::Set(self.parse_set_statement()),
                    "as" => Statement::As(self.parse_as_statement()),
                    "to" => Statement::To(self.parse_to_statement()),
                    "then" => Statement::Then(self.parse_then_statement()),
                    "do" => Statement::Do(self.parse_do_statement()),
                    "print" => Statement::Print(self.parse_print_statement()),
                    "where" => Statement::Where(self.parse_where_statement()),
                    "loop" => Statement::Loop(self.parse_loop_statement()),
                    "iter" => Statement::Iter(self.parse_iter_statement()),
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
            Token::Identifier(_) => {
                // Handle identifiers in specific contexts, if needed
                println!("Expected a statement, got {:?}", self.current_token);
                self.advance();
                Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("Unexpected Identifier.".to_string())),
                }))
            },
            _ => {
                println!("Expected a statement, got {:?}", self.current_token);
                self.advance();
                Statement::Print(Box::new(PrintStatement {
                    expression: Box::new(Expression::StringLiteral("No Statement.".to_string())),
                }))
            }
        }
    }

    fn parse_with_statement(&mut self) -> Box<WithStatement> {
        self.advance(); // Skip "with"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'with' keyword, got {:?}", self.current_token);
                return Box::new(WithStatement {
                    identifier: "error".to_string(),
                    expression: Box::new(Expression::StringLiteral("No Expression (With)".to_string())),
                });
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Box::new(WithStatement {
            identifier,
            expression: Box::new(expression),
        })
    }

    fn parse_set_statement(&mut self) -> Box<SetStatement> {
        self.advance(); // Skip "set"
        let variable = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'set' keyword, got {:?}", self.current_token);
                return Box::new(SetStatement {
                    variable: "error".to_string(),
                    expression: Box::new(Expression::StringLiteral("No Expression (Set)".to_string())),
                });
            }
        };
        self.advance(); // Skip identifier

        match &self.current_token {
            Token::Keyword(ref keyword) if keyword == "to" => self.advance(),
            _ => {
                println!("Expected 'to' after variable name, got {:?}", self.current_token);
                return Box::new(SetStatement {
                    variable: "error".to_string(),
                    expression: Box::new(Expression::StringLiteral("No Expression (Set)".to_string())),
                });
            }
        }

        let expression = self.parse_expression();
        Box::new(SetStatement {
            variable,
            expression: Box::new(expression),
        })
    }

    fn parse_as_statement(&mut self) -> Box<AsStatement> {
        self.advance(); // Skip "as"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'as' keyword, got {:?}", self.current_token);
                return Box::new(AsStatement {
                    identifier: "error".to_string(),
                    expression: Box::new(Expression::StringLiteral("No Expression (As)".to_string())),
                });
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Box::new(AsStatement {
            identifier,
            expression: Box::new(expression),
        })
    }

    fn parse_to_statement(&mut self) -> Box<ToStatement> {
        self.advance(); // Skip "to"
        let identifier = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'to' keyword, got {:?}", self.current_token);
                return Box::new(ToStatement {
                    identifier: "error".to_string(),
                    expression: Box::new(Expression::StringLiteral("No Expression (To)".to_string())),
                });
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Box::new(ToStatement {
            identifier,
            expression: Box::new(expression),
        })
    }

    fn parse_then_statement(&mut self) -> Box<ThenStatement> {
        self.advance(); // Skip "then"
        let statement = self.parse_statement();
        Box::new(ThenStatement {
            statement: Box::new(statement),
        })
    }

    fn parse_do_statement(&mut self) -> Box<DoStatement> {
        self.advance(); // Skip "do"
        let expression = self.parse_expression();
        Box::new(DoStatement {
            expression: Box::new(expression),
            statement: Box::new(self.parse_statement()),
        })
    }

    fn parse_print_statement(&mut self) -> Box<PrintStatement> {
        self.advance(); // Skip "print"
        let expression = self.parse_expression();
        Box::new(PrintStatement {
            expression: Box::new(expression),
        })
    }

    fn parse_where_statement(&mut self) -> Box<WhereStatement> {
        self.advance(); // Skip "where"
        let condition = self.parse_expression();
        let true_branch = self.parse_program();
        let false_branch = if self.current_token == Token::Keyword("otherwise".to_string()) {
            self.advance(); // Skip "otherwise"
            Some(self.parse_program())
        } else {
            None
        };
        Box::new(WhereStatement {
            condition: Box::new(condition),
            true_branch,
            false_branch,
        })
    }

    fn parse_loop_statement(&mut self) -> Box<LoopStatement> {
        self.advance(); // Skip "loop"
        let expression = self.parse_expression();
        Box::new(LoopStatement {
            expression: Box::new(expression),
        })
    }

    fn parse_iter_statement(&mut self) -> Box<IterStatement> {
        self.advance(); // Skip "iter"
        let variable = match self.current_token {
            Token::Identifier(ref id) => id.clone(),
            _ => {
                println!("Expected identifier after 'iter' keyword, got {:?}", self.current_token);
                return Box::new(IterStatement {
                    variable: "error".to_string(),
                    iterable: Box::new(Expression::StringLiteral("No Iterable (Iter)".to_string())),
                    expression: Box::new(Expression::StringLiteral("No Identifier (Iter).".to_string())),
                });
            }
        };
        self.advance(); // Advance to next token
        let expression = self.parse_expression();
        Box::new(IterStatement {
            variable,
            iterable: Box::new(self.parse_expression()),
            expression: Box::new(expression),
        })
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
    
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let program = self.parse_program();
        let mut type_checker = TypeChecker::new();
        type_checker.check(&program)?;
        Ok(program)
    }
}
