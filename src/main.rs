mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function fizzbuzz(n) {
            let out = \"\"
            if n % 3 == 0 out = \"Fizz\"
            if n % 5 == 0 out += \"Buzz\"
            if out == \"\" out = n
            println(out)
        }

        for (let i = 0; i < 500; i += 1) {
            fizzbuzz(i)
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
