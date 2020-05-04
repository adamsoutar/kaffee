use std::collections::HashMap;
use crate::interpretting::interpreter_utils::*;

pub struct Variables {
    pub alloced: Vec<AllocedValue>,
    pub scopestack: Vec<HashMap<String, usize>>
}

impl Variables {
    pub fn resolve_identifier (&mut self, name: &String) -> &KaffeeValue {
        let idx = self.find_variable_index(name);
        &self.alloced[idx].value
    }

    pub fn find_variable_index (&mut self, name: &String) -> usize {
        let max = self.scopestack.len() - 1;
        for i in (0..=max).rev() {
            let hm = &self.scopestack[i];
            match hm.get(name) {
                Some(idx) => return idx.clone(),
                None => continue
            }
        }
        panic!("Unresolved identifier \"{}\"", name)
    }

    pub fn new_scope (&mut self) {
        self.scopestack.push(HashMap::new())
    }

    pub fn alloc_in_scope (&mut self, identifier: &String, value: KaffeeValue, constant: bool) {
        let idx = self.alloced.len();
        self.alloc_value(value, constant);
        self.add_to_scope(identifier.clone(), idx);
    }

    pub fn add_to_scope (&mut self, identifier: String, alloc_index: usize) {
        let idx = self.scopestack.len() - 1;
        self.scopestack[idx].insert(identifier, alloc_index);
    }

    pub fn alloc_value (&mut self, value: KaffeeValue, constant: bool) {
        self.alloced.push(AllocedValue {
            value,
            constant,
            ref_count: 0
        })
    }

    pub fn print_allocced (&self) {
        for i in 0..self.alloced.len() {
            let v = &self.alloced[i];
            println!("{} - {}", i,
                match &v.value {
                    KaffeeValue::Number(n) => {
                        format!("Number: {}", n)
                    },
                    KaffeeValue::String(s) => {
                        format!("String: \"{}\"", s)
                    },
                    KaffeeValue::NativeFunction(nm) => {
                        format!("NativeFunction: \"{}\" - {} args", nm.name, nm.arg_count)
                    }
                    _ => String::from("TODO: This value type")
                }
            )
        }
    }

    pub fn print_scopestack (&self) {
        for i in 0..self.scopestack.len() {
            let scope = &self.scopestack[i];
            println!("Stack frame {}:", i);
            for (ident, idx) in scope {
                println!(" - \"{}\" - {}", ident, idx);
            }
        }
    }
}

pub fn new () -> Variables {
    Variables {
        alloced: vec![],
        scopestack: vec![]
    }
}
