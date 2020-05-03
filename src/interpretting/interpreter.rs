use std::collections::HashMap;
use crate::parsing::parser;
use crate::parsing::ast_utils::*;
use crate::interpretting::interpreter_utils::*;

/*
    TODO: Instead of panicking, throw exceptions within the
          language which can be caught etc.
*/

pub struct Interpreter {
    pub ast: Vec<ASTNode>,
    pub alloced_values: Vec<AllocedValue>,
    pub scopestack: Vec<HashMap<String, usize>>
}

fn print_allocced (av: &Vec<AllocedValue>) {
    for i in 0..av.len() {
        let v = &av[i];
        println!("{} - {}", i,
            match &v.value {
                KaffeeValue::Number(n) => {
                    format!("Number: {}", n)
                },
                KaffeeValue::String(s) => {
                    format!("String: \"{}\"", s)
                },
                _ => String::from("TODO: This value type")
            }
        )
    }
}

fn print_scopestack (stack: &Vec<HashMap<String, usize>>) {
    for i in 0..stack.len() {
        let scope = &stack[i];
        println!("Stack frame {}:", i);
        for (ident, idx) in scope {
            println!(" - \"{}\" - {}", ident, idx);
        }
    }
}

impl Interpreter {
    pub fn run (&mut self) {
        // TODO: Hoisting pass which puts functions etc.
        //       in the scopestack
        self.new_scope();
        for i in 0..self.ast.len() {
            let node = self.ast[i].clone();
            self.eval_node(&node)
        }

        println!("\nAllocced values:");
        print_allocced(&self.alloced_values);
        print_scopestack(&self.scopestack);
    }

    fn eval_node (&mut self, node: &ASTNode) {
        match node {
            ASTNode::BlockStatement(bs) => {
                for n in bs {
                    self.eval_node(n)
                }
            },
            ASTNode::Declaration(dcl) => self.define_variable(dcl),
            ASTNode::Assignment(asn) => self.assign_variable(asn),
            _ => {
                panic!("Unsupported executable node")
            }
        }
    }

    fn assign_variable (&mut self, bin: &BinaryProperties) {
        // TODO: Assignment to non-identifiers
        if let ASTNode::Identifier(id) = bin.left.as_ref() {
            let idx = self.find_variable_index(id);
            if self.alloced_values[idx].constant {
                panic!("Assignment to constant value \"{}\"", id);
            }

            let val = self.resolve_value(bin.right.as_ref());
            self.alloced_values[idx].value = val;
        } else {
            panic!("Assignment to non-identifiers is not yet supported")
        }
    }

    fn find_variable_index (&mut self, name: &String) -> usize {
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

    fn define_variable (&mut self, dcl: &DeclarationProperties) {
        if let ASTNode::Identifier(id) = dcl.assignment.left.as_ref() {
            let val = self.resolve_value(dcl.assignment.right.as_ref());
            self.alloc_in_scope(id, val, dcl.constant)
        } else {
            panic!("Left side of a declaration isn't an identifier")
        }
    }

    fn resolve_value (&mut self, node: &ASTNode) -> KaffeeValue {
        match node {
            ASTNode::String(st) => KaffeeValue::String(st.clone()),
            ASTNode::Number(n) => KaffeeValue::Number(n.clone()),
            _ => panic!("Unresolvable ASTNode value")
        }
    }

    fn new_scope (&mut self) {
        self.scopestack.push(HashMap::new())
    }

    fn alloc_in_scope (&mut self, identifier: &String, value: KaffeeValue, constant: bool) {
        let idx = self.alloced_values.len();
        self.alloc_value(value, constant);
        self.add_to_scope(identifier.clone(), idx);
    }

    fn add_to_scope (&mut self, identifier: String, alloc_index: usize) {
        let idx = self.scopestack.len() - 1;
        self.scopestack[idx].insert(identifier, alloc_index);
    }

    fn alloc_value (&mut self, value: KaffeeValue, constant: bool) {
        self.alloced_values.push(AllocedValue {
            value,
            constant,
            ref_count: 0
        })
    }
}

pub fn new (code: String) -> Interpreter {
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();

    Interpreter {
        ast,
        alloced_values: vec![],
        scopestack: vec![]
    }
}
