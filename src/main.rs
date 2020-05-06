mod parsing;
mod interpretting;
mod std_lib;
use interpretting::interpreter;
use parsing::printer::print_ast;

fn main() {
    let code = String::from("
        let dude = {
            name: \"Adam\"
        }

        let key = \"age\"
        dude[key] = 17
        dude.project = \"KaffeeLang\"

        println(dude.name)
        println(dude[\"age\"])
        println(dude.project)
    ");
    let mut interp = interpreter::new(code);
    print_ast(&interp.ast);
    interp.run();
}
