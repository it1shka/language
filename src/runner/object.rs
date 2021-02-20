use crate::parser::ast::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i32),
    Float(f64),
    Str(String),
    Boolean(bool),
    Function(Vec<String>, Box<Statement>),
    BuiltIn(fn(Vec<Object>) -> Result<Object, String>),
    Null
}

impl Object {
    pub fn to_int(&self) -> Object {
        match self {
            Object::Int(x) => Object::Int(*x),
            Object::Float(x) => Object::Int(*x as i32),
            Object::Str(x) => match x.parse::<i32>() {
                Ok(integer) => Object::Int(integer),
                Err(_) => Object::Null
            },
            Object::Boolean(x) => Object::Int(*x as i32),
            Object::Function(_, _) => Object::Null,
            Object::Null => Object::Null,
            Object::BuiltIn(_) => Object::Null
        }
    }

    pub fn to_float(&self) -> Object {
        match self {
            Object::Int(x) => Object::Float(*x as f64),
            Object::Float(x) => Object::Float(*x),
            Object::Str(x) => match x.parse::<f64>() {
                Ok(floating) => Object::Float(floating),
                Err(_) => Object::Null
            },
            Object::Boolean(x) => Object::Float(*x as i32 as f64),
            Object::Function(_, _) => Object::Null,
            Object::Null => Object::Null,
            Object::BuiltIn(_) => Object::Null
        }
    }

    pub fn to_str(&self) -> Object {
        match self {
            Object::Int(x) => Object::Str(x.to_string()),
            Object::Float(x) => Object::Str(x.to_string()),
            Object::Str(x) => Object::Str(x.clone()),
            Object::Boolean(x) => Object::Str(String::from(if *x {"true"} else {"false"})),
            Object::Function(_,_) => Object::Str(String::from("function")),
            Object::Null => Object::Str(String::from("null")),
            Object::BuiltIn(_) => Object::Str(String::from("builtin function"))
        }
    }

    pub fn to_bool(&self) -> Object {
        match self {
            Object::Int(x) => Object::Boolean(*x > 0),
            Object::Float(x) => Object::Boolean(*x > 0f64),
            Object::Str(x) => Object::Boolean(&x[..] == "true"),
            Object::Boolean(x) => Object::Boolean(*x),
            Object::Function(_,_) => Object::Null,
            Object::Null => Object::Boolean(false),
            Object::BuiltIn(_) => Object::Null
        }
    }

    //operators

