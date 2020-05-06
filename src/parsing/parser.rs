use crate::parsing::tokens::*;
use crate::parsing::tokeniser;
use crate::parsing::tokeniser::Tokeniser;
use crate::parsing::ast_utils::*;
use crate::parsing::printer::print_token;

// Parser does not act like a stream, it
// constructs the AST in one go
pub struct Parser {
    pub tokens: Tokeniser
}

// NOTE: There is more implementation than this!
//       Little methods like expect_punctuation()
//       are in parser_helpers.rs
impl Parser {
    fn parse_atom (&mut self, accept_statements: bool) -> ASTNode {
        let t = self.tokens.read();

        if let Token::Punctuation(pnc) = t {
            // Bracketed expressions
            if pnc == '(' {
                let exp = self.parse_component(true, 0);
                self.expect_punctuation(')');
                return exp;
            }
        }

        match t {
            Token::String(st) => {
                return ASTNode::String(st)
            },
            Token::Number(n) => {
                return ASTNode::Number(n)
            },
            Token::Identifier(id) => {
                return ASTNode::Identifier(id)
            },
            Token::Boolean(bl) => {
                return ASTNode::Boolean(bl)
            },
            Token::Null => {
                return ASTNode::Null
            },
            _ => {}
        }

        // Some keywords are ok for expressions
        // without being statements
        if let Token::Keyword(kw) = &t {
            match &kw[..] {
                "function" => {
                    return self.parse_function_definition()
                },
                _ => {}
            }
        }

        if !accept_statements {
            // When we aren't looking for statements,
            // {} is a dictionary

            if let Token::Punctuation(pnc) = t {
                if pnc == '{' {
                    return self.parse_object_literal();
                }
            }

            // If we've got here and we're not using a dictionary,
            // we're in trouble
            panic!("Used a statement where they are not allowed")
        }

        self.parse_statement(t)
    }

    fn parse_statement (&mut self, t: Token) -> ASTNode {
        if let Token::Keyword(kw) = t {
            let kwstr = &kw[..];
            match kwstr {
                "let" => {
                    return self.parse_variable_declaration(false)
                },
                "const" => {
                    return self.parse_variable_declaration(true)
                },
                "if" => {
                    return self.parse_if_statement()
                },
                "return" => {
                    return self.parse_return_statement()
                },
                "continue" => {
                    return ASTNode::ContinueStatement
                },
                "break" => {
                    return ASTNode::BreakStatement
                },
                "while" => {
                    return self.parse_while_loop()
                },
                "for" => {
                    return self.parse_for_loop()
                }
                _ => panic!("Unknown keyword \"{}\"", kw)
            }
        }

        if let Token::Punctuation(pnc) = t {
            if pnc == '{' {
                return self.parse_block_statement(false, true);
            }
        }

        print_token(&t);
        panic!("Unsupported syntax")
    }

    fn parse_for_loop (&mut self) -> ASTNode {
        // These are actually transformed into while loops by the parser
        // NOTE: This does lead to quite a few unnecessary scopes
        // TODO: for (let key in object) etc.
        let mut expect_last = false;
        if self.is_next_punctuation('(') {
            // Bracketing for loop conditions is optional
            self.tokens.read();
            expect_last = true;
        }

        let decl = self.parse_component(true, 0);
        let check = self.parse_component(true, 0);
        let incr = self.parse_component(true, 0);

        if expect_last { self.expect_punctuation(')') }

        let body = self.parse_component(true, 0);

        // Performs body of loop, then increments
        let incr_body = ASTNode::BlockStatement(vec![body, incr]);

        let wl = ASTNode::WhileLoop(WhileProperties {
            check: Box::new(check),
            body: Box::new(incr_body)
        });

        // Bundles the declaration at the start of the loop
        ASTNode::BlockStatement(vec![decl, wl])
    }

    fn parse_while_loop (&mut self) -> ASTNode {
        let check = Box::new(self.parse_component(false, 0));
        let body = Box::new(self.parse_component(true, 0));

        ASTNode::WhileLoop(WhileProperties {
            check, body
        })
    }

