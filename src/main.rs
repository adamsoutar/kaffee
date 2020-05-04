mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        const name = \"Adam\"
        println(\"My name is:\")
        println(name)
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
