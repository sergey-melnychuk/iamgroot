use std::collections::HashMap;

use crate::binding;

#[derive(Debug, Clone)]
pub struct Cache {
    pub data: HashMap<String, binding::Binding>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, binding: binding::Binding) {
        if let Some(existing) = self.data.get(&name) {
            if existing != &binding {
                eprintln!("Conflicting name: '{name}'!");
                eprintln!("existing: {existing:?}\ninserted: {binding:?}\n");
            }
        }
        self.data.insert(name, binding);
    }

    pub fn overwrite(&mut self, name: String, binding: binding::Binding) {
        self.data.insert(name, binding);
    }

    pub fn get(&self, name: &str) -> Option<&binding::Binding> {
        self.data.get(name)
    }
}
