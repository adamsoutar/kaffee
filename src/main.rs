mod parser;
use parser::tokeniser;
use parser::tokens::*;

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

fn main() {
    let code = String::from("
        let code = \"Hello, world!\"
        const object = {
            a: \"Hi\"
        }
    ");
    let mut tokens = tokeniser::new(code);

    while !tokens.eof {
        let tk = tokens.read();
        print_token(&tk);
    }
}
