use crate::parsing::ast_utils::ASTNode;
use crate::interpretting::variables::Variables;

#[derive(Clone)]
pub struct AllocedValue {
    pub value: KaffeeValue,
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
    // Points into the alloc hashmap
    Array(Vec<usize>),
    Function(FunctionDefinition)
}

#[derive(Clone, PartialEq)]
pub struct FunctionDefinition {
    pub args: Vec<String>,
    pub body: Vec<ASTNode>
}

pub type NativeFuncSignature = fn(Vec<KaffeeValue>, &mut Variables) -> KaffeeValue;

#[derive(Clone)]
pub struct NativeMapping {
    pub name: String,
    pub arg_count: usize,
    pub func: NativeFuncSignature
}

// Native mappings (eg. println) are the same if their names are the same
impl PartialEq for NativeMapping {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone, PartialEq)]
pub struct ObjectValue {
    pub keys: Vec<usize>,
    // Indexes into alloced
    pub values: Vec<usize>
}
