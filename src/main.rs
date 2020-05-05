mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function test () {
            let i = 0
            while true {
                if i == 10 break
                i = i + 1
                // Skip 5
                if i == 5 continue
                println(i)
            }
            println(\"Make sure we didn't return from the function\")
        }

        println(test())
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
