use super::token::Token;

use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

pub struct Stream<'a> {
    buffer: Peekable<Chars<'a>>,
    line: usize,
    column: usize
}

impl<'a> Stream<'a> {

    pub fn new(input: &str) -> Stream {
        Stream {
            buffer: input.chars().peekable(),
            line: 0,
            column: 0
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.buffer.next();
        if let Some('\n') = ch {
            self.line += 1;
            self.column = 0;
        }
        else {
            self.column += 1;
        }
        ch
    }

    fn peek(&mut self) -> Option<&char> {
        self.buffer.peek()
    }

    fn read_while(&mut self, predicate: &impl Fn(char) -> bool) -> String {
        let mut val = String::new();
        while let Some(current) = self.peek() {
            if predicate(*current) {
                val.push(*current);
                self.next();
            }
            else {
                break;
            }
        }
        val
    }

    fn eat_whitespace(&mut self) {
        self.read_while(&|x| match x {
            ' '| '\n' | '\t' | '\r' => true,
            _ => false
        }); 
    }

    fn eat_comment(&mut self) {
        self.read_while(&|x| x != '\n');
    }

    fn read_token(&mut self) -> Result<Token, String> {
        
        self.eat_whitespace();

        let current = self.peek();
        match current {
            None => Ok(Token::EOF),
            Some(c) => match c {

                //operators

                '+' => {
                    self.next();
                    Ok(Token::Add)
                },
                '-' => {
                    self.next();
                    Ok(Token::Sub)
                },
                '*' => {
                    self.next();
                    Ok(Token::Mul)
                },
                '/' => {
                    self.next();
                    if let Some('/') = self.peek() {
                        self.eat_comment();
                        self.read_token()
                    }
                    else {
                        Ok(Token::Div)
                    }
                },
                '%' => {
                    self.next();
                    Ok(Token::Mod)
                },
                '=' => {
                    self.next();
                    if let Some('=') = self.peek() {
                        self.next();
                        Ok(Token::Equal)
                    }else {
                        Ok(Token::Assign)
                    }
                },
                '!' => {
                    self.next();
                    if let Some('=') = self.peek() {
                        self.next();
                        Ok(Token::NotEqual)
                    }
                    else {
                        Ok(Token::Not)    
                    }
                },
                '>' => {
                    self.next();
                    if let Some('=') = self.peek() {
                        self.next();
                        Ok(Token::GreaterOrEqual)
                    } else {
                        Ok(Token::Greater)
                    }
                },
                '<' => {
                    self.next();
                    if let Some('=') = self.peek() {
                        self.next();
                        Ok(Token::LessOrEqual)
                    } else {
                        Ok(Token::Less)
                    }
                },
                '&' => {
                    self.next();
                    if let Some('&') = self.peek() {
                        self.next();
                        Ok(Token::And)
                    } else {
                        Err(String::from("'&' operator hasn't been implemented yet"))
                    }
                },
                '|' => {
                    self.next();
                    if let Some('|') = self.peek() {
                        self.next();
                        Ok(Token::Or)
                    } else {
                        Err(String::from("'|' operator hasn't been implemented yet"))
                    }
                },

                //punctuation
                
                '(' => {
                    self.next();
                    Ok(Token::LeftBracket)
                },
                ')' => {
                    self.next();
                    Ok(Token::RightBracket)
                },
                '{' => {
                    self.next();
                    Ok(Token::LeftBrace)
                },
                '}' => {
                    self.next();
                    Ok(Token::RightBrace)
                },
                ';' => {
                    self.next();
                    Ok(Token::Semicolon)
                },
                ',' => {
                    self.next();
                    Ok(Token::Comma)
                },

                //number

                '0' ..= '9' => 
                    Ok(self.read_number()),
                
                //word

                'a' ..= 'z' | 'A' ..= 'Z' | '_' =>
                    Ok(self.read_word()),

                //string literal

                '"' | '\'' => 
                    Ok(self.read_string()),

                //other

                _ => Err(String::from(format!(
                    "Unexpected character while lexing: '{}'",
                    *c
                )))

            }
        }
    }

    fn read_number(&mut self) -> Token {
        let is_digit = |x| match x {
            '0' ..= '9' => true,
            _ => false
        };
        let mut number = self.read_while(&is_digit);
        if let Some('.') = self.peek() {
            number.push(self.next().unwrap());
            number += &self.read_while(&is_digit);
            let val = number.parse::<f64>().unwrap();
            Token::Float(val)
        } else {
            let val = number.parse::<i32>().unwrap();
            Token::Int(val)
        }
    }

    fn read_word(&mut self) -> Token {
        let is_letter = |x| match x {
            '_' | 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => true,
            _ => false
        };

        let word = self.read_while(&is_letter);

        //matching with keywords
        match &word[..] {
            "break" => Token::Break,
            "continue" => Token::Continue,
            "else" => Token::Else,
            "if" => Token::If,
            "return" => Token::Return,
            "while" => Token::While,
            "function" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(word)
        }
    }

    //just a simple impl. To fix later....
    fn read_string(&mut self) -> Token {
        let q = self.next().unwrap();
        let value = self.read_while(&|x| x != q);
        self.next();
        Token::Str(value)
    }
}

impl Iterator for Stream<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_token())
    }
}