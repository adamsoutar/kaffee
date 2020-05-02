mod parsing;
use parsing::parser;

fn main() {
    let code = String::from("
        let code = \"Hello, world!\"
    ");
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();
}
