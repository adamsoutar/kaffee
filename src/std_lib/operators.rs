use crate::interpretting::interpreter_utils::*;

pub fn operator_handler (left: KaffeeValue, op: &String, right: KaffeeValue) -> KaffeeValue {
    // TODO: This isn't very clean
    let generic_ops = vec!["==", "!="];
    if generic_ops.contains(&&op[..]) {
        return generic(left, op, right)
    }

    match (left, right) {
        (KaffeeValue::Number(n1), KaffeeValue::Number(n2)) => maths(n1, op, n2),
        (KaffeeValue::Boolean(b1), KaffeeValue::Boolean(b2)) => bools(b1, op, b2),
        _ => panic!("Invalid binary operation type signature.")
    }
}

// Some operators work on all types
fn generic (l: KaffeeValue, op: &String, r: KaffeeValue) -> KaffeeValue {
    KaffeeValue::Boolean(match &op[..] {
        "==" => l == r,
        "!=" => l != r,
        _ => unreachable!()
    })
}

fn maths (l: f64, op: &String, r: f64) -> KaffeeValue {
    KaffeeValue::Number(match &op[..] {
        "+" => l + r,
        "-" => l - r,
        "*" => l * r,
        "/" => l / r,
        "%" => l % r,
        "**" => l.powf(r),
        _ => panic!("Invalid operator for two number types \"{}\"", op)
    })
}

fn bools (l: bool, op: &String, r: bool) -> KaffeeValue {
    KaffeeValue::Boolean(match &op[..] {
        "&&" => l && r,
        "||" => l || r,
        _ => panic!("Invalid operator for two boolean types \"{}\"", op)
    })
}
