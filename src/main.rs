mod parsing;
use parsing::parser;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let n = 1
    ");
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();

    print_ast(ast);
}
