use crate::parsing::printer::print_ast_node;
use crate::parsing::parser;
use crate::parsing::ast_utils::*;
use crate::interpretting::interpreter_utils::*;
use crate::interpretting::variables::Variables;
use crate::interpretting::variables;
use crate::std_lib::functions::*;
use crate::std_lib::operators;
use crate::interpretting::garbage_collector;
use std::time::Instant;

/*
    TODO: Instead of panicking, throw exceptions within the
          language which can be caught etc.
*/
// TODO: So many methods unnecessarily take &mut self references

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

        let now = Instant::now();

        for i in 0..self.ast.len() {
            let node = self.ast[i].clone();
            self.eval_node(&node);
        }

        println!("\nProgram execution time: {}ms", now.elapsed().as_millis());

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

    fn gc_collect (&mut self) {
        // Garbage collection
        garbage_collector::gc_collect(&mut self.vars.alloced, &mut self.vars.scopestack)
    }

    // Returns (Did it return early?, what did it return)
    fn eval_node (&mut self, node: &ASTNode) -> (BreakType, KaffeeValue) {
        match node {
            ASTNode::BlockStatement(bs) => {
                self.vars.new_scope();
                for n in bs {
                    // If we eval a sub-block and it returns, we need to return, too
                    let (bt, kv) = self.eval_node(n);
                    if bt != BreakType::None {
                        // We still need to GC collect if we return early
                        self.vars.pop_scope();
                        self.gc_collect();
                        return (bt, kv)
                    }
                }
                // TODO: Remove this repetition
                self.vars.pop_scope();
                self.gc_collect();
            },
            ASTNode::Declaration(dcl) => self.define_variable(dcl),
            ASTNode::Assignment(asn) => self.assign_variable(asn),
            ASTNode::FunctionCall(cp) => { self.eval_call(&cp); },
            ASTNode::FunctionDefinition(fd) => self.eval_function_definition(&fd),
            ASTNode::IfStatement(ifs) => { return self.eval_if_stmnt(&ifs) },
            ASTNode::ReturnStatement(rs) => {
                return (BreakType::Return, self.resolve_node(rs.as_ref()))
            },
            ASTNode::ContinueStatement => {
                return (BreakType::Continue, KaffeeValue::Null)
            },
            ASTNode::BreakStatement => {
                return (BreakType::Break, KaffeeValue::Null)
            },
            ASTNode::WhileLoop(wp) => { return self.eval_while_loop(&wp) },
            _ => {
                print_ast_node(node, 0);
                panic!("Unsupported executable node")
            }
        }

        (BreakType::None, KaffeeValue::Null)
    }

    fn eval_while_loop (&mut self, wp: &WhileProperties) -> (BreakType, KaffeeValue) {
        while self.node_as_bool(wp.check.as_ref()) {
            let (b_type, ret_val) = self.eval_node(wp.body.as_ref());
            if b_type == BreakType::Return {
                return (b_type, ret_val)
            }
            if b_type == BreakType::Break {
                break;
            }
        }

        (BreakType::None, KaffeeValue::Null)
    }

    fn node_as_bool (&mut self, node: &ASTNode) -> bool {
        let res = self.resolve_node(node);
        // Truthy coercion
        match res {
            KaffeeValue::Boolean(bl) => bl,
            KaffeeValue::Null => false,
            _ => true
        }
    }

    fn eval_if_stmnt(&mut self, ifp: &IfProperties) -> (BreakType, KaffeeValue) {
        let check = self.node_as_bool(ifp.check_exp.as_ref());
        if check {
            return self.eval_node(ifp.body.as_ref());
        } else {
            if let Some(en) = &ifp.else_exp {
                return self.eval_node(en.as_ref());
            } else {
                return (BreakType::None, KaffeeValue::Null)
            }
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
            let rargs: Vec<KaffeeValue> = cp.args.iter().map(|x| self.resolve_node(x)).collect();

            if rargs.len() != nf.arg_count {
                panic!("{} takes {} arguments, but {} were supplied", nf.name, nf.arg_count, rargs.len())
            }

            (nf.func)(rargs, &mut self.vars)
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

        let (_, ret_val) = self.eval_node(&ASTNode::BlockStatement(fd.body.clone()));

        self.vars.pop_scope();
        // This collects the argument variables
        self.gc_collect();

        ret_val
    }

    fn assign_variable (&mut self, bin: &BinaryProperties) {
        if bin.operator != "=" { unreachable!() }

        let (key_exists, val_idx) = self.resolve_assignment_target(bin.left.as_ref());

        if !key_exists {
            self.handle_insertion(bin);
            return
        }

        if self.vars.alloced[&val_idx].constant {
            panic!("Assignment to constant value")
        }

        let val = self.resolve_node(bin.right.as_ref());
        self.vars.alloced.get_mut(&val_idx).unwrap().value = val;
    }

    fn handle_insertion (&mut self, bin: &BinaryProperties) {
        let pa = match bin.left.as_ref() {
            ASTNode::PropertyAccess(x) => x,
            _ => unreachable!()
        };

        // Can't do implicit assignment with an array
        match self.resolve_node(pa.object.as_ref()) {
            KaffeeValue::Object(_) => {},
            _ => panic!("Attempted to access a non-existent key in an array")
        }

        let (exists, obj_idx) = self.resolve_assignment_target(pa.object.as_ref());
        if !exists {
            // TODO: Reword this error
            panic!("Assignment with more than one level of non-existant key")
        }

        let key = self.resolve_node(pa.property.as_ref());

        let value = self.resolve_node(bin.right.as_ref());

        self.vars.insert_into_object(key, value, obj_idx);
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
            ASTNode::ArrayLiteral(items) => self.resolve_array_literal(&items),
            _ => {
                print_ast_node(node, 0);
                panic!("Unresolvable ASTNode value")
            }
        }
    }

    fn resolve_array_literal (&mut self, items: &Vec<ASTNode>) -> KaffeeValue {
        let mut idxs = vec![];
        for i in items {
            let val = self.resolve_node(i);
            idxs.push(self.vars.alloc_value(val, false))
        }

        KaffeeValue::Array(idxs)
    }

    // Returns (key exists (for property access), alloc idx)
    fn resolve_assignment_target (&mut self, node: &ASTNode) -> (bool, usize) {
        match node {
            ASTNode::Identifier(id) => (true, self.vars.find_variable_index(&id)),
            ASTNode::PropertyAccess(pa) => self.lookup_property_access(pa),
            _ => {
                print_ast_node(node, 0);
                panic!("Can't assign to this type")
            }
        }
    }

    fn lookup_property_access (&mut self, pa: &AccessProperties) -> (bool, usize) {
        // TODO: std_lib wrapper for prop. access on primitives like String for chars
        let lft = self.resolve_node(pa.object.as_ref());
        let key = self.resolve_node(pa.property.as_ref());

        if let KaffeeValue::Object(obj) = lft {
            return self.vars.lookup_object_value_index(&obj, &key)
        } else if let KaffeeValue::Array(items) = lft {
            return self.vars.lookup_array_value_index(&items, &key)
        }

        panic!("Property access isn't supported on that type")
    }

    fn resolve_property_access (&mut self, pa: &AccessProperties) -> KaffeeValue {
        let (exists, idx) = self.lookup_property_access(pa);

        if !exists {
            panic!("Property access key doesn't exist")
        }

        self.vars.alloced[&idx].value.clone()
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
