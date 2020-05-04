use crate::parsing::ast_utils::ASTNode;

#[derive(Clone)]
pub struct AllocedValue {
    pub value: KaffeeValue,
    pub ref_count: usize,
    pub constant: bool
}

// These also correspond to types
#[derive(Clone, PartialEq)]
pub enum KaffeeValue {
    // Primitives
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    // Composed
    Object(ObjectValue),
    NativeFunction(NativeMapping),
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
