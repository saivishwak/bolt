use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::object::Object;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Rc<Box<dyn Object>>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }));
    }

    pub fn get(&self, key: String) -> Option<Rc<Box<dyn Object>>> {
        match self.store.get(&key) {
            Some(val) => return Some(val.clone()),
            None => {
                if let Some(outer_env) = self.outer.as_ref() {
                    return outer_env.borrow().get(key);
                }
                return None;
            }
        }
    }

    pub fn set(&mut self, key: String, value: Rc<Box<dyn Object>>) -> Option<Rc<Box<dyn Object>>> {
        self.store.insert(key, value.clone());
        return Some(value);
    }
}

pub fn new_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
    let env = Environment::new();
    env.borrow_mut().outer = Some(outer);
    return env;
}
