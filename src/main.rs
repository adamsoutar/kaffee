mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function sayHello () {
            println(\"Hello\")
        }

        const funcs = {
            sayHello,
            sayWorld: function sayWorld () {
                println(\"World\")
            }
        }
        funcs.sayHello()
        funcs.sayWorld()
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
