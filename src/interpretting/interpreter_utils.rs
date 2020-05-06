use crate::parsing::ast_utils::ASTNode;

#[derive(Clone)]
pub struct AllocedValue {
    pub value: KaffeeValue,
    pub ref_count: usize,
    pub constant: bool
}

// Used to determine how blocks were left
#[derive(Clone, PartialEq)]
pub enum BreakType {
    None,
    Break,
    Continue,
    Return
    // TODO: Exception
}

// These also correspond to types
#[derive(Clone, PartialEq)]
pub enum KaffeeValue {
    // Primitives
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    NativeFunction(NativeMapping),

    // Composed
    Object(ObjectValue),
    Function(FunctionDefinition)
}

#[derive(Clone, PartialEq)]
pub struct FunctionDefinition {
    pub args: Vec<String>,
    pub body: Vec<ASTNode>
}

pub type NativeFuncSignature = fn(Vec<KaffeeValue>) -> KaffeeValue;

#[derive(Clone, PartialEq)]
pub struct NativeMapping {
    pub name: String,
    pub arg_count: usize,
    pub func: NativeFuncSignature
}

#[derive(Clone, PartialEq)]
pub struct ObjectValue {
    pub keys: Vec<usize>,
    // Indexes into alloced
    pub values: Vec<usize>
}
