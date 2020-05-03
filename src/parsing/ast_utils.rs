#[derive(Clone)]
pub enum ASTNode {
    String(String),
    Number(f64),
    Identifier(String),
    ObjectLiteral(ObjectLiteralProperties),
    BlockStatement(Vec<ASTNode>),
    Assignment(BinaryProperties),
    Declaration(DeclarationProperties),
    BinaryNode(BinaryProperties)
}

#[derive(Clone)]
pub struct ObjectLiteralProperties {
    pub keys: Vec<String>,
    pub values: Vec<ASTNode>
}

#[derive(Clone)]
pub struct BinaryProperties {
    pub left: Box<ASTNode>,
    pub operator: String,
    pub right: Box<ASTNode>
}

#[derive(Clone)]
pub struct DeclarationProperties {
    pub constant: bool,
    pub assignment: BinaryProperties
}