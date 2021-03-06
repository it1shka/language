use crate::parser::ast::*;

use super::object::Object;
use super::storage::MemStack;
use super::builtins::*;

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
        self.init_builtins();
        let statements = &ast.0;
        let maybe_callback = self.visit_statement_list(statements)?;
        if let Some(Callback::Return(object)) = maybe_callback {
            echo(object);
        }
        Ok(())
    }

    fn init_builtins(&mut self) {
        self.memory.set_var("print".to_string(), Object::BuiltIn(print));
        self.memory.set_var("input".to_string(), Object::BuiltIn(input));
        self.memory.set_var("int".to_string(), Object::BuiltIn(int));
        self.memory.set_var("float".to_string(), Object::BuiltIn(float));
        self.memory.set_var("bool".to_string(), Object::BuiltIn(bool_));
        self.memory.set_var("typeof".to_string(), Object::BuiltIn(object_typeof));
        self.memory.set_var("string".to_string(), Object::BuiltIn(string))
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
                Ok(Some(Callback::Return(self.visit_expression(expression)?))),
            Statement::Echo(expression) => {
                echo(self.visit_expression(expression)?);
                Ok(None)
            },
            Statement::While(expression, stmt) => 
                self.visit_while(expression, stmt),
            Statement::If(expression, stmt1, stmt2) =>
                self.visit_if(expression, stmt1, stmt2),
            Statement::ExpressionStmt(expression) => {
                self.visit_expression(expression)?;
                Ok(None)
            },
            Statement::FunctionDecl(name, args, stmt) => {
                self.visit_func_decl(name, args, stmt)?;
                Ok(None)
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<Object, String> {
        match expression {
            Expression::Primary(prim) => self.visit_prim(prim),
            Expression::BinaryOperation(op, expr1, expr2) =>
                self.visit_bin_op(op, expr1, expr2)
        }
    }

    fn visit_prim(&mut self, primary: &PrimaryExpression) -> Result<Object, String> {
        match primary {
            PrimaryExpression::UnaryPlus(pr) => 
                Ok(self.visit_prim(pr)?.unary_plus()),
            PrimaryExpression::UnaryMinus(pr) =>
                Ok(self.visit_prim(pr)?.unary_minus()),
            PrimaryExpression::UnaryNot(pr) =>
                Ok(self.visit_prim(pr)?.not()),
            PrimaryExpression::InBrackets(expr) =>
                self.visit_expression(expr),
            PrimaryExpression::Ident(name) => 
                Ok(self.memory.get_var(name.clone())),
            PrimaryExpression::Float(x) =>
                Ok(Object::Float(*x)),
            PrimaryExpression::Int(x) =>
                Ok(Object::Int(*x)),
            PrimaryExpression::Str(x) => 
                Ok(Object::Str(x.clone())),
            PrimaryExpression::Boolean(x) =>
                Ok(Object::Boolean(*x)),
            PrimaryExpression::Null =>
                Ok(Object::Null),
            PrimaryExpression::Call(call_object, args) =>
                self.visit_func_call(call_object, args)
        }
    }

    fn visit_bin_op(&mut self, operator: &BinaryOperator, 
    left: &Box<Expression>, right: &Box<Expression>) -> Result<Object, String> {
        match operator {
            BinaryOperator::Assign => {
                match &**left {
                    Expression::Primary(PrimaryExpression::Ident(name)) 
                    => {
                        let val = self.visit_expression(right)?;
                        self.memory.set_or_rewrite_var(name.clone(), val.clone());
                        Ok(val)
                    },
                    _ => {Err("Can't assign to a constant".to_string()) }
                }
            },

            BinaryOperator::Add => 
            Ok(self.visit_expression(left)?.add(&self.visit_expression(right)?)),
            BinaryOperator::Sub => 
            Ok(self.visit_expression(left)?.sub(&self.visit_expression(right)?)),
            BinaryOperator::Mul => 
            Ok(self.visit_expression(left)?.mul(&self.visit_expression(right)?)),
            BinaryOperator::Div => 
            Ok(self.visit_expression(left)?.div(&self.visit_expression(right)?)),
            BinaryOperator::Mod => 
            Ok(self.visit_expression(left)?.mod_(&self.visit_expression(right)?)),
            

            BinaryOperator::Equal => 
            Ok(self.visit_expression(left)?.equal(&self.visit_expression(right)?)),
            BinaryOperator::NotEqual => 
            Ok(self.visit_expression(left)?.not_equal(&self.visit_expression(right)?)),
            BinaryOperator::Greater => 
            Ok(self.visit_expression(left)?.greater(&self.visit_expression(right)?)),
            BinaryOperator::Less => 
            Ok(self.visit_expression(left)?.less(&self.visit_expression(right)?)),
            BinaryOperator::GreaterOrEqual => 
            Ok(self.visit_expression(left)?.greater_or_equal(&self.visit_expression(right)?)),
            BinaryOperator::LessOrEqual => 
            Ok(self.visit_expression(left)?.less_or_equal(&self.visit_expression(right)?)),
            
            BinaryOperator::And => 
            Ok(self.visit_expression(left)?.and(&self.visit_expression(right)?)),
            BinaryOperator::Or => 
            Ok(self.visit_expression(left)?.or(&self.visit_expression(right)?)),
            
            BinaryOperator::StrAdd =>
            Ok(self.visit_expression(left)?.str_add(&self.visit_expression(right)?)),

            BinaryOperator::Not => 
            Err("Unexpected 'not' operator!".to_string())
            
        }
    }

    fn visit_while(&mut self, expression: &Expression, statement: &Box<Statement>) ->
    Result<Option<Callback>, String> {
        self.memory.new_scope();
        while let Object::Boolean(true) = self.visit_expression(expression)? {
            if let Some(callback) = self.visit_statement(statement)? {
                match callback {
                    Callback::Continue => (),
                    Callback::Break => {self.memory.leave_scope(); return Ok(None)},
                    _ => {self.memory.leave_scope(); return Ok(Some(callback))}
                }
            }
        }
        self.memory.leave_scope();
        Ok(None)
    }

    fn visit_if(&mut self, expression: &Expression, 
    first_statement: &Box<Statement>, second_statement: &Option<Box<Statement>>) ->
    Result<Option<Callback>, String> {
        self.memory.new_scope();
        if let Object::Boolean(true) = self.visit_expression(expression)? {
            let maybe_callback = self.visit_statement(first_statement);
            self.memory.leave_scope();
            maybe_callback
        }
        else {
            if let Some(second) = second_statement {
                let maybe_callback = self.visit_statement(second);
                self.memory.leave_scope();
                maybe_callback
            }
            else {
                self.memory.leave_scope();
                Ok(None)
            }
        }
    }

    fn visit_func_decl(&mut self, name: &String, args: &Vec<String>, 
    statement: &Box<Statement>) -> Result<(), String> {
        let f_object = Object::Function(args.clone(), statement.clone());
        self.memory.set_or_rewrite_var(name.clone(), f_object);
        Ok(())
    }

    fn visit_func_call(&mut self, call_object: &Box<PrimaryExpression>, 
    call_args: &Vec<Expression>) -> Result<Object, String> {
        match self.visit_prim(call_object)? {
            Object::Function(func_args, body) => {
                self.memory.new_scope();
                for (f_arg, c_arg) in func_args.iter().zip(call_args.iter()) {
                    let value = self.visit_expression(c_arg)?;
                    self.memory.set_var(f_arg.clone(), value);
                }
                
                let result = match self.visit_statement(&body)? {
                    Some(callback) => match callback {
                        Callback::Return(object) => Ok(object),
                        Callback::Break | Callback::Continue => Ok(Object::Null)
                    },
                    None => Ok(Object::Null)
                };
                self.memory.leave_scope();
                result
            },
            Object::BuiltIn(builtin_function) => {
                let arguments_map = call_args
                    .iter()
                    .map(|argument| self.visit_expression(argument));
                let mut arguments: Vec<Object> = Vec::new();
                for maybe_arg in arguments_map {
                    arguments.push(maybe_arg?);
                }
                let builtin_result = builtin_function(arguments);
                builtin_result
            }
            _ => Err(format!("Can't call '{:?}' object!", call_object))
        }
    }

}