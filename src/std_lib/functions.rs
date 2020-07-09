use std::io;
use crate::interpretting::interpreter_utils::*;
use crate::interpretting::variables::Variables;

// TODO: To be able to stringify objects, we need access to the alloc array
fn rust_stringify (value: &KaffeeValue, _: &mut Variables) -> String {
    match value {
        KaffeeValue::Number(n) => format!("{}", n),
        KaffeeValue::String(st) => st.clone(),
        KaffeeValue::Boolean(bl) => format!("{}", bl),
        // TODO: Exception for this
        _ => panic!("Unstringifyable value")
    }
}

fn native_println (args: Vec<KaffeeValue>, vars: &mut Variables) -> KaffeeValue {
    println!("{}", rust_stringify(&args[0], vars));
    KaffeeValue::Null
}

fn native_stringify(args: Vec<KaffeeValue>, vars: &mut Variables) -> KaffeeValue {
    KaffeeValue::String(rust_stringify(&args[0], vars))
}

fn native_len(args: Vec<KaffeeValue>, _: &mut Variables) -> KaffeeValue {
    KaffeeValue::Number(match &args[0] {
        KaffeeValue::String(st) => st.len(),
        KaffeeValue::Array(ar) => ar.len(),
        _ => panic!("Value has no discernable length")
    } as f64)
}

fn native_append(args: Vec<KaffeeValue>, vars: &mut Variables) -> KaffeeValue {
    let mut arr = match &args[0] {
        KaffeeValue::Array(itms) => itms.clone(),
        _ => panic!("Append's first arg should be an array")
    };

    let idx = vars.alloc_value(args[1].clone(), false);
    arr.push(idx);

    KaffeeValue::Array(arr)
}

fn native_input(_: Vec<KaffeeValue>, _: &mut Variables) -> KaffeeValue {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Chop the \n off the end.
            KaffeeValue::String(
                input
                    .chars()
                    .take(input.len() - 1)
                    .collect())
        },
        Err(error) => panic!(error)
    }
}

pub fn get_std_lib_mappings () -> Vec<NativeMapping> {
    vec![
        NativeMapping {
            name: "println".to_string(),
            arg_count: 1,
            func: native_println
        },
        NativeMapping {
            name: "stringify".to_string(),
            arg_count: 1,
            func: native_stringify
        },
        NativeMapping {
            name: "len".to_string(),
            arg_count: 1,
            func: native_len
        },
        NativeMapping {
            name: "append".to_string(),
            arg_count: 2,
            func: native_append
        },
        NativeMapping {
            name: "input".to_string(),
            arg_count: 0,
            func: native_input
        }
    ]
}
