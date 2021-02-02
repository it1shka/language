
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    
    EOF,
    Ident(String),
    Int(i32),
    Float(f64),
    Str(String),

    True,
    False,

    //keywords
    Break,
    Continue,
    Return,
    While,
    If,
    Else,
    Function,
    
    //opearators
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
    
}

//some useful functions for parsing
impl Token {

    pub fn is_int(&self) -> bool {
        match self {
            Token::Int(_) => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Token::Float(_) => true,
            _ => false
        }
    }

    pub fn is_id(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Token::Str(_) => true,
            _ => false
        }
    }

    pub fn is_num(&self) -> bool {
        self.is_int() || self.is_float()
    }

    pub fn is_prim(&self) -> bool {
        self.is_num() || self.is_id() || self.is_str()
    }

    pub fn is_eof(&self) -> bool {
        match self {
            Token::EOF => true,
            _ => false
        }
    }

}

//
use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match self {
                Token::EOF              => "End of file",
                Token::Ident(_)         => "Ident",
                Token::Int(_)           => "Int",
                Token::Float(_)         => "Float",
                Token::Str(_)           => "Str",
                Token::True             => "True",
                Token::False            => "False",
                Token::Break            => "Break",
                Token::Continue         => "Continue",
                Token::Return           => "Return",
                Token::While            => "While",
                Token::If               => "If",
                Token::Else             => "Else",
                Token::Function         => "Function",
                Token::Add              => "+",
                Token::Sub              => "-",
                Token::Mul              => "*",
                Token::Div              => "/",
                Token::Mod              => "%",
                Token::Equal             => "==",
                Token::NotEqual         => "!=",
                Token::Greater          => ">",
                Token::Less             => "<",
                Token::GreaterOrEqual   => ">=",
                Token::LessOrEqual      => "<=",
                Token::And              => "&&",
                Token::Or               => "||",
                Token::Not              => "!",
                Token::Assign           => "=",
                Token::LeftBracket      => "(",
                Token::RightBracket     => ")",
                Token::LeftBrace        => "{",
                Token::RightBrace       => "}",
                Token::Semicolon        => ";",
                Token::Comma            => ",",
            } 
        )
    }
}