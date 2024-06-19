// type_checker.rs - Type checker for the Clarice programming language

use crate::parser::{ASTNode, Expression, Statement};
use crate::symbol_table::{SymbolTable, Type};

pub struct TypeChecker {
    symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn check(&mut self, program: &ASTNode) -> Result<(), String> {
        match program {
            ASTNode::Program(statements) => {
                for statement in statements {
                    self.check_statement(statement);
                }
            }
            _ => {
                return Err("Expected program - got something else".to_string());
            }
        }
        Ok(())
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Set(set_statement) => {
                let expression_type = self.check_expression(&set_statement.expression);
                self.symbol_table.insert(set_statement.variable.clone(), expression_type?);
                Ok(())
            }
            Statement::As(as_statement) => {
                let expression_type = self.check_expression(&as_statement.expression);
                self.symbol_table.insert(as_statement.identifier.clone(), expression_type?);
                Ok(())
            }
            Statement::To(to_statement) => {
                let expression_type = self.check_expression(&to_statement.expression);
                self.symbol_table.insert(to_statement.identifier.clone(), expression_type?);
                Ok(())
            }
            Statement::Then(then_statement) => {
                self.check_statement(&then_statement.statement);
                Ok(())
            }
            Statement::Do(do_statement) => {
                self.check_expression(&do_statement.expression);
                Ok(())
            }
            Statement::Print(print_statement) => {
                self.check_expression(&print_statement.expression);
                Ok(())
            }
            Statement::With(with_statement) => {
                self.check_expression(&with_statement.expression);
                Ok(())
            }
            Statement::Where(where_statement) => {
                self.check_expression(&where_statement.condition);
                Ok(())
            }
            Statement::Loop(loop_statement) => {
                self.check_expression(&loop_statement.expression);
                Ok(())
            }
            Statement::Iter(iter_statement) => {
                self.check_expression(&iter_statement.expression);
                Ok(())
            }
            _ => {
                Err((format!("Undefined statement `{:?}`", statement)).to_string())
            }
        }
    }

    fn check_expression(&mut self, expression: &Expression) -> Result<Type, String> {
        match expression {
            Expression::IntegerLiteral(_) => Ok(Type::Integer),
            Expression::StringLiteral(_) => Ok(Type::String),
            Expression::BooleanLiteral(_) => Ok(Type::Boolean),
            Expression::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    Ok(symbol.symbol_type.clone())
                }
                else {
                    Err(format!("Undefined variable `{}`", name))
                }
            }
            _ => Err((format!("Invalid expression `{:?}`", expression)).to_string()),
        }
    }
}