// interpreter.rs - The Clarice Interpreter

use std::collections::HashMap;

use crate::parser::*;
use crate::symbol_table::{SymbolTable, Type};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    List(Vec<Expression>),
    Closure(String, Vec<Expression>),
    Void,
}

pub struct Environment {
    pub variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn interpret(&mut self, program: ASTNode) {
        match program {
            ASTNode::Program(statements) => {
                for statement in statements {
                    self.execute_statement(&statement);
                }
            }
        }
    }

    fn execute_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::With(with_statement) => self.execute_with(with_statement),
            Statement::Set(set_statement) => self.execute_set(set_statement),
            Statement::As(as_statement) => self.execute_as(as_statement),
            Statement::To(to_statement) => self.execute_to(to_statement),
            Statement::Then(then_statement) => self.execute_then(then_statement),
            Statement::Do(do_statement) => self.execute_do(do_statement),
            Statement::Print(print_statement) => self.execute_print(print_statement),
            Statement::Where(where_statement) => self.execute_where(where_statement),
            Statement::Loop(loop_statement) => self.execute_loop(loop_statement),
            Statement::Iter(iter_statement) => self.execute_iter(iter_statement),
            Statement::Expression(expression) => { self.evaluate_expression(expression); },
        }
    }

    /// The `with` statement creates a temporary variable which is dropped
    /// immediately at the end of the succeeding statement (i.e., at the start
    /// of the next `then` or newline).
    /// `with` requires an expression followed by the `as` keyword and a value.
    /// 
    /// Example:
    /// ```clarice
    /// with x as 1 print x + 1
    /// # -> 2
    /// # After that `print` statement is done, `x` is dropped, and it can't be used anymore
    /// ```
    fn execute_with(&mut self, with_statement: &WithStatement) {
        let value = self.evaluate_expression(&with_statement.expression);
        self.variables.insert(with_statement.identifier.clone(), value);
    }

    /// The `set` statement assigns a value to a variable permanently, unlike
    /// `with` which drops the variable immediately.
    /// `set` requires an expression followed by the `to` keyword and a value.
    /// 
    /// Example:
    /// ```clarice
    /// set x to "Hello!" then print x
    /// # -> Hello!
    /// ```
    fn execute_set(&mut self, set_statement: &SetStatement) {
        let value = self.evaluate_expression(&set_statement.expression);
        self.variables.insert(set_statement.variable.clone(), value);
    }

    /// `as` is used only with `with` - it cannot be used on its own.
    fn execute_as(&mut self, _as_statement: &AsStatement) {
        eprintln!("`as` cannot be used on its own - use `with expr as val`")
    }

    /// `to` is used only with `set` - it cannot be used on its own.
    fn execute_to(&mut self, _to_statement: &ToStatement) {
        eprintln!("`to` cannot be used on its own - use `set var to expr`")
    }

    /// The `then` statement is used for separating statements without using
    /// a newline, like the `;` symbol in Python.
    /// 
    /// Example:
    /// ```clarice
    /// set x to "Hello!" then print x
    /// # -> Hello!
    /// ```
    fn execute_then(&mut self, then_statement: &ThenStatement) {
        self.execute_statement(&then_statement.statement);
    }

    /// The `do` statement is used to create a block, like `:` in Python or the
    /// opening brace (`{`) in C.
    /// 
    /// Example:
    /// ```clarice
    /// with x as "indented!" do
    ///     print "This block is " .. x 
    /// # -> This block is indented!
    /// ```
    /// 
    /// TODO: Add support for single-line `do` blocks, with `then`
    fn execute_do(&mut self, do_statement: &DoStatement) {
        self.execute_statement(&do_statement.statement);
    }

    /// The `print` statement prints the value of an expression.
    /// 
    /// Example:
    /// ```clarice
    /// print "Hello, World!"
    /// ```
    fn execute_print(&mut self, _print_statement: &PrintStatement) {
        let value = self.evaluate_expression(&_print_statement.expression);
        match value {
            Value::String(s) => println!("{}", s),
            Value::Integer(i) => println!("{}", i),
            Value::Double(d) => println!("{}", d),
            Value::Boolean(b) => println!("{}", b),
            Value::List(l) => println!("{:?}", l),
            Value::Closure(str, stat) => println!("{:?}, {:?}", str, stat),
            Value::Void => println!(""),
        }
    }

    /// The `where` statement is used to create a condition, like `if` in
    /// most other languages, and `otherwise` is like `else`.
    /// 
    /// Example:
    /// ```clarice
    /// where x > 10 do
    ///     print "x is greater than 10"
    /// otherwise do
    ///     print "x is less than or equal to 10"
    /// ```
    fn execute_where(&mut self, where_statement: &WhereStatement) {
        let value = self.evaluate_expression(&where_statement.condition);
        if let Value::Boolean(true) = value {
            self.interpret(where_statement.true_branch.clone());
        }
        else if let Some(false_branch) = &where_statement.false_branch {
            self.interpret(false_branch.clone());
        }
    }

    /// The `loop` statement creates an infinite loop.
    /// 
    /// Example:
    /// ```clarice
    /// loop do
    ///     print "Hello, World!"
    /// # -> Hello, World!
    /// ```
    fn execute_loop(&mut self, loop_statement: &LoopStatement) {
        loop {
            self.evaluate_expression(&loop_statement.expression.clone());
        }
    }

    /// The `iter` statement is like `for` loops in Python and Rust.
    /// 
    /// Example:
    /// ```clarice
    /// iter x in "Hello, World!" do
    ///     print x .. " "
    /// # -> H e l l o   ,   W o r l d !
    /// ```
    fn execute_iter(&mut self, iter_statement: &IterStatement) {
        let iterable = self.evaluate_expression(&iter_statement.iterable);
        match iterable {
            Value::String(s) => {
                for c in s.chars() {
                    self.variables.insert(iter_statement.variable.clone(), Value::String(c.to_string()));
                    self.evaluate_expression(&iter_statement.expression.clone());
                }
            }
            Value::Integer(i) => {
                for _ in 0..i {
                    self.evaluate_expression(&iter_statement.expression.clone());
                }
            }
            Value::List(l) => {
                for v in l {
                    let value = self.evaluate_expression(&v);
                    self.variables.insert(iter_statement.variable.clone(), value);
                    self.evaluate_expression(&iter_statement.expression.clone());
                }
            }
            _ => eprintln!("Cannot iterate over {:?}", iterable),
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Identifier(id) => {
                if let Some(value) = self.variables.get(id) {
                    return value.clone();
                } else {
                    eprintln!("No variable `{}` - use `with` or `set` to define it", id);
                    return Value::Void;
                }
            }
            Expression::IntegerLiteral(i) => Value::Integer(*i),
            Expression::DoubleLiteral(d) => Value::Double(*d),
            Expression::BooleanLiteral(b) => Value::Boolean(*b),
            Expression::StringLiteral(s) => Value::String(s.clone()),
            Expression::ListLiteral(l) => Value::List(l.clone()),
            Expression::FunctionCall(n, f) => Value::Closure(n.clone(), f.clone()),
        }
    }
}