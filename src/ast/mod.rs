#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub types: Vec<Type>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Type {
    pub kind: String,
    pub relations: Vec<Relation>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Relation {
    pub kind: String,
    pub aliases: Vec<Alias>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Alias {
    pub kind: AliasKind,
    pub parent: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AliasKind {
    This,
    Named(String),
    Negative(String),
}
