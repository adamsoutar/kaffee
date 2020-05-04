fn rust_stringify (value: KaffeeValue) -> String {
    match value {
        KaffeeValue::Number(n) => format!("{}", n),
        KaffeeValue::String(st) => st.clone(),
        _ => String::from("Unstringifyable value")
    }
}

pub fn print (value: KaffeeValue) {
    println!("{}", rust_stringify(value))
}
