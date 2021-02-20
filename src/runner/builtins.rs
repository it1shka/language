use super::object::Object;
use std::io::stdin;

pub fn echo(object: Object) {
    println!("{}", match object.to_str() {Object::Str(obj_str) => obj_str, _ => "".to_string()})
}

pub fn print(args: Vec<Object>) -> Result<Object, String> {
    args.iter().for_each(|x| echo(x.clone()));
    Ok(Object::Null)
}

pub fn int(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        None => Err("Expected argument in builtin 'int'!".to_string()),
        Some(val) => Ok(val.to_int())
    }
}

pub fn float(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        None => Err("Expected argument in builtin 'float'!".to_string()),
        Some(val) => Ok(val.to_float())
    }
}

pub fn bool_(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        None => Err("Expected argument in builtin 'bool'!".to_string()),
        Some(val) => Ok(val.to_bool())
    }
}

pub fn string(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        None => Err("Expected argument in builtin 'bool'!".to_string()),
        Some(val) => Ok(val.to_str())
    }
}

pub fn object_typeof(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        None => Err("Expected argument in builtin 'typeof'!".to_string()),
        Some(val) => Ok(match val {
            Object::Int(_) => Object::Str("int".to_string()),
            Object::Float(_) => Object::Str("float".to_string()),
            Object::Boolean(_) => Object::Str("bool".to_string()),
            Object::Function(_,_) | Object::BuiltIn(_) 
                => Object::Str("function".to_string()),
            Object::Null => Object::Str("null".to_string()),
            Object::Str(_) => Object::Str("string".to_string()),
        })
    }
}

pub fn input(args: Vec<Object>) -> Result<Object, String> {
    print(args)?;
    let mut user_input = String::new();
    match stdin().read_line(&mut user_input) {
        Ok(_) => { 
            Ok(Object::Str(user_input.trim_end().to_string())) 
        },
        Err(_) => Err("Unexpected error while io reading!".to_string())
    }
}