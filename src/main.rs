mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let nums = [1, 2, 3, 4]

        function stickOn(a, n) {
            return append(a, n)
        }
        nums = stickOn(nums, 5)

        for let i = 0; i < len(nums); i += 1 {
            println(nums[i])
        }
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
