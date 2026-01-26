#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl { name: String, value: Literal },
    FuncDecl { name: String, params: Vec<String>, body: Vec<Statement> },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    String(String),
}