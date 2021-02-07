
#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, PartialEq)]
pub enum Statement {
    List(Vec<Statement>),
    Break,
    Continue,
    Return(Expression),
    While(Expression, Box<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Function(String, Vec<Expression>, Box<Statement>),
    ExpressionStmt(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Primary(PrimaryExpression),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>)
}

#[derive(Debug, PartialEq)] 
pub enum BinaryOperator {
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
}

#[derive(Debug, PartialEq)]
pub enum PrimaryExpression {
    UnaryPlus(Box<PrimaryExpression>),
    UnaryMinus(Box<PrimaryExpression>),
    UnaryNot(Box<PrimaryExpression>),
    InBrackets(Box<Expression>),
    Ident(String),
    Float(f64),
    Int(i32),
    Str(String),
    Boolean(bool),
    FunctionCall(String, Vec<Expression>),
    Null
}