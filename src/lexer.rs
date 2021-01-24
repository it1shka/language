use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Int(String),
    Float(String),

    //operators
    Add,            //+
    Sub,            //-
    Mul,            //*
    Div,            // /
    Mod,            // %

    Equal,          // ==
    NotEqual,       // !=
    Greater,        // >
    Less,           // <
    GreaterOrEqual, // >=
    LessOrEqual,    // <=

    And,            // &&
    Or,             // ||
    Not,            // !

    Assign,         // =

    //punctuation
    LeftBracket,    // (
    RightBracket,   // )
    LeftBrace,      // {
    RightBrace,     // }
    Semicolon,      // ;
    Comma,          // ,

    //keywords

    //error special token
    LexicalError(String)
}

#[derive(Debug)]
pub struct TokenInfo {
    token: Token,
    line: u32,
    column: u32
}

impl TokenInfo {
    fn new(token: Token, line: u32, column: u32) 
        -> TokenInfo {
            TokenInfo{
                token,
                line,
                column
            }
        }
}

pub struct TokenStream<'a> {
    char_stream: Peekable<Chars<'a>>,
    column: u32,
    line: u32,
    error: bool
}

impl<'a> TokenStream<'a> {

    pub fn new(input: &String) -> TokenStream {
        TokenStream {
            char_stream: input.chars().peekable(),
            column: 0,
            line: 0,
            error: false
        }
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.char_stream.next();
        //tracking the position
        match ch {
            None => (),
            Some(ch) => match ch {
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                }
                _ => self.column += 1
            }
        }
        ch
    }

    fn read_while(&mut self, condition: impl Fn(char) -> bool ) -> String{
        let mut value = String::new();
        while let Some(current) = self.char_stream.peek() {
            if condition(*current) {
                value.push(*current);
                self.next();
            }
            else {
                break;
            }
        }
        value
    }

    fn read_token(&mut self) -> Option<Token> {
        //check if there is an error
        if self.error {
            return None;
        }

        //skipping whitespace
        self.read_while(|x| match x {
            ' ' | '\n' | '\t' | '\r' => true,
            _ => false
        });

        let current = self.char_stream.peek();
        match current {
            None => None,
            Some(c) => match c {
                //operators
                '+' => {
                    self.next();
                    Some(Token::Add)
                },
                '-' => {
                    self.next();
                    Some(Token::Sub)
                },
                '*' => {
                    self.next();
                    Some(Token::Mul)
                },
                '/' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    match nc {
                        None => Some(Token::Div),
                        //check if comment
                        Some(ncc) => if *ncc == '/' {
                            self.skip_comment();
                            self.read_token()
                        }
                        else {
                            Some(Token::Div)
                        }
                    }
                },
                '%' => {
                    self.next();
                    Some(Token::Mod)
                },
                '=' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    match nc {
                        None => Some(Token::Assign),
                        Some(ncc) => if *ncc == '=' {
                            self.next();
                            Some(Token::Equal)
                        }
                        else {
                            Some(Token::Assign)
                        }
                    }
                },
                '!' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    match nc {
                        None => Some(Token::Not),
                        Some(ncc) => if *ncc == '=' {
                            self.next();
                            Some(Token::NotEqual)
                        }else {
                            Some(Token::Not)
                        }
                    }
                },
                '>' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    match nc {
                        None => Some(Token::Greater),
                        Some(ncc) => if *ncc == '=' {
                            self.next();
                            Some(Token::GreaterOrEqual)
                        }else {
                            Some(Token::Greater)
                        }
                    }
                },
                '<' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    match nc {
                        None => Some(Token::Less),
                        Some(ncc) => if *ncc == '=' {
                            self.next();
                            Some(Token::LessOrEqual)
                        }else {
                            Some(Token::Less)
                        }
                    }
                },
                '&' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    
                    match nc {
                        None => {
                            self.error = true;
                            Some(Token::LexicalError(
                                String::from("'&' operator hasn't been implemented yet")
                            ))
                        },
                        Some(ncc) => if *ncc == '&' {
                            self.next();
                            Some(Token::And)
                        }else {
                            self.error = true;
                            Some(Token::LexicalError(
                                String::from("'&' operator hasn't been implemented yet")
                            ))
                        }
                    }
                },
                '|' => {
                    self.next();
                    let nc = self.char_stream.peek();
                    
                    match nc {
                        None => {
                            self.error = true;
                            Some(Token::LexicalError(
                                String::from("'|' operator hasn't been implemented yet")
                            ))
                        },
                        Some(ncc) => if *ncc == '|' {
                            self.next();
                            Some(Token::Or)
                        }else {
                            self.error = true;
                            Some(Token::LexicalError(
                                String::from("'|' operator hasn't been implemented yet")
                            ))
                        }
                    }
                },

                //punctuation
                '(' => {
                    self.next();
                    Some(Token::LeftBracket)
                },
                ')' => {
                    self.next();
                    Some(Token::RightBracket)
                },
                '{' => {
                    self.next();
                    Some(Token::LeftBrace)
                },
                '}' => {
                    self.next();
                    Some(Token::RightBrace)
                },
                ';' => {
                    self.next();
                    Some(Token::Semicolon)
                },
                ',' => {
                    self.next();
                    Some(Token::Comma)
                },

                //number
                '0'..='9' => {
                    Some(self.read_number())
                },

                //word
                'a'..='z' | 'A' ..= 'Z' | '_' => {
                    Some(self.read_word())
                },

                _ => {
                    self.error = true;
                    Some(Token::LexicalError(
                        String::from(format!("Unexpected character: {}", *c))
                    ))
                }
            }
        }
    }

    fn read_token_with_position(&mut self) -> Option<TokenInfo> {
        let token = self.read_token();
        match token {
            None => None,
            Some(token) =>
            Some(
                TokenInfo::new(
                    token,
                    self.line,
                    self.column
                )
            )
        }
    }

    fn read_number(&mut self) -> Token {
        let is_digit = |x| match x {
            '0'..='9' => true,
            _ => false
        };

        let mut number = self.read_while(is_digit);
        let nc = self.char_stream.peek();
        match nc {
            None => Token::Int(number),
            Some(ncc) => if *ncc == '.' {
                number.push(self.next().unwrap());
                number += &self.read_while(is_digit);
                Token::Float(number)

            }else {
                Token::Int(number)
            }
        }

    }

    fn read_word(&mut self) -> Token {
        let is_letter = |x| match x {
            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false
        };

        let word = self.read_while(is_letter);
        Token::Ident(word)
    }

    fn skip_comment(&mut self) {
        self.read_while(|x| x != '\n');
    }

}

impl Iterator for TokenStream<'_> {
    type Item = TokenInfo;
    fn next(&mut self) -> Option<Self::Item> {
        self.read_token_with_position()
    }
}