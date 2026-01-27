#[derive(Debug, Clone)]
pub enum Statement {

    VarDecl {
        name: String, 
        value: Literal,
        is_private: bool,
    },

    FuncDecl { 
        name: String, 
        params: Vec<Parameter>, 
        body: Vec<Statement>,
        is_private: bool,
        is_mut: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: String,
    pub is_mut: bool,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    String(String),
}