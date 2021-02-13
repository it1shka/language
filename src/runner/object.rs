use crate::parser::ast::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i32),
    Float(f64),
    Str(String),
    Boolean(bool),
    Function(Vec<Expression>, Box<Statement>),
    Null
}

impl Object {

//type coersion

    pub fn as_int(&self) -> Self {
        match self {
            Object::Int(x) => Object::Int(*x),
            Object::Float(x) => Object::Int(*x as i32),
            Object::Str(x) => {
                let int = x.parse::<i32>();
                match int {
                    Ok(some_val) => Object::Int(some_val),
                    Err(_) => Object::Null
                }
            },
            Object::Boolean(x) => 
                if *x { Object::Int(1) } else {Object::Int(0)},
            Object::Function(_, _) => Object::Null,
            Object::Null => Object::Null
        }
    }

    pub fn as_float(&self) -> Self {
        match self {
            Object::Int(x) => Object::Float(*x as f64),
            Object::Float(x) => Object::Float(*x),
            Object::Str(x) => {
                let int = x.parse::<f64>();
                match int {
                    Ok(some_val) => Object::Float(some_val),
                    Err(_) => Object::Null
                }
            },
            Object::Boolean(x) => 
                if *x { Object::Float(1f64) } else {Object::Float(0f64)},
            Object::Function(_, _) => Object::Null,
            Object::Null => Object::Null
        }
    }

    pub fn as_str(&self) -> Self {
        match self {
            Object::Int(x) => Object::Str(format!("{}", *x)),
            Object::Float(x) => Object::Str(format!("{}", *x)),
            Object::Str(x) => self.clone(),
            Object::Boolean(x) =>
                if *x { Object::Str(String::from("true")) } else {Object::Str(String::from("false"))},
            Object::Function(_, _) => Object::Str(String::from("function")),
            Object::Null => Object::Str(String::from("null"))
        }
    }

    pub fn as_boolean(&self) -> Self {
        match self {
            Object::Int(x) => Object::Boolean(*x > 0),
            Object::Float(x) => Object::Boolean(*x > 0f64),
            Object::Str(x) => Object::Boolean(x == "true"),
            Object::Boolean(x) => self.clone(),
            Object::Function(_, _) => Object::Boolean(true),
            Object::Null => Object::Boolean(false)
        }
    }

    //............

    pub fn add(&self, other: &Self) -> Self {
        match self {

            Object::Int(left) => {
                let other = other.as_int();
                if let Object::Int(right) = other {
                    Object::Int(right + left)
                }
                else {
                    Object::Null
                }
            }

            Object::Float(left) => {
                let other = other.as_float();
                if let Object::Float(right) = other {
                    Object::Float(left + right)
                }
                else {
                    Object::Null
                }
            }

            Object::Str(left) => {
                let other = other.as_str();
                if let Object::Str(right) = other {
                    Object::Str(format!("{}{}", left, right))
                }
                else {
                    Object::Str(String::from("Null"))
                }
            }

            Object::Boolean(left) => {
                let other = other.as_int();
                if let Object::Int(right) = other {
                    Object::Int(if *left {1} else {0} + right)
                }
                else {
                    Object::Null
                }
            }

            Object::Function(_,_) => Object::Null,

            Object::Null => Object::Null            

        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        match self {

            Object::Int(left) => {
                let other = other.as_int();
                if let Object::Int(right) = other {
                    Object::Int(right - left)
                }
                else {
                    Object::Null
                }
            }

            Object::Float(left) => {
                let other = other.as_float();
                if let Object::Float(right) = other {
                    Object::Float(left - right)
                }
                else {
                    Object::Null
                }
            }

            Object::Str(left) => {
                let other = other.as_str();
                if let Object::Str(right) = other {
                    Object::Str(format!("{}{}", left, right))
                }
                else {
                    Object::Str(String::from("Null"))
                }
            }

            Object::Boolean(left) => {
                let other = other.as_int();
                if let Object::Int(right) = other {
                    Object::Int(if *left {1} else {0} + right)
                }
                else {
                    Object::Null
                }
            }

            Object::Function(_,_) => Object::Null,

            Object::Null => Object::Null            

        }
    }

}