mod parsing;
use parsing::parser;

fn main() {
    let code = String::from("
        let code = 3 + 2 * 5
        a = 5 / 2

        const adam = {
            name: \"Adam Soutar\",
            age: 17,
            code
        }
    ");
    let mut ps = parser::new(code);
    let ast = ps.generate_ast();
}
