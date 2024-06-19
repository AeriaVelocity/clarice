// symbol_table.rs - Symbol table for the Clarice programming language

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    Double,
    String,
    Boolean,
    Closure,
    List,
    Void,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol_type: Type) {
        let symbol = Symbol {
            name: name.clone(),
            symbol_type
        };
        self.symbols.insert(name, symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}