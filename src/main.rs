mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        fn stringy (n) {
            let out = \"\"
            if n % 3 == 0 out = \"Fizz\"
            if n % 5 == 0 out += \"Buzz\"
            if out == \"\" out = stringify(n)
            println(out)
        }

        fn fizzbuzz (x) {
            for let i = 1 i < x i += 1 stringy(i)
        }

        fizzbuzz(100)
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
