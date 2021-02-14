
#[derive(Debug, PartialEq, Clone)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    List(Vec<Statement>),
    Break,
    Continue,
    Return(Expression),
    Echo(Expression),
    While(Expression, Box<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    FunctionDecl(String, Vec<Expression>, Box<Statement>),
    ExpressionStmt(Expression)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Primary(PrimaryExpression),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>)
}

#[derive(Debug, PartialEq, Clone)] 
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

#[derive(Debug, PartialEq, Clone)]
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
    Call(Box<PrimaryExpression>, Vec<Expression>),
    Null
}