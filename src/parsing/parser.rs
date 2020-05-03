use crate::parsing::tokens::*;
use crate::parsing::tokeniser;
use crate::parsing::tokeniser::Tokeniser;
use crate::parsing::ast_utils::*;

// Parser does not act like a stream, it
// constructs the AST in one go
pub struct Parser {
    pub tokens: Tokeniser
}

fn print_token (tk: &Token) {
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
        }
    }
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
                let exp = self.parse_component(true);
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
            }
            // TODO: Boolean literals
            _ => {}
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
                _ => {}
            }
        }

        panic!("Unsupported syntax")
    }

    fn parse_object_literal (&mut self) -> ASTNode {
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
                    values.push(self.parse_component(false));

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
        let nxt = self.parse_component(false);

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

    fn might_be_assignment (&mut self, me: ASTNode) -> ASTNode {
        // The clone is to prevent a mutable/immutable borrow
        let t = self.tokens.peek().clone();

        if let Token::Operator(op) = t {
            if is_assignment_operator(&op) {
                self.tokens.read();

                return ASTNode::Assignment(BinaryProperties {
                    left: Box::new(me),
                    operator: op.clone(),
                    right: Box::new(self.parse_component(false))
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

                    let them = self.parse_component(false);

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

    fn parse_component (&mut self, accept_statements: bool) -> ASTNode {
        // TODO: Property access and function calls
        let node = self.parse_atom(accept_statements);
        let mba = self.might_be_assignment(node);
        self.might_be_binary(mba, 0)
    }

    fn parse_block_statement (&mut self, expect_first_brace: bool, expect_last_brace: bool) -> ASTNode {
        if expect_first_brace {
            self.expect_punctuation('{')
        }

        let mut statements = vec![];
        while !self.tokens.eof {
            statements.push(self.parse_component(true))
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
