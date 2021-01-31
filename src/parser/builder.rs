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

    //to fix later... it actually should return a ref like
    // &Result<Token, String>
    fn peek(&mut self) -> Result<Token, String> {
        self.stream.peek().unwrap().clone()
    }

    fn expect(&mut self, t_type: Token) -> Result<bool, String> {
        Ok(t_type == self.peek()?)
    }

    fn eat(&mut self, t_type: Token) -> Result<Token, String> {
        if t_type != self.peek()? {
            Err(format!("Expected token of type '{}'", t_type))
        }
        else {
            self.next()
        }
    }

    //

    pub fn get_ast(&mut self) -> Result<Program, String> {
        let statements = self.parse_program()?;
        Ok(Program(statements))
    }

    fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        //replace and continue
        Err(String::from("You Are Gay"))
    }

}