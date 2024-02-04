use std::{collections::HashMap, rc::Rc};

use crate::object::object::Object;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Rc<Box<dyn Object>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            store: HashMap::new(),
        };
    }

    pub fn get(&self, key: String) -> Option<Rc<Box<dyn Object>>> {
        match self.store.get(&key) {
            Some(val) => return Some(val.clone()),
            None => {
                return None;
            }
        }
    }

    pub fn set(&mut self, key: String, value: Rc<Box<dyn Object>>) -> Option<Rc<Box<dyn Object>>> {
        self.store.insert(key, value.clone());
        return Some(value);
    }
}
