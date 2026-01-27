#[derive(Debug, Clone)]
pub enum Statement {

    VarDecl {
        name: String, 
        type_annotation: Option<String>,
        value: Expression,
        is_private: bool,
    },

    FuncDecl { 
        name: String, 
        params: Vec<Parameter>,
        return_type: Option<String>,
        body: Vec<Statement>,
        is_private: bool,
        is_mut: bool,
    },

    Return {
        value: Option<Expression>,
    },
}

//function parameters
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: String,
    pub is_mut: bool,
}

//expressions - things that evaluate to a value
#[derive(Debug, Clone)]
pub enum Expression {
    //literals
    IntLiteral(i64),
    StringLiteral(String),
    
    //variable reference
    Variable(String),
    
    //binary operations, a + b, x * y, health > 0
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    
    //function call, print(x), heal_player(player, 50)
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    //arithmetic
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    
    //comparison
    Equals,        // ==
    NotEquals,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    
    //logical
    And, // &&
    Or,  // ||
}