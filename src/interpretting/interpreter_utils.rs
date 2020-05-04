use crate::parsing::ast_utils::ASTNode;

#[derive(Clone)]
pub struct AllocedValue {
    pub value: KaffeeValue,
    pub ref_count: usize,
    pub constant: bool
}

// These also correspond to types
#[derive(Clone)]
pub enum KaffeeValue {
    // Primitives
    Number(f64),
    String(String),
    // TODO: Null
    // TODO: Boolean

    // Composed
    Object(ObjectValue),
    NativeFunction(NativeMapping),
    Function(FunctionDefinition)
}

#[derive(Clone)]
pub struct FunctionDefinition {
    pub args: Vec<String>,
    pub body: Vec<ASTNode>
}

pub type NativeFuncSignature = fn(Vec<KaffeeValue>);

#[derive(Clone)]
pub struct NativeMapping {
    pub name: String,
    pub arg_count: usize,
    pub func: NativeFuncSignature
}

#[derive(Clone)]
pub struct ObjectValue {
    pub keys: Vec<AllocedValue>,
    pub values: Vec<AllocedValue>
}
