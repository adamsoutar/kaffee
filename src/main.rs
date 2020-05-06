mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let object = {
            counter: 0
        }
        // This doesn't work yet
        // You can't change an object's properties after it's defined
        for ({}; object.counter < 5; object.counter += 1) {
            println(object.counter)
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
