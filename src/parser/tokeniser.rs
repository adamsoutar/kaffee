use crate::parser::char_stream;
use crate::parser::char_stream::CharStream;
use crate::parser::tokens::*;
use std::iter::FromIterator;

pub struct Tokeniser {
    pub code: CharStream,
    pub current: Token
}

impl Tokeniser {
    fn eat_whitespace (&mut self) {
        while is_whitespace(&self.code.peek()) {
            self.code.read();
        }
    }

    fn read_next (&mut self) {
        self.eat_whitespace();

        let c = self.code.peek();

        if is_number(&c) {
            self.current = self.read_number();
        } else if c == '"' {
            self.current = self.read_string();
        } else if is_identifier_start(&c) {
            self.current = self.read_identifier();
        }
    }

    fn read_identifier (&mut self) -> Token {
        let mut ident = vec![];
        while is_identifier(&self.code.peek()) {
            ident.push(self.code.read());
        }
        let st = String::from_iter(ident);
        if is_keyword(&st) {
            Token::Keyword(st)
        } else {
            Token::Identifier(st)
        }
    }

    fn read_string (&mut self) -> Token {
        self.code.read();
        let mut chars = vec![];
        // TODO: Escapes and EOF
        while self.code.peek() != '"' {
            chars.push(self.code.read());
        }
        self.code.read();
        Token::String(chars.iter().collect())
    }

    fn read_number (&mut self) -> Token {
        let mut vc = vec![];
        while is_number(&self.code.peek()) {
            vc.push(self.code.read())
        }
        let st = String::from_iter(vc);
        Token::Number(st.parse().unwrap())
    }

    pub fn peek (&self) -> &Token {
        &self.current
    }

    pub fn read (&mut self) -> Token {
        let tk = self.current.clone();
        self.read_next();
        tk
    }
}

pub fn new (code: String) -> Tokeniser {
    let cs = char_stream::new(code);
    let mut tk = Tokeniser {
        code: cs,
        current: Token::Number(0.)
    };
    tk.read_next();
    tk
}
