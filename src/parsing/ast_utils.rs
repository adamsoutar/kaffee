#[derive(Clone, PartialEq)]
pub enum ASTNode {
    String(String),
    Number(f64),
    Identifier(String),
    Boolean(bool),
    Null,
    ObjectLiteral(ObjectLiteralProperties),
    BlockStatement(Vec<ASTNode>),
    Assignment(BinaryProperties),
    Declaration(DeclarationProperties),
    BinaryNode(BinaryProperties),
    FunctionCall(CallProperties),
    FunctionDefinition(FunctionDefinitionProperties),
    PropertyAccess(AccessProperties),
    IfStatement(IfProperties),
    ReturnStatement(Box<ASTNode>),
    WhileLoop(WhileProperties),
    ContinueStatement,
    BreakStatement,
    ArrayLiteral(Vec<ASTNode>)
}

#[derive(Clone, PartialEq)]
pub struct WhileProperties {
    pub check: Box<ASTNode>,
    pub body: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct FunctionDefinitionProperties {
    pub name: String,
    pub args: Vec<String>,
    // TODO: Make this a normal Box<ASTNode>
    pub body: Vec<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct IfProperties {
    pub check_exp: Box<ASTNode>,
    pub body: Box<ASTNode>,
    pub else_exp: Option<Box<ASTNode>>
}

#[derive(Clone, PartialEq)]
pub struct ObjectLiteralProperties {
    pub keys: Vec<String>,
    pub values: Vec<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct BinaryProperties {
    pub left: Box<ASTNode>,
    pub operator: String,
    pub right: Box<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct DeclarationProperties {
    pub constant: bool,
    pub assignment: BinaryProperties
}

#[derive(Clone, PartialEq)]
pub struct CallProperties {
    pub callee: Box<ASTNode>,
    pub args: Vec<ASTNode>
}

#[derive(Clone, PartialEq)]
pub struct AccessProperties {
    pub object: Box<ASTNode>,
    pub property: Box<ASTNode>
}
