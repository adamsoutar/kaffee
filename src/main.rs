mod parsing;
use parsing::parser;

fn main() {
    let code = String::from("
        let code = 3 + 1
    ");
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();
}
