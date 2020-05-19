mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let numbers = [1, 2, 3, 4]
        
        for let i = 0; i < 4; i += 1 {
            println(numbers[i])
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
