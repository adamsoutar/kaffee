// Prints the AST for debug
// not, like... inkjet.
use crate::parsing::ast_utils::*;
use crate::parsing::tokens::*;

pub fn print_token (tk: &Token) {
    match tk {
        Token::Number(nm) => {
            println!("Number: {}", nm)
        },
        Token::String(st) => {
            println!("String: \"{}\"", st)
        },
        Token::Keyword(kw) => {
            println!("Keyword: {}", kw)
        },
        Token::Identifier(id) => {
            println!("Identifier: {}", id)
        },
        Token::Operator(op) => {
            println!("Operator: {}", op)
        },
        Token::Punctuation(pnc) => {
            println!("Punctuation: {}", pnc)
        },
        Token::Boolean(bl) => {
            println!("Boolean: {}", bl)
        },
        Token::Null => {
            println!("Null literal")
        }
    }
}

fn print_at_depth (s: String, depth: i32) {
    let mut str = String::from("");
    for _ in 0..depth * 4 {
        str += &String::from(" ");
    }
    str += &s;
    println!("{}", str);
}

fn print_binary (bin: &BinaryProperties, depth: i32) {
    print_at_depth(String::from("Left:"), depth);
    print_ast_node(bin.left.as_ref(), depth + 1);
    print_at_depth(String::from("Operator: ") + &bin.operator, depth);
    print_at_depth(String::from("Right: "), depth);
    print_ast_node(bin.right.as_ref(), depth + 1);
}

fn print_object_literal (obj: &ObjectLiteralProperties, depth: i32) {
    print_at_depth(String::from("Object literal:"), depth);
    for i in 0..obj.keys.len() {
        let key = &obj.keys[i];
        let val = &obj.values[i];
        print_at_depth(format!("Key: \"{}\", Value:", key), depth + 1);
        print_ast_node(val, depth + 2);
    }
}

pub fn print_ast_node (node: &ASTNode, depth: i32) {
    match node {
        ASTNode::Declaration(dec) => {
            print_at_depth(format!("Declaration, constant: {}", dec.constant), depth);
            print_binary(&dec.assignment, depth + 1);
        },
        ASTNode::Assignment(asn) => {
            print_at_depth(String::from("Assignment"), depth);
            print_binary(&asn, depth + 1);
        },
        ASTNode::String(st) => {
            print_at_depth(format!("String: \"{}\"", st), depth);
        },
        ASTNode::Number(n) => {
            print_at_depth(format!("Number: {}", n), depth);
        },
        ASTNode::Identifier(id) => {
            print_at_depth(format!("Identifier: \"{}\"", id), depth);
        },
        ASTNode::ObjectLiteral(obj) => {
            print_object_literal(&obj, depth);
        },
        ASTNode::BlockStatement(bs) => {
            print_at_depth(String::from("Block statement:"), depth);
            for stmt in bs {
                print_ast_node(stmt, depth + 1);
            }
        },
        ASTNode::BinaryNode(bn) => {
            print_at_depth(String::from("Binary node:"), depth);
            print_binary(&bn, depth + 1);
        },
        ASTNode::FunctionCall(fnc) => {
            print_at_depth(String::from("Function call:"), depth);
            print_at_depth(String::from("Callee:"), depth + 1);
            print_ast_node(fnc.callee.as_ref(), depth + 2);
            print_at_depth(String::from("Args:"), depth + 1);
            for arg in &fnc.args {
                print_ast_node(arg, depth + 2);
            }
        },
        ASTNode::PropertyAccess(prp) => {
            print_at_depth(String::from("Property access:"), depth);
            print_at_depth(String::from("Object:"), depth + 1);
            print_ast_node(prp.object.as_ref(), depth + 2);
            print_at_depth(String::from("Property:"), depth + 1);
            print_ast_node(prp.property.as_ref(), depth + 2);
        },
        ASTNode::FunctionDefinition(fd) => {
            print_at_depth(format!("Function definition - \"{}\":", fd.name), depth);
            print_at_depth(String::from("Args:"), depth + 1);
            for arg in &fd.args {
                print_at_depth(format!("{}", arg), depth + 2)
            }
            print_at_depth(String::from("Body:"), depth + 1);
            for node in &fd.body {
                print_ast_node(node, depth + 2)
            }
        },
        ASTNode::Boolean(bl) => {
            print_at_depth(format!("Boolean - {}", bl), depth)
        },
        ASTNode::IfStatement(istmt) => {
            print_at_depth(String::from("If Statement:"), depth);
            print_at_depth(String::from("Condition:"), depth + 1);
            print_ast_node(istmt.check_exp.as_ref(), depth + 2);

            print_at_depth(String::from("Then statement:"), depth + 1);
            print_ast_node(istmt.body.as_ref(), depth + 2);

            if let Some(els) = &istmt.else_exp {
                print_at_depth(String::from("Else statement:"), depth + 1);
                print_ast_node(els, depth + 2);
            }
        },
        ASTNode::Null => {
            print_at_depth(String::from("Null"), depth);
        },
        ASTNode::ReturnStatement(rs) => {
            print_at_depth(String::from("Return statement:"), depth);
            print_ast_node(rs.as_ref(), depth + 1);
        },
        ASTNode::WhileLoop(wl) => {
            print_at_depth(String::from("While loop:"), depth);
            print_at_depth(String::from("Check:"), depth + 1);
            print_ast_node(wl.check.as_ref(), depth + 2);
            print_at_depth(String::from("Body:"), depth + 1);
            print_ast_node(wl.body.as_ref(), depth + 2);
        },
        ASTNode::BreakStatement => {
            print_at_depth(String::from("Break statement"), depth)
        },
        ASTNode::ContinueStatement => {
            print_at_depth(String::from("Continue statement"), depth)
        },
        ASTNode::ArrayLiteral(items) => {
            print_at_depth(format!("Array (len {}):", items.len()), depth);
            for i in items {
                print_ast_node(i, depth + 1)
            }
        }
    }
}

pub fn print_ast (ast: &Vec<ASTNode>) {
    for node in ast {
        print_ast_node(node, 0);
    }
}
