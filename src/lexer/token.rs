
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Ident(String),
    Int(i32),
    Float(f64),
    Str(String),

    True,
    False,
    Null,

    //keywords
    Break,
    Continue,
    Return,
    While,
    If,
    Else,
    Function,

    //вывод
    Echo,
    
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

    //special operator
    StrAdd, // .

    //punctuation
    LeftBracket,    // (
    RightBracket,   // )
    LeftBrace,      // {
    RightBrace,     // }
    Semicolon,      // ;
    Comma,          // ,
}


impl Token {
    pub fn is_ident(&self) -> bool {
        if let Token::Ident(_) = self {true}
        else {false}
    }

    pub fn is_int(&self) -> bool {
        if let Token::Int(_) = self {true}
        else {false}
    }

    pub fn is_float(&self) -> bool {
        if let Token::Float(_) = self {true}
        else {false}
    }

    pub fn is_str(&self) -> bool {
        if let Token::Str(_) = self {true}
        else {false}
    }

    pub fn is_eof(&self) -> bool {
        if let Token::EOF = self {true}
        else {false}
    }
}
