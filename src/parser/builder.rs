use crate::lexer::stream::Stream;
use crate::lexer::token::Token;

use super::ast::*;

use std::iter::Peekable;

fn tr_op(token: Token) -> BinaryOperator {
    match token {
        Token::Add => BinaryOperator::Add,
        Token::Sub => BinaryOperator::Sub,
        Token::Mul => BinaryOperator::Mul,
        Token::Div => BinaryOperator::Div,
        Token::Mod => BinaryOperator::Mod,
        Token::Equal => BinaryOperator::Equal,
        Token::NotEqual => BinaryOperator::NotEqual,
        Token::Greater => BinaryOperator::Greater,
        Token::Less => BinaryOperator::Less,
        Token::GreaterOrEqual => BinaryOperator::GreaterOrEqual,
        Token::LessOrEqual => BinaryOperator::LessOrEqual,
        Token::And => BinaryOperator::And,
        Token::Or => BinaryOperator::Or,
        Token::Not => BinaryOperator::Not,
        Token::Assign => BinaryOperator::Assign,
        _ => panic!("Can't translate Token to BinaryOperator")
    }
}

pub struct Builder<'a> {
    stream: Peekable<Stream<'a>>
}

impl<'a> Builder<'a> {
    pub fn new(stream: Stream) -> Builder {
        Builder {
            stream: stream.peekable()
        }
    }

    //core functions

    fn next(&mut self) -> Result<Token, String> {
        self.stream.next().unwrap()
    }

    fn peek(&mut self) -> Result<Token, String> {
        self.stream.peek().unwrap().clone()
    }

    fn eat(&mut self, token_type: Token) -> Result<(), String> {
        let next_token = self.peek()?;
        if token_type != next_token {
            Err(format!("Expected token of type '{:?}', not '{:?}'", 
            token_type, next_token))
        }
        else {
            self.next()?;
            Ok(())
        }
    }

    //parsing functions

    pub fn build(&mut self) -> Result<Program, String> {
        let statements = self.parse_program()?;
        Ok(Program(statements))
    }

    fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.peek()?.is_eof() {
            let statement = self.parse_statement()?;
            statements.push(statement)
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek()? {
            Token::LeftBrace => self.parse_statement_list(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Return => self.parse_return(),
            //вывод
            Token::Echo => self.parse_echo(),
            Token::While => self.parse_while(),
            Token::If => self.parse_if(),
            Token::Function => self.parse_function(),
            _ => self.parse_expression_stmt()
        }
    }

    fn parse_statement_list(&mut self) -> Result<Statement, String> {
        self.eat(Token::LeftBrace)?;
        let mut statements: Vec<Statement> = Vec::new();
        loop {
            match self.peek()? {
                Token::RightBrace | Token::EOF => {
                    self.next()?;
                    break;
                },
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement)
                }
            }
        }
        Ok(Statement::List(statements))
    }

    fn parse_break(&mut self) -> Result<Statement, String> {
        self.eat(Token::Break)?;
        self.eat(Token::Semicolon)?;
        Ok(Statement::Break)
    }

    fn parse_continue(&mut self) -> Result<Statement, String> {
        self.eat(Token::Continue)?;
        self.eat(Token::Semicolon)?;
        Ok(Statement::Continue)
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        self.eat(Token::Return)?;
        let expression = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Statement::Return(expression))
    }

    //вывод
    fn parse_echo(&mut self) -> Result<Statement, String> {
        self.eat(Token::Echo)?;
        let expression = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Statement::Echo(expression))
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.eat(Token::While)?;
        self.eat(Token::LeftBracket)?;
        let expression = self.parse_expression()?;
        self.eat(Token::RightBracket)?;
        let statement = self.parse_statement()?;
        Ok(Statement::While(expression, Box::new(statement)))

    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.eat(Token::If)?;
        self.eat(Token::LeftBracket)?;
        let expression = self.parse_expression()?;
        self.eat(Token::RightBracket)?;
        let statement1 = self.parse_statement()?;
        match self.peek()? {
            Token::Else => {
                self.eat(Token::Else)?;
                let statement2 = self.parse_statement()?;
                Ok(Statement::If(
                    expression,
                    Box::new(statement1),
                    Some(Box::new(statement2))
                ))
            },
            _ => {
                Ok(Statement::If(
                    expression, 
                    Box::new(statement1),
                    None 
                ))
            }
        }
    }

    fn parse_function(&mut self) -> Result<Statement, String> {
        self.eat(Token::Function)?;
        let ident = self.next()?;
        match ident {
            Token::Ident(name) => {
                self.eat(Token::LeftBracket)?;
                let args = self.parse_args()?;
                self.eat(Token::RightBracket)?;
                let body = self.parse_statement()?;
                Ok(Statement::FunctionDecl(name, args, Box::new(body)))
            },
            _ => Err(format!(
                "Expected function name, not '{:?}'", ident
            ))
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, String> {
        let mut args: Vec<Expression> = Vec::new();
        while self.peek()? != Token::RightBracket {
            let expr = self.parse_expression()?;
            args.push(expr);
            match self.peek()? {
                Token::Comma => {self.next()?; },
                _ => break
            }
        }
        Ok(args)
    }

    fn parse_expression_stmt(&mut self) -> Result<Statement, String> {
        let expression = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Statement::ExpressionStmt(expression))
    }

    //expressions
    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.expr1()
    }

    fn expr1(&mut self) -> Result<Expression, String> {
        let mut left = self.expr2()?;
        while let Token::Assign = self.peek()? {
            let op = self.next()?;
            let right = self.expr2()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr2(&mut self) -> Result<Expression, String> {
        let mut left = self.expr3()?;
        while let Token::Or = self.peek()? {
            let op = self.next()?;
            let right = self.expr3()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr3(&mut self) -> Result<Expression, String> {
        let mut left = self.expr4()?;
        while let Token::And = self.peek()? {
            let op = self.next()?;
            let right = self.expr4()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr4(&mut self) -> Result<Expression, String> {
        let mut left = self.expr5()?;
        while match self.peek()? {
            Token::Equal | Token::NotEqual => true,
            _ => false
        }{
            let op = self.next()?;
            let right = self.expr5()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr5(&mut self) -> Result<Expression, String> {
        let mut left = self.expr6()?;
        while match self.peek()? {
            Token::Less 
            | Token::LessOrEqual
            | Token::Greater
            | Token::GreaterOrEqual => true,
            _ => false
        }{
            let op = self.next()?;
            let right = self.expr6()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr6(&mut self) -> Result<Expression, String> {
        let mut left = self.expr7()?;
        while match self.peek()? {
            Token::Add | Token::Sub => true,
            _ => false
        }{
            let op = self.next()?;
            let right = self.expr7()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr7(&mut self) -> Result<Expression, String> {
        let mut left = self.expr8()?;
        while match self.peek()? {
            Token::Mul 
            | Token::Div
            | Token::Mod => true,
            _ => false
        }{
            let op = self.next()?;
            let right = self.expr8()?;
            left = Expression::
                BinaryOperation(
                    tr_op(op),
                    Box::new(left),
                    Box::new(right)
                );
        }
        Ok(left)
    }

    fn expr8(&mut self) -> Result<Expression, String> {
        Ok(Expression::Primary(
            self.parse_primary_highlevel()?
        ))
    }

    //primary expression
    
    fn parse_primary_highlevel(&mut self) -> Result<PrimaryExpression, String> {
        self.parse_post_ops()
    }

    //parses call operator '()' and later get operator '[]'
    fn parse_post_ops(&mut self) -> Result<PrimaryExpression, String> {
        let mut prim = self.parse_primary()?;
        while let Token::LeftBracket = self.peek()? {
            self.eat(Token::LeftBracket)?;
            let args = self.parse_args()?;
            self.eat(Token::RightBracket)?;
            prim = PrimaryExpression::Call(
                Box::new(prim),
                args
            )
        }
        Ok(prim)
    }

    fn parse_primary(&mut self) -> Result<PrimaryExpression, String> {
        let tok = self.next()?;
        match tok {
            Token::Ident(x) => Ok(PrimaryExpression::Ident(x)),
            Token::Int(x) => Ok(PrimaryExpression::Int(x)),
            Token::Float(x) => Ok(PrimaryExpression::Float(x)),
            Token::Str(x) => Ok(PrimaryExpression::Str(x)),
            Token::True => Ok(PrimaryExpression::Boolean(true)),
            Token::False => Ok(PrimaryExpression::Boolean(false)),
            Token::Null => Ok(PrimaryExpression::Null),
            Token::LeftBracket => {
                let expr = self.parse_expression()?;
                self.eat(Token::RightBracket)?;
                Ok(PrimaryExpression::InBrackets(Box::new(expr)))
            },
            Token::Add => Ok(PrimaryExpression::UnaryPlus(
                Box::new(self.parse_primary()?)
            )),
            Token::Sub => Ok(PrimaryExpression::UnaryMinus(
                Box::new(self.parse_primary()?)
            )),
            Token::Not => Ok(PrimaryExpression::UnaryNot(
                Box::new(self.parse_primary()?)
            )),
            _ => Err(format!("Unexpected token '{:?}' while parsing primary!", tok))
        }
    }

}