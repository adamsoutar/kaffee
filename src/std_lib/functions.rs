use crate::interpretting::interpreter_utils::*;

fn rust_stringify (value: &KaffeeValue) -> String {
    match value {
        KaffeeValue::Number(n) => format!("{}", n),
        KaffeeValue::String(st) => st.clone(),
        KaffeeValue::Boolean(bl) => format!("{}", bl),
        KaffeeValue::Null => String::from("Kaffee::null"),
        // TODO: Exception for this
        _ => String::from("Unstringifyable value")
    }
}

fn native_println (args: Vec<KaffeeValue>) -> KaffeeValue {
    println!("{}", rust_stringify(&args[0]));
    KaffeeValue::Null
}

pub fn get_std_lib_mappings () -> Vec<NativeMapping> {
    vec![
        NativeMapping {
            name: String::from("println"),
            arg_count: 1,
            func: native_println
        }
    ]
}
