mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function say(str) {
            function doubleNested () {
                println(str)
            }
            doubleNested()
        }

        function wrappedSay(str) {
            say(str)
        }

        function nestTest () {
            say(\"Nested\")
            wrappedSay(\"Function calls?\")
        }

        nestTest()
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
