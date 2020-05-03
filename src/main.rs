mod parsing;
use parsing::parser;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        const adam = {
            name: \"Adam Soutar\",
            age: 17,
            code
        }
    ");
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();

    print_ast(ast);
}
