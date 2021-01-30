
#[derive(Debug, PartialEq)]
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