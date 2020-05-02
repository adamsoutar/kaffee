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
                    // TODO: Parse object literal
                    return ASTNode::ObjectLiteral(ObjectLiteralProperties {
                        keys: vec![],
                        values: vec![]
                    })
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

        panic!("As-yet unsupported statement token")
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

                ASTNode::Assignment(AssignmentProperties{
                    left: Box::new(me),
                    operator: op.clone(),
                    right: Box::new(self.parse_component(false))
                })
            } else { me }
        } else { me }
    }

    fn parse_component (&mut self, accept_statements: bool) -> ASTNode {
        // TODO
        let node = self.parse_atom(accept_statements);
        self.might_be_assignment(node)
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
