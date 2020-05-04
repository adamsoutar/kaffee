#[derive(Clone)]
pub enum Token {
    String(String),
    Identifier(String),
    Keyword(String),
    Number(f64),
    Boolean(bool),
    Null,
    Operator(String),
    Punctuation(char)
}

// TODO: Don't redefine the vectors in each
//       is_ function.

fn in_string_vector (s: &String, v: Vec<&str>) -> bool {
    let vs: Vec<String> = v.into_iter().map(|x| x.to_owned()).collect();
    vs.contains(s)
}

fn in_char_string (c: &char, s: &str) -> bool {
    let x: Vec<char> = s.chars().collect();
    x.contains(c)
}

pub fn is_whitespace (c: &char) -> bool {
    let whitespace = vec![' ', '\t', '\n', ';'];
    whitespace.contains(c)
}

pub fn is_number(c: &char) -> bool {
    let nums = vec!['0','1','2','3','4','5','6','7','8','9','.'];
    nums.contains(c)
}

pub fn is_keyword(s: &String) -> bool {
    in_string_vector(s, vec![
        "let", "const", "function", "if", "else"
    ])
}

pub fn is_operator(s: &String) -> bool {
    // TODO: Exponentiation operator
    in_string_vector(s, vec![
        "=", "==", "+", "-", "*", "/",
        "!="
    ])
}
pub fn is_assignment_operator (s: &String) -> bool {
    // TODO: *=, +=, -=, etc.
    in_string_vector(s, vec![
        "="
    ])
}
pub fn is_binary_operator (s: &String) -> bool {
    in_string_vector(s, vec![
        "+", "-", "*", "/", "==",
        "!="
    ])
}
pub fn get_operator_precedence (s: &String) -> i32 {
    let sstr = &s[..];
    match sstr {
        "+" => 13,
        "-" => 13,
        "*" => 14,
        "/" => 14,
        "==" => 10,
        "!=" => 10,
        _ => 0
    }
}

// Characters may be part of an operator, but not operators themselves
pub fn is_operator_char (c: &char) -> bool {
    in_char_string(c, "=!+-/*")
}

pub fn is_punctuation(c: &char) -> bool {
    in_char_string(c, ":,.()[]{}")
}

pub fn is_identifier(c: &char) -> bool {
    in_char_string(c, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$_")
}

pub fn is_identifier_start(c: &char) -> bool {
    in_char_string(c, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$_")
}
