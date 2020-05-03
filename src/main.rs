mod parsing;
mod interpretting;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let n = 1
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
