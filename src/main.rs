mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        function fizzbuzz(n) {
            let out = n
            if n % 3 == 0 out = \"Fizz\"
            else if n % 5 == 0 out = \"Buzz\"
            if n % 3 == 0 && n % 5 == 0 out = \"FizzBuzz\"
            println(out)
        }
        function countupto (n, x) {
            if n == x return null

            fizzbuzz(n)
            countupto(n + 1, x)
        }
        countupto(1, 100)
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
