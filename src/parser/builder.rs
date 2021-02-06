use crate::lexer::stream::Stream;
use crate::lexer::token::Token;

use super::ast::*;

use std::iter::Peekable;

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

    fn drop(&mut self, amount: u8) -> Result<(), String> {
        for i in 0..amount {
            self.next()?;
        }
        Ok(())
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
                let args = self.parse_decl_args()?;
                let body = self.parse_statement()?;
                Ok(Statement::Function(name, args, Box::new(body)))
            },
            _ => Err(format!(
                "Expected function name, not '{:?}'", ident
            ))
        }
    }

    //еле работает
    fn parse_decl_args(&mut self) -> Result<Vec<String>, String> {
        let mut args: Vec<String> = Vec::new();
        self.eat(Token::LeftBracket)?;
        loop {
            if let Token::Ident(name) = self.peek()? {
                args.push(name);
                self.next()?;
                if let Token::Comma = self.peek()? {
                    self.next()?;
                }
                else {
                    break;
                }
            }
        }
        self.eat(Token::RightBracket)?;
        Ok(args)
    }

    fn parse_expression_stmt(&mut self) -> Result<Statement, String> {
        let expression = self.parse_expression()?;
        Ok(Statement::ExpressionStmt(expression))
    }

    //here we go: expressions!!!
    //продолжу завтра ибо еще оч много что писать
    fn parse_expression(&mut self) -> Result<Expression, String> {
        //просто втыкнул чтобы тестить
        Ok(Expression::Primary(PrimaryExpression::Int(1)))
    }

}