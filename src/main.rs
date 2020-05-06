mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let pi = 4
        let denom = 1
        let plus = false
        while true {
            println(pi)

            denom += 2
            if plus {
                pi += 4 / denom
                plus = false
            } else {
                pi -= 4 / denom
                plus = true
            }
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