    fn parse_return_statement (&mut self) -> ASTNode {
        let val = self.parse_component(false, 0);
        ASTNode::ReturnStatement(Box::new(val))
    }

    fn parse_if_statement (&mut self) -> ASTNode {
        let check_exp = Box::new(self.parse_component(false, 0));
        let body = Box::new(self.parse_component(true, 0));

        let mut else_exp = None;
        if self.is_next_keyword("else") {
            self.tokens.read();
            else_exp = Some(Box::new(self.parse_component(true, 0)));
        }

        ASTNode::IfStatement(IfProperties {
            check_exp,
            body,
            else_exp
        })
    }

    fn parse_function_definition (&mut self) -> ASTNode {
        // TODO: Anonymous functions, also, warn when a non-anonymous
        //       function is assigned to a variable
        let name_ident = &self.parse_atom(false);
        let name = self.ident_as_string(name_ident);

        let args = self.parse_delimited('(', ',', ')')
            .iter().map(|x| self.ident_as_string(x)).collect();

        let body_block = self.parse_block_statement(true, true);

        if let ASTNode::BlockStatement(body) = body_block {
            ASTNode::FunctionDefinition(FunctionDefinitionProperties {
                name, args, body
            })
        } else {
            unreachable!()
        }
    }

    fn ident_as_string (&mut self, ident: &ASTNode) -> String {
        if let ASTNode::Identifier(name) = ident {
            name.clone()
        } else {
            panic!("Expected identifier.");
        }
    }

