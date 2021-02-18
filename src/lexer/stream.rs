use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

use super::token::Token;

pub struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
    current_char: Option<char>
}

//added a small macro, for better flexibility
macro_rules! tok {
    ($token:expr) => {
        Ok($token)
    };
}

impl<'a> Stream<'a> {
    pub fn new(source: &str) -> Stream {
        Stream {
            chars: source.chars().peekable(),
            current_char: None
        }
    }

    fn next(&mut self) -> Option<char> {
        self.current_char = self.chars.next();
        self.current_char
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn get_cur(&self) -> char {
        self.current_char.unwrap()
    }

    fn read_while(&mut self, 
        predicate: impl Fn(char) -> bool) -> String {
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if predicate(*c) {
                value.push(*c);
                self.next();
            }
            else {
                break
            }
        }
        value
    }

    //core function
    fn read_token(&mut self) -> Result<Token, String> {
        self.eat_whitespace();

        match self.next() {
            None => tok!(Token::EOF),
            Some(ch) => match ch {
                '+' => tok!(Token::Add),
                '-' => tok!(Token::Sub),
                '*' => tok!(Token::Mul),
                '.' => tok!(Token::StrAdd),
                '/' => {
                    if let Some('/') = self.peek() {
                        self.eat_comment();
                        self.read_token()
                    }
                    else {
                        tok!(Token::Div)
                    }
                },
                '%' => tok!(Token::Mod),
                '=' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        tok!(Token::Equal)
                    }
                    else {
                        tok!(Token::Assign)
                    }
                },
                '!' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        tok!(Token::NotEqual)
                    }
                    else {
                        tok!(Token::Not)
                    }
                },
                '>' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        tok!(Token::GreaterOrEqual)
                    }
                    else {
                        tok!(Token::Greater)
                    }
                },
                '<' => {
                    if let Some('=') = self.peek() {
                        self.next();
                        tok!(Token::LessOrEqual)
                    }
                    else {
                        tok!(Token::Less)
                    }
                },
                '&' => {
                    if let Some('&') = self.next() {
                        tok!(Token::And)
                    }
                    else {
                        Err(String::from("'&' isn't implemented yet!"))
                    }
                },
                '|' => {
                    if let Some('|') = self.next() {
                        tok!(Token::Or)
                    }
                    else {
                        Err(String::from("'|' isn't implemented yet!"))
                    }
                },
                '(' => tok!(Token::LeftBracket),
                ')' => tok!(Token::RightBracket),
                '{' => tok!(Token::LeftBrace),
                '}' => tok!(Token::RightBrace),
                ';' => tok!(Token::Semicolon),
                ',' => tok!(Token::Comma),

                'a' ..= 'z' | 'A' ..= 'Z' | '_' =>
                    tok!(self.read_word()),
                
                '"' | '\'' => 
                    tok!(self.read_string_literal()),

                '0' ..= '9' => self.read_number(),

                _ => {
                    Err(format!("Unexpected character: '{}'", 
                    self.get_cur()))
                }

            }
        }
    }

    //precore functions
    fn eat_whitespace(&mut self) {
        self.read_while(|x| match x {
            '\n' | '\t' | ' ' | '\r' => true,
            _ => false
        });
    }

    fn eat_comment(&mut self) {
        self.read_while(|x| x != '\n');
    }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();
        word.push(self.get_cur());
        word.push_str(&self.read_while(Stream::is_letter));
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
            "null" => Token::Null,
            //вывод
            "echo" => Token::Echo,
            _ => Token::Ident(word)
        }
    }

    //just a simple implementation. later fix that... Maybe not
    fn read_string_literal(&mut self) -> Token {
        let q = self.get_cur();
        let value = self.read_while(|x| x != q);
        self.next();
        Token::Str(value)
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();
        number.push(self.get_cur());
        number.push_str(& self.read_while(Stream::is_digit));
        if let Some('.') = self.peek() {
            self.next();
            number.push(self.get_cur());
            number.push_str(& self.read_while(Stream::is_digit));
            let val = number.parse::<f64>();
            match val {
                Ok(v) => Ok(Token::Float(v)),
                Err(_) => Err(String::from("Can't parse floating number!"))
            }
        }
        else {
            let val = number.parse::<i32>();
            match val {
                Ok(v) => Ok(Token::Int(v)),
                Err(_) => Err(String::from("Can't parse integer number!"))
            }
        }
    }

    fn is_letter(x: char) -> bool {
        match x {
            '_' | 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => true,
            _ => false
        }
    }

    fn is_digit(x: char) -> bool {
        match x {
            '0' ..= '9' => true,
            _ => false
        }
    }

}

impl Iterator for Stream<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_token())
    }
}