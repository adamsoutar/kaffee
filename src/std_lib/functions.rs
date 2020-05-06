use crate::interpretting::interpreter_utils::*;

// TODO: To be able to stringify objects, we need access to the alloc array
fn rust_stringify (value: &KaffeeValue) -> String {
    match value {
        KaffeeValue::Number(n) => format!("{}", n),
        KaffeeValue::String(st) => st.clone(),
        KaffeeValue::Boolean(bl) => format!("{}", bl),
        KaffeeValue::Null => String::from("Kaffee::null"),
        // TODO: Exception for this
        _ => panic!("Unstringifyable value")
    }
}

fn native_println (args: Vec<KaffeeValue>) -> KaffeeValue {
    println!("{}", rust_stringify(&args[0]));
    KaffeeValue::Null
}

fn native_stringify(args: Vec<KaffeeValue>) -> KaffeeValue {
    KaffeeValue::String(rust_stringify(&args[0]))
}

pub fn get_std_lib_mappings () -> Vec<NativeMapping> {
    vec![
        NativeMapping {
            name: String::from("println"),
            arg_count: 1,
            func: native_println
        },
        NativeMapping {
            name: String::from("stringify"),
            arg_count: 1,
            func: native_stringify
        }
    ]
}
