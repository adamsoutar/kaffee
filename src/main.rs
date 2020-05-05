mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        // This prints two to the power of four
        /*
            It's so impressive I feel the need
            to write about it on multiple lines
        */
        println(2 ** 4)
        // Now I can put // TODOs in my code
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
