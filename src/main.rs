mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function test () {
            println(\"Hello\")
            if false {
                println(\"New\")
            } else {
                return null
            }
            println(\"World\")
        }
        test()
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
