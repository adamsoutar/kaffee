mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        {
            let gc = \"?\"
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
