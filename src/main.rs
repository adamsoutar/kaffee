mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function subret () {
            return null
        }

        function rettest () {
            println(\"I should be printed\")
            subret()
            println(\"I should also be printed\")
        }

        println(\"Top-level 1\")
        rettest()
        println(\"Top-level 2\")
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
