#[derive(Clone)]
pub enum Token {
    String(String),
    Identifier(String),
    Keyword(String),
    Number(f64)
}

// TODO: Don't redefine the vectors in each
//       is_ function.

pub fn is_whitespace (c: &char) -> bool {
    let whitespace = vec![' ', '\t', '\n', ';'];
    whitespace.contains(c)
}

pub fn is_number(c: &char) -> bool {
    let nums = vec!['0','1','2','3','4','5','6','7','8','9','.'];
    nums.contains(c)
}

pub fn is_keyword(s: &String) -> bool {
    let keywords: Vec<String> = vec![
        "let", "const"
    ].into_iter().map(|x| x.to_owned()).collect();
    keywords.contains(s)
}

pub fn is_identifier(c: &char) -> bool {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$_"
        .chars().collect();
    chars.contains(c)
}

pub fn is_identifier_start(c: &char) -> bool {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$_"
        .chars().collect();
    chars.contains(c)
}
