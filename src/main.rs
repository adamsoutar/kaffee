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
        }
    }
}

fn main() {
    let code = String::from(" 3.14 \"Hello, world!\" 71 hello let ");
    let mut tokens = tokeniser::new(code);

    print_token(&tokens.read());
    print_token(&tokens.read());
    print_token(&tokens.read());
    print_token(&tokens.read());
    print_token(tokens.peek());
}
