#[derive(Debug, Clone)]
pub enum Statement {

    VarDecl {
        name: String, 
        value: Literal,
        is_private: bool,
    },

    FuncDecl { 
        name: String, 
        params: Vec<String>, 
        body: Vec<Statement>,
        is_private: bool,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    String(String),
}