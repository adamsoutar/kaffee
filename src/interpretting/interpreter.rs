use std::collections::HashMap;
use crate::parsing::parser;
use crate::parsing::ast_utils::*;
use crate::interpretting::interpreter_utils::*;
use crate::interpretting::variables::Variables;
use crate::interpretting::variables;

/*
    TODO: Instead of panicking, throw exceptions within the
          language which can be caught etc.
*/

pub struct Interpreter {
    pub ast: Vec<ASTNode>,
    pub vars: Variables
}

impl Interpreter {
    pub fn run (&mut self) {
        // TODO: Hoisting pass which puts functions etc.
        //       in the scopestack
        self.vars.new_scope();
        for i in 0..self.ast.len() {
            let node = self.ast[i].clone();
            self.eval_node(&node)
        }

        println!("\nAllocced values:");
        self.vars.print_allocced();
        self.vars.print_scopestack();
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
            let idx = self.vars.find_variable_index(id);
            if self.vars.alloced[idx].constant {
                panic!("Assignment to constant value \"{}\"", id);
            }

            let val = self.resolve_value(bin.right.as_ref());
            self.vars.alloced[idx].value = val;
        } else {
            panic!("Assignment to non-identifiers is not yet supported")
        }
    }

    pub fn define_variable (&mut self, dcl: &DeclarationProperties) {
        if let ASTNode::Identifier(id) = dcl.assignment.left.as_ref() {
            let val = self.resolve_value(dcl.assignment.right.as_ref());
            self.vars.alloc_in_scope(id, val, dcl.constant)
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

}

pub fn new (code: String) -> Interpreter {
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();

    Interpreter {
        ast,
        vars: variables::new()
    }
}
