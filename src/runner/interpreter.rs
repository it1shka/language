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

fn echo(object: Object) {
    println!("{:?}", object)
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
           memory: MemStack::new() 
        }
    }

    pub fn run(&mut self, ast: &Program) -> Result<(), String> {
        let statements = &ast.0;
        let maybe_callback = self.visit_statement_list(statements)?;
        if let Some(Callback::Return(object)) = maybe_callback {
            echo(object);
        }
        Ok(())
    }

    fn visit_statement_list(&mut self, statements: &Vec<Statement>) -> 
    Result<Option<Callback>, String> {
        for statement in statements {
            let maybe_callback = self.visit_statement(statement)?;
            if let Some(_) = maybe_callback {
                return Ok(maybe_callback);
            }
        }
        Ok(None)
    }

    fn visit_statement(&mut self, statement: &Statement) ->
    Result<Option<Callback>, String> {
        match statement {
            Statement::List(stmts) => self.visit_statement_list(stmts),
            Statement::Break => Ok(Some(Callback::Break)),
            Statement::Continue => Ok(Some(Callback::Continue)),
            Statement::Return(expression) => 
                Ok(Some(Callback::Return(self.visit_expression(expression)))),
            Statement::Echo(expression) => {
                echo(self.visit_expression(expression));
                Ok(None)
            },
            Statement::While(expression, stmt) => 
                self.visit_while(expression, stmt),
            Statement::If(expression, stmt1, stmt2) =>
                self.visit_if(expression, stmt1, stmt2),
            Statement::ExpressionStmt(expression) => {
                self.visit_expression(expression);
                Ok(None)
            },
            Statement::FunctionDecl(name, args, stmt) => {
                self.visit_func_decl(name, args, stmt);
                Ok(None)
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> Object {
        match expression {
            Expression::Primary(prim) => self.visit_prim(prim),
            Expression::BinaryOperation(op, expr1, expr2) =>
                self.visit_bin_op(op, expr1, expr2)
        }
    }

    fn visit_prim(&mut self, primary: &PrimaryExpression) -> Object {
        match primary {
            PrimaryExpression::UnaryPlus(pr) => 
                self.visit_prim(pr).unary_plus(),
            PrimaryExpression::UnaryMinus(pr) =>
                self.visit_prim(pr).unary_minus(),
            PrimaryExpression::UnaryNot(pr) =>
                self.visit_prim(pr).not(),
            PrimaryExpression::InBrackets(expr) =>
                self.visit_expression(expr),
            PrimaryExpression::Ident(name) => 
                self.memory.get_var(name.clone()),
            PrimaryExpression::Float(x) =>
                Object::Float(*x),
            PrimaryExpression::Int(x) =>
                Object::Int(*x),
            PrimaryExpression::Str(x) => 
                Object::Str(x.clone()),
            PrimaryExpression::Boolean(x) =>
                Object::Boolean(*x),
            PrimaryExpression::Call(call_object, args) =>
                
        }
    }

    fn visit_bin_op(&mut self, operator: &BinaryOperator, 
    left: &Box<Expression>, right: &Box<Expression>) -> Object {
        
    }

    fn visit_while(&mut self, expression: &Expression, statement: &Box<Statement>) ->
    Result<Option<Callback>, String> {
        Ok(Some(Callback::Break))
    }

    fn visit_if(&mut self, expression: &Expression, 
    first_statement: &Box<Statement>, second_statement: &Option<Box<Statement>>) ->
    Result<Option<Callback>, String> {
        Ok(Some(Callback::Break))
    }

    fn visit_func_decl(&mut self, name: &String, args: &Vec<Expression>, 
    statement: &Box<Statement>) -> Result<(), String> {
        Ok(())
    }

    fn visit_func_call(&mut self, call_object: &Box<PrimaryExpression>, 
    args: &Vec<Expression>) -> Result<Object, String> {

    }

}