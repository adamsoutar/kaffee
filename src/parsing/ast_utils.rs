pub enum ASTNode {
    String(String),
    Number(f64),
    Identifier(String),
    ObjectLiteral(ObjectLiteralProperties),
    BlockStatement(Vec<ASTNode>),
    Assignment(AssignmentProperties),
    Declaration(DeclarationProperties)
}

pub struct ObjectLiteralProperties {
    pub keys: Vec<String>,
    pub values: Vec<ASTNode>
}

pub struct AssignmentProperties {
    pub left: Box<ASTNode>,
    pub operator: String,
    pub right: Box<ASTNode>
}

pub struct DeclarationProperties {
    pub constant: bool,
    pub assignment: AssignmentProperties
}
