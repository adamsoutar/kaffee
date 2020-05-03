use crate::parsing::parser;
use crate::parsing::ast_utils::*;
use crate::interpretting::interpreter_utils::*;

pub struct Interpreter {
    pub ast: Vec<ASTNode>,
    pub alloced_values: Vec<AllocedObject>,
    pub scopestack: Vec<HashMap>
}

impl Interpreter {
    pub fn run (&mut self) {
        // TODO: Hoisting pass which puts functions etc.
        //       in the scopestack
        self.new_scope();
        for node in ast {
            self.eval_node(node);
        }
    }

    fn eval_node (&mut self, node: ASTNode) {
        match node => {
            ASTNode::BlockStatement(bs) => {
                for n in bs {
                    self.eval_node(n)
                }
            },
            ASTNode::Declaration(dcl) => self.define_variable(dcl),
            _ => {
                panic!("Unsupported executable node")
            }
        }
    }

    fn define_variable (&mut self, dcl: DeclarationProperties) {
        if let ASTNode::Identifier(id) = dcl.assignment.left {
            let val = self.resolve_value(dcl.assignment.right);
            self.alloc_in_scope(id, val, dcl.constant)
        } else {
            panic!("Left side of a declaration isn't an identifier")
        }
    }

    fn resolve_value (&mut self, node: ASTNode) -> KaffeeValue {

    }

    fn new_scope (&mut self) {
        self.scopestack.push(HashMap::new())
    }

    fn alloc_in_scope (&mut self, identifier: String, value: KaffeeValue, constant: bool) {
        let idx = self.alloced_values.len();
        self.alloc_value(value, constant);
        self.add_to_scope(identifier, idx);
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