    pub fn add(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left + right),
            (Object::Float(left), Object::Float(right)) => Object::Float(left + right),
            (Object::Int(left), Object::Float(right)) => Object::Float(*left as f64 + right),
            (Object::Float(left), Object::Int(right)) => Object::Float(left + *right as f64),
            (Object::Str(left), Object::Str(right)) => Object::Str(format!("{}{}", left, right)),
            _ => Object::Null
        }
    }

    pub fn sub(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left - right),
            (Object::Float(left), Object::Float(right)) => Object::Float(left - right),
            (Object::Int(left), Object::Float(right)) => Object::Float(*left as f64 - right),
            (Object::Float(left), Object::Int(right)) => Object::Float(left - *right as f64),
            _ => Object::Null
        }
    }

    pub fn str_add(&self, other: &Object) -> Object {
        self.to_str().add(&other.to_str())
    }

    pub fn mul(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left * right),
            (Object::Float(left), Object::Float(right)) => Object::Float(left * right),
            (Object::Int(left), Object::Float(right)) => Object::Float(*left as f64 * right),
            (Object::Float(left), Object::Int(right)) => Object::Float(left * *right as f64),
            (Object::Int(left), Object::Str(right)) => Object::Str(mul_str(*left, right)),
            (Object::Str(left), Object::Int(right)) => Object::Str(mul_str(*right, left)),
            _ => Object::Null
        }
    }

    pub fn div(&self, other: &Object) -> Object {
        match (self, other) {
            //(Object::Int(left), Object::Int(right)) => Object::Int(left / right),
            (Object::Int(left), Object::Int(right)) => Object::Float(*left as f64 / *right as f64),
            (Object::Float(left), Object::Float(right)) => Object::Float(left / right),
            (Object::Int(left), Object::Float(right)) => Object::Float(*left as f64 / right),
            (Object::Float(left), Object::Int(right)) => Object::Float(left / *right as f64),

            _ => Object::Null
        }
    }

    pub fn mod_(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left % right),
            (Object::Float(left), Object::Float(right)) => Object::Float(left % right),
            (Object::Int(left), Object::Float(right)) => Object::Float(*left as f64 % right),
            (Object::Float(left), Object::Int(right)) => Object::Float(left % *right as f64),

            _ => Object::Null
        }
    }

    pub fn equal(&self, other: &Object) -> Object {
        Object::Boolean(self == other)
    }

    pub fn not_equal(&self, other: &Object) -> Object {
        Object::Boolean(self != other)
    }

    pub fn greater(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Boolean(left > right),
            (Object::Float(left), Object::Float(right)) => Object::Boolean(left > right),
            (Object::Int(left), Object::Float(right)) => Object::Boolean(*left as f64 > *right),
            (Object::Float(left), Object::Int(right)) => Object::Boolean(*left > *right as f64),
            (Object::Str(left), Object::Str(right)) => Object::Boolean(left > right),
            _ => Object::Null
        }
    }

    pub fn less(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Boolean(left < right),
            (Object::Float(left), Object::Float(right)) => Object::Boolean(left < right),
            (Object::Int(left), Object::Float(right)) => Object::Boolean((*left as f64) < *right),
            (Object::Float(left), Object::Int(right)) => Object::Boolean(*left < *right as f64),
            (Object::Str(left), Object::Str(right)) => Object::Boolean(left < right),
            _ => Object::Null
        }
    }

    pub fn greater_or_equal(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Boolean(left >= right),
            (Object::Float(left), Object::Float(right)) => Object::Boolean(left >= right),
            (Object::Int(left), Object::Float(right)) => Object::Boolean(*left as f64 >= *right),
            (Object::Float(left), Object::Int(right)) => Object::Boolean(*left >= *right as f64),
            (Object::Str(left), Object::Str(right)) => Object::Boolean(left >= right),
            _ => Object::Null
        }
    }

    pub fn less_or_equal(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Int(left), Object::Int(right)) => Object::Boolean(left <= right),
            (Object::Float(left), Object::Float(right)) => Object::Boolean(left <= right),
            (Object::Int(left), Object::Float(right)) => Object::Boolean((*left as f64) <= *right),
            (Object::Float(left), Object::Int(right)) => Object::Boolean(*left <= *right as f64),
            (Object::Str(left), Object::Str(right)) => Object::Boolean(left <= right),
            _ => Object::Null
        }
    }

    pub fn and(&self, other: &Object) -> Object {
        match (self, other){
            (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(*left && *right),
            _ => Object::Null
        }
    }

    pub fn or(&self, other: &Object) -> Object {
        match (self, other) {
            (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(*left || *right),
            (Object::Null, _) => other.clone(),
            (_, Object::Null) => self.clone(),
            _ => Object::Null
        } 
    }

    pub fn not(&self) -> Object {
        match self {
            Object::Boolean(left) => Object::Boolean(!left),
            _ => Object::Null
        }
    }

    pub fn unary_plus(&self) -> Object {
        let mut val = self.to_int();
        if let Object::Null = val {
            val = self.to_float();
        }
        val
    }

    pub fn unary_minus(&self) -> Object {
        match self {
            Object::Float(left) => Object::Float(-*left),
            Object::Int(left) => Object::Int(-*left),
            _ => Object::Null
        }
    }

}

fn mul_str(mul: i32, string: &str) -> String {
    let mut val = String::new();
    for _ in 0..mul {
        val.push_str(string)
    }
    val
}