    fn parse_object_literal (&mut self) -> ASTNode {
        // TODO: Empty object literals
        let mut keys = vec![];
        let mut values = vec![];

        while !self.tokens.eof {
            let t = self.tokens.read();

            if let Token::Identifier(id) = t {
                keys.push(id.clone());

                if self.is_next_punctuation(',') ||
                   self.is_next_punctuation('}') {
                    // This is an implicit key/value { a, b, c }
                    values.push(ASTNode::Identifier(id));
                    if self.is_next_punctuation('}') {
                        self.tokens.read();
                        break;
                    }
                    self.tokens.read();
                    continue;
                } else {
                    // Explicit key/value { a: b }
                    self.expect_punctuation(':');
                    values.push(self.parse_component(false, 0));

                    let nt = self.tokens.read();
                    if let Token::Punctuation(pnc) = nt {
                        if pnc == '}' {
                            break
                        } else if pnc == ',' {
                            continue
                        }
                    }

                    panic!("Invalid token after value in object literal.")
                }
            } else {
                panic!("Object keys should be identifiers
(or you left a dangling comma { a, })")
            }
        }

        ASTNode::ObjectLiteral(ObjectLiteralProperties{
            keys,
            values
        })
    }

    fn parse_variable_declaration (&mut self, constant: bool) -> ASTNode {
        let nxt = self.parse_component(false, 0);

        match nxt {
            ASTNode::Assignment(assignment) => {
                return ASTNode::Declaration(DeclarationProperties{
                    constant,
                    assignment
                })
            },
            _ => {
                panic!("Declaration (let, const) wasn't followed by an assignment");
            }
        }
    }

    fn parse_delimited (&mut self, start: char, delim: char, end: char) -> Vec<ASTNode> {
        self.expect_punctuation(start);

        let mut args = vec![];
        loop {
            // Some delims are empty ()
            if self.is_next_punctuation(end) { break; }

            args.push(self.parse_component(false, 0));

            if !self.is_next_punctuation(delim) {
                break;
            }

            // Read the delim char
            self.tokens.read();
        }

        self.expect_punctuation(end);
        return args;
    }

    fn might_be_assignment (&mut self, me: ASTNode) -> ASTNode {
        // The clone is to prevent a mutable/immutable borrow
        let t = self.tokens.peek().clone();

        if let Token::Operator(op) = t {
            if is_assignment_operator(&op) {
                self.tokens.read();
                let right = Box::new(self.parse_component(false, 0));

                if op == "=" {
                    return ASTNode::Assignment(BinaryProperties {
                        left: Box::new(me),
                        operator: op.clone(),
                        right
                    })
                }

                // We have an assignment operator
                let idx = op.len() - 1;
                // Takes the = off the end (**= => **)
                let bin_op = String::from(&op[0..idx]);
                // Synthesise a binary node
                // Transforms a += 1 to a = a + 1
                let bin = ASTNode::BinaryNode(BinaryProperties {
                    left: Box::new(me.clone()),
                    operator: bin_op,
                    right
                });

                return ASTNode::Assignment(BinaryProperties {
                    left: Box::new(me),
                    operator: String::from("="),
                    right: Box::new(bin)
                })
            }
        }

        me
    }

    fn might_be_binary (&mut self, me: ASTNode, my_precedence: i32) -> ASTNode {
        let t = self.tokens.peek().clone();

        if let Token::Operator(op) = t {
            if is_binary_operator(&op) {
                let their_prec = get_operator_precedence(&op);

                if their_prec > my_precedence {
                    self.tokens.read();

                    let them = self.parse_component(false, their_prec);

                    let node = ASTNode::BinaryNode(BinaryProperties {
                        left: Box::new(me),
                        operator: op,
                        right: Box::new(them)
                    });

                    return self.might_be_binary(node, my_precedence)
                }
            }
        }

        me
    }

    fn might_be_call (&mut self, node: ASTNode) -> (bool, ASTNode) {
        if self.is_next_punctuation('(') {
            let args = self.parse_delimited('(', ',', ')');
            return (true, ASTNode::FunctionCall(CallProperties {
                callee: Box::new(node),
                args
            }))
        }

        (false, node)
    }

    fn might_be_property_access (&mut self, node: ASTNode) -> (bool, ASTNode) {
        if self.is_next_punctuation('.') {
            self.tokens.read();

            let key = self.parse_atom(false);
            if let ASTNode::Identifier(id) = key {
                // This transforms obj.key into obj["key"]
                return (true, ASTNode::PropertyAccess(AccessProperties {
                    object: Box::new(node),
                    property: Box::new(ASTNode::String(id))
                }))
            } else {
                panic!("Property access (a.b) key must be an identifier")
            }
        }

        (false, node)
    }

    fn might_be_computed_property_access (&mut self, node: ASTNode) -> (bool, ASTNode) {
        if self.is_next_punctuation('[') {
            self.tokens.read();

            let key = self.parse_atom(false);
            self.expect_punctuation(']');

            return (true, ASTNode::PropertyAccess(AccessProperties {
                object: Box::new(node),
                property: Box::new(key)
            }))
        }

        (false, node)
    }

    fn parse_component (&mut self, accept_statements: bool, prec: i32) -> ASTNode {
        let mut node = self.parse_atom(accept_statements);

        while !self.tokens.eof {
            let (was_acc, acc_node) = self.might_be_property_access(node.clone());
            let (was_comp, comp_node) = self.might_be_computed_property_access(acc_node);
            let (was_call, call_node) = self.might_be_call(comp_node);

            if !(was_acc || was_call || was_comp) {
                break;
            }

            node = call_node;
        }

        let mba = self.might_be_assignment(node);
        self.might_be_binary(mba, prec)
    }

    fn parse_block_statement (&mut self, expect_first_brace: bool, expect_last_brace: bool) -> ASTNode {
        if expect_first_brace {
            self.expect_punctuation('{')
        }

        let mut statements = vec![];
        while !self.tokens.eof {
            if expect_last_brace && self.is_next_punctuation('}') {
                break
            }

            statements.push(self.parse_component(true, 0));
        }

        if expect_last_brace {
            self.expect_punctuation('}')
        }

        ASTNode::BlockStatement(statements)
    }

    pub fn generate_ast (&mut self) -> Vec<ASTNode> {
        let bs = self.parse_block_statement(false, false);

        match bs {
            ASTNode::BlockStatement(statements) => statements,
            _ => unreachable!()
        }
    }
}

pub fn new (code: String) -> Parser {
    let tk = tokeniser::new(code);
    let pr = Parser {
        tokens: tk
    };
    pr
}
