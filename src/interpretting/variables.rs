use std::collections::HashMap;
use crate::interpretting::interpreter_utils::*;

// TODO: Objects are shallow copied
//       This isn't right.

pub struct Variables {
    // TODO: Constant should be in the scopestack?
    pub alloced: HashMap<usize, AllocedValue>,
    pub alloc_index: usize,
    pub scopestack: Vec<HashMap<String, usize>>
}

impl Variables {
    pub fn resolve_identifier (&mut self, name: &String) -> &KaffeeValue {
        let idx = self.find_variable_index(name);

        &self.alloced[&idx].value
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

    pub fn pop_scope (&mut self) {
        self.scopestack.pop();
    }

    pub fn alloc_in_scope (&mut self, identifier: &String, value: KaffeeValue, constant: bool) {
        let top_scope = self.scopestack.len() - 1;
        if self.scopestack[top_scope].contains_key(identifier) {
            panic!("Attempt to shadow identifier \"{}\" within the same scope!
(You declared a variable with a conflicting name)", identifier)
        }

        let idx = self.alloc_value(value, constant);
        self.add_to_scope(identifier.clone(), idx);
    }

    pub fn add_to_scope (&mut self, identifier: String, alloc_index: usize) {
        let idx = self.scopestack.len() - 1;
        self.scopestack[idx].insert(identifier, alloc_index);
    }

    pub fn alloc_value (&mut self, value: KaffeeValue, constant: bool) -> usize {
        self.alloced.insert(self.alloc_index, AllocedValue {
            value,
            constant
        });
        self.alloc_index += 1;

        self.alloc_index - 1
    }

    pub fn print_allocced (&self) {
        for (i, v) in &self.alloced {
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
                    },
                    KaffeeValue::Function(f) => {
                        format!("Function: {} args, {} body nodes", f.args.len(), f.body.len())
                    },
                    KaffeeValue::Object(ov) => {
                        let mut st = format!("Object:");
                        for i in 0..ov.keys.len() {
                            st = format!("{}\n    - {} - {}", st, ov.keys[i], ov.values[i])
                        }
                        st
                    },
                    KaffeeValue::Array(it) => {
                        let mut st = format!("Array:");
                        for i in it {
                            st = format!("{}\n    - {}", st, i);
                        }
                        st
                    },
                    KaffeeValue::Boolean(bl) => {
                        format!("Boolean: {}", bl)
                    },
                    KaffeeValue::Null => {
                        format!("Null")
                    }
                }
            )
        }
    }

    pub fn print_scopestack (&self) {
        for i in 0..self.scopestack.len() {
            let scope = &self.scopestack[i];
            println!("Scope frame {}:", i);
            for (ident, idx) in scope {
                println!(" - \"{}\" - {}", ident, idx);
            }
        }
    }

    pub fn lookup_object_value_index (&self, obj: &ObjectValue, kv: &KaffeeValue) -> (bool, usize) {
        for i in 0..obj.keys.len() {
            let idx = obj.keys[i];
            let key = &self.alloced[&idx].value;

            if key == kv {
                return (true, obj.values[i])
            }
        }
        (false, 0)
    }

    pub fn lookup_array_value_index (&self, arr: &Vec<usize>, kv: &KaffeeValue) -> (bool, usize) {
        if let KaffeeValue::Number(n) = kv {
            // Can't cast a negative num to usize
            if n < &0. { return (false, 0) }
            // Can't use a non-integer to index array
            if n % 1. != 0. { return (false, 0) }

            let idx = n.clone() as usize;
            if idx >= arr.len() { return (false, 0) }

            return (true, arr[idx])
        }

        (false, 0)
    }

    pub fn insert_into_object (&mut self, key: KaffeeValue, value: KaffeeValue, obj_idx: usize) {
        // Alloc
        let ki = self.alloc_value(key, false);
        let vi = self.alloc_value(value, false);

        let obj_val = self.alloced.get_mut(&obj_idx).unwrap();
        let obj = match &mut obj_val.value {
            KaffeeValue::Object(x) => x,
            _ => unreachable!()
        };

        // Add the mapping
        obj.keys.push(ki);
        obj.values.push(vi);
    }
}

pub fn new () -> Variables {
    Variables {
        alloced: HashMap::new(),
        alloc_index: 0,
        scopestack: vec![]
    }
}
