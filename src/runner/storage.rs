use super::object::Object;
use std::collections::HashMap;

struct Mem {
    vars: HashMap<String, Object>
}

impl Mem {

    fn new() -> Mem {
        Mem {
            vars: HashMap::new()
        }
    }

    fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            None => None,
            Some(object) => Some(object.clone())
        }
    }

    fn set(&mut self, name: String, value: Object) {
        self.vars.insert(name, value);
    }

    fn has(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

}

//core struct of lang's memory
//stack of memory frames
pub struct MemStack {
    stack: Vec<Mem>
}

impl MemStack {

    pub fn new() -> MemStack {
        MemStack {
            stack: vec![Mem::new()]
        }
    }

    pub fn scope(&mut self) -> &mut Mem {
        let stack_size = self.stack.len();
        &mut self.stack[stack_size - 1]
    }

    pub fn new_scope(&mut self) {
        self.stack.push(Mem::new());
    }

    pub fn leave_scope(&mut self) {
        self.stack.pop();
    }

    pub fn set_var(&mut self, name: String, value: Object) {
        let current_scope = self.scope();
        current_scope.set(name, value)
    }

    pub fn get_var(&mut self, name: String) -> Object {
        for mem in self.stack.iter().rev() {
            if let Some(object) = mem.get(&name) {
                return object;
            }
        }
        Object::Null
    }

    pub fn set_or_rewrite_var(&mut self, name: String, value: Object) {
        for mem in self.stack.iter_mut().rev() {
            if mem.has(&name) {
                mem.set(name.clone(), value.clone());
                return;
            }
        }
        self.set_var(name, value);
    }

}