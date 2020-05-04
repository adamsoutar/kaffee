use crate::parsing::printer::print_ast_node;
use crate::parsing::parser;
use crate::parsing::ast_utils::*;
use crate::interpretting::interpreter_utils::*;
use crate::interpretting::variables::Variables;
use crate::interpretting::variables;
use crate::std_lib::functions::*;
use crate::std_lib::operators;

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
        println!("\nProgram output:");

        self.vars.new_scope();
        // Put funcs like println in the global scope
        self.load_std_lib();

        // User global scope is above std_lib global scope
        self.vars.new_scope();

        for i in 0..self.ast.len() {
            let node = self.ast[i].clone();
            self.eval_node(&node)
        }

        println!("\nAllocced values:");
        self.vars.print_allocced();
        println!("");
        self.vars.print_scopestack();
    }

    fn load_std_lib (&mut self) {
        for mapping in get_std_lib_mappings() {
            self.vars.alloc_in_scope(
                &mapping.name,
                KaffeeValue::NativeFunction(mapping.clone()),
                true)
        }
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
            ASTNode::FunctionCall(cp) => { self.eval_call(&cp); },
            ASTNode::FunctionDefinition(fd) => self.eval_function_definition(&fd),
            ASTNode::IfStatement(ifs) => self.eval_if_stmnt(&ifs),
            _ => {
                print_ast_node(node, 0);
                panic!("Unsupported executable node")
            }
        }
    }

    fn eval_if_stmnt(&mut self, ifp: &IfProperties) {
        // TODO: Proper scoping, and returns from within the block
        let cbool = self.resolve_node(ifp.check_exp.as_ref());
        if let KaffeeValue::Boolean(check) = cbool {
            if check {
                self.eval_node(ifp.body.as_ref());
            } else {
                if let Some(en) = &ifp.else_exp {
                    self.eval_node(en.as_ref());
                }
            }
        } else {
            // TODO: Value coercion
            panic!("If statement check doesn't resolve to a boolean")
        }
    }

    fn ast_func_to_value (&mut self, fd: &FunctionDefinitionProperties) -> KaffeeValue {
        KaffeeValue::Function(FunctionDefinition {
            args: fd.args.clone(),
            body: fd.body.clone()
        })
    }

    fn eval_function_definition (&mut self, fd: &FunctionDefinitionProperties) {
        let kv_fn = self.ast_func_to_value(fd);
        self.vars.alloc_in_scope(&fd.name, kv_fn, false);
    }

    // Returns the "return value" of the function
    fn eval_call (&mut self, cp: &CallProperties) -> KaffeeValue {
        let callee = self.resolve_node(cp.callee.as_ref());
        if let KaffeeValue::NativeFunction(nf) = callee {
            let rargs = cp.args.iter().map(|x| self.resolve_node(x)).collect();
            (nf.func)(rargs)
        } else if let KaffeeValue::Function(f) = callee {
            self.eval_userfn_call(cp, &f)
        } else {
            panic!("Called an uncallable value, eg. 3.14()");
        }
    }

    fn eval_userfn_call (&mut self, cp: &CallProperties, fd: &FunctionDefinition) -> KaffeeValue {
        self.vars.new_scope();

        // Allocate arguments to the block scope
        for i in 0..fd.args.len() {
            let val = self.resolve_node(&cp.args[i]);
            self.vars.alloc_in_scope(&fd.args[i], val, false);
        }

        let mut return_value = KaffeeValue::Null;
        for node in &fd.body {
            if let ASTNode::ReturnStatement(rs) = node {
                return_value = self.resolve_node(rs);
                break;
            } else {
                self.eval_node(node);
            }
        }

        // TODO: Garbage collection
        self.vars.pop_scope();

        return_value
    }

    fn assign_variable (&mut self, bin: &BinaryProperties) {
        // TODO: Assignment to non-identifiers
        if let ASTNode::Identifier(id) = bin.left.as_ref() {
            let idx = self.vars.find_variable_index(id);
            if self.vars.alloced[idx].constant {
                panic!("Assignment to constant value \"{}\"", id);
            }

            let val = self.resolve_node(bin.right.as_ref());
            self.vars.alloced[idx].value = val;
        } else {
            panic!("Assignment to non-identifiers is not yet supported")
        }
    }

    pub fn define_variable (&mut self, dcl: &DeclarationProperties) {
        if let ASTNode::Identifier(id) = dcl.assignment.left.as_ref() {
            let val = self.resolve_node(dcl.assignment.right.as_ref());
            self.vars.alloc_in_scope(id, val, dcl.constant)
        } else {
            panic!("Left side of a declaration isn't an identifier")
        }
    }

    fn resolve_node (&mut self, node: &ASTNode) -> KaffeeValue {
        // TODO: Resolving of function calls
        match node {
            ASTNode::String(st) => KaffeeValue::String(st.clone()),
            ASTNode::Number(n) => KaffeeValue::Number(n.clone()),
            ASTNode::Boolean(bl) => KaffeeValue::Boolean(bl.clone()),
            ASTNode::Null => KaffeeValue::Null,
            ASTNode::Identifier(id) => self.vars.resolve_identifier(id).clone(),
            ASTNode::BinaryNode(bn) => self.resolve_binary(&bn),
            ASTNode::ObjectLiteral(ov) => self.resolve_object_literal(ov),
            ASTNode::PropertyAccess(pa) => self.resolve_property_access(&pa),
            ASTNode::FunctionDefinition(fd) => self.ast_func_to_value(&fd),
            ASTNode::FunctionCall(cp) => self.eval_call(&cp),
            _ => {
                print_ast_node(node, 0);
                panic!("Unresolvable ASTNode value")
            }
        }
    }

    fn resolve_property_access (&mut self, pa: &AccessProperties) -> KaffeeValue {
        let lft = self.resolve_node(pa.object.as_ref());

        if let KaffeeValue::Object(obj) = lft {
            if let ASTNode::Identifier(key) = pa.property.as_ref() {
                let kstr = KaffeeValue::String(key.clone());

                return self.lookup_object_value(&obj, &kstr);
            } else {
                panic!("Property is not an identifier.")
            }
        } else {
            panic!("Property access on a non-object.")
        }
    }

    fn lookup_object_value (&mut self, obj: &ObjectValue, kv: &KaffeeValue) -> KaffeeValue {
        for i in 0..obj.keys.len() {
            let idx = obj.keys[i];
            let key = &self.vars.alloced[idx].value;

            if key == kv {
                let val = &self.vars.alloced[obj.values[i]].value;
                return val.clone();
            }
        }

        panic!("Key isn't present in object");
    }

    fn resolve_object_literal (&mut self, ov: &ObjectLiteralProperties) -> KaffeeValue {
        let mut keys = vec![];
        let mut values = vec![];

        // Alloc the keys as Kaffee strings
        for key in &ov.keys {
            let idx = self.vars.alloc_value(KaffeeValue::String(key.clone()), true);
            keys.push(idx);
        }

        // Alloc the values
        for val in &ov.values {
            // Resolve the value
            let res_val = self.resolve_node(val);
            // Alloc the value
            let idx = self.vars.alloc_value(res_val, false);
            values.push(idx);
        }

        KaffeeValue::Object(ObjectValue {
            keys,
            values
        })
    }

    fn resolve_binary (&mut self, bn: &BinaryProperties) -> KaffeeValue {
        let lft = self.resolve_node(bn.left.as_ref());
        let rgt = self.resolve_node(bn.right.as_ref());

        return operators::operator_handler(lft, &bn.operator, rgt);
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
