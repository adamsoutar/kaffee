mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function test () {
            // Can we return from within loops?
            while true {
                println(\"Stage 1\")
                return null
                println(\"Stage 2\")
            }
            println(\"We shouldn't get here\")
        }
        println(\"Hello\")
        test()
        println(\"World\")
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
