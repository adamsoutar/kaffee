use crate::interpretting::interpreter_utils::*;

pub fn operator_handler (left: KaffeeValue, op: &String, right: KaffeeValue) -> KaffeeValue {
    if op == "==" {
        equal(left, right)
    } else {
        match (left, right) {
            (KaffeeValue::Number(n1), KaffeeValue::Number(n2)) => maths(n1, op, n2),
            _ => panic!("Invalid binary operation type signature.")
        }
    }
}

fn equal (left: KaffeeValue, right: KaffeeValue) -> KaffeeValue {
    // The values derive from PartialEq so impl is easy
    // Only matches values of the same type
    KaffeeValue::Boolean(left == right)
}

fn maths (l: f64, op: &String, r: f64) -> KaffeeValue {
    KaffeeValue::Number(match &op[..] {
        "+" => l + r,
        "-" => l - r,
        "*" => l * r,
        "/" => l / r,
        _ => panic!("Invalid operator for two number types")
    })
}
