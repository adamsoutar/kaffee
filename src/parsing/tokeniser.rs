use crate::parsing::char_stream;
use crate::parsing::char_stream::CharStream;
use crate::parsing::tokens::*;
use std::iter::FromIterator;

// TODO: Comments

// Tokeniser acts like a stream
pub struct Tokeniser {
    pub code: CharStream,
    pub current: Token,
    pub eof: bool
}

impl Tokeniser {
    fn eat_whitespace (&mut self) {
        while !self.code.eof && is_whitespace(&self.code.peek()) {
            self.code.read();
        }
    }

    fn read_next (&mut self) {
        self.eat_whitespace();

        if self.code.eof {
            self.eof = true;
            return;
        }

        let c = self.code.peek();

        // TODO: Fix reading of numbers like .5 as properties
        //       See https://github.com/adamsoutar/ajs/blob/d392fcd388a5cb3e044a7fcd32534d7b816520a7/parser/tokeniser.go#L108
        if c == '"' {
            self.current = self.read_string();
        } else if is_identifier_start(&c) {
            self.current = self.read_identifier();
        } else if is_operator_char(&c) {
            self.current = self.read_operator();
        } else if is_punctuation(&c) {
            // Punctuation is just one char, doesn't need a
            // read method
            self.current = Token::Punctuation(self.code.read())
        } else if is_number(&c) {
            self.current = self.read_number();
        } else {
            panic!("Invalid syntax - unexpected character {} in code", c);
        }
    }

    fn read_operator (&mut self) -> Token {
        let mut op = vec![];
        while !self.code.eof && is_operator_char(&self.code.peek()) {
            op.push(self.code.read());
        }
        let st = String::from_iter(op);

        if !is_operator(&st) {
            panic!("\"{}\" is not a valid operator", st)
        }

        Token::Operator(st)
    }

    fn read_identifier (&mut self) -> Token {
        let mut ident = vec![];
        while !self.code.eof && is_identifier(&self.code.peek()) {
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
        while !self.code.eof && self.code.peek() != '"' {
            chars.push(self.code.read());
        }
        self.code.read();
        Token::String(chars.iter().collect())
    }

    fn read_number (&mut self) -> Token {
        let mut vc = vec![];
        while !self.code.eof && is_number(&self.code.peek()) {
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
        current: Token::Number(0.),
        eof: false
    };
    tk.read_next();
    tk
}
