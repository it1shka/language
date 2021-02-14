use crate::parser::ast::*;
use crate::parser::builder::Builder;

use super::object::Object;
use super::storage::MemStack;

pub struct Engine {
    memory: MemStack
}

#[derive(Debug)]
enum Callback {
    Break,
    Continue,
    Return(Object)
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
           memory: MemStack::new() 
        }
    }

    pub fn run(&mut self, ast: &Program) -> Result<(), String> {
        for statement in &ast.0 {
            if let Some(callback) = self.visit_statement(statement)? {
                return Err(
                    format!("Unexpected callback '{:?}' while executing program!", callback, 
                ))
            }
        }
        Ok(())
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<Option<Callback>, String> {
        match statement {
            
        }
    }

}