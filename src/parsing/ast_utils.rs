#[derive(Clone)]
pub enum ASTNode {
    String(String),
    Number(f64),
    Identifier(String),
    ObjectLiteral(ObjectLiteralProperties),
    BlockStatement(Vec<ASTNode>),
    Assignment(BinaryProperties),
    Declaration(DeclarationProperties),
    BinaryNode(BinaryProperties),
    FunctionCall(CallProperties),
    FunctionDefinition(FunctionDefinitionProperties),
    PropertyAccess(AccessProperties)
}

#[derive(Clone)]
pub struct FunctionDefinitionProperties {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<ASTNode>
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
    // TODO: Panic if declaration binary doesn't use =
    //       and/or the left isn't an identifier
    pub assignment: BinaryProperties
}

#[derive(Clone)]
pub struct CallProperties {
    pub callee: Box<ASTNode>,
    pub args: Vec<ASTNode>
}

#[derive(Clone)]
pub struct AccessProperties {
    pub object: Box<ASTNode>,
    pub property: Box<ASTNode>
}
