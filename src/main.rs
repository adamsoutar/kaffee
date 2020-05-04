mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        const dude = {
            name: \"Adam Soutar\",
            age: 17
        }

        function greet (person) {
            println(\"Hello,\")
            println(person.name)
            println(\"You're\")
            println(person.age)
        }

        greet(dude)
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
