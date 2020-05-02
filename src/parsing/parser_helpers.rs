use crate::parsing::parser::Parser;
use crate::parsing::tokens::*;

impl Parser {
    pub fn expect_punctuation(&mut self, c: char) {
        let tk = self.tokens.read();
        if !match tk {
            Token::Punctuation(pnc) => pnc == c,
            _ => false
        } {
            panic!("Expected punctution '{}'", c)
        }
    }

    pub fn is_next_punctuation(&self, c: char) -> bool {
        match self.tokens.peek() {
            Token::Punctuation(pnc) => *pnc == c,
            _ => false
        }
    }
}
