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
    Object(ObjectValue)
    // TODO: Function
}

#[derive(Clone)]
pub struct ObjectValue {
    pub keys: Vec<AllocedValue>,
    pub values: Vec<AllocedValue>
}
