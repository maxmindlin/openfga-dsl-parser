pub struct Document {
    pub types: Vec<Type>,
}

pub struct Type {
    pub kind: String,
    pub relations: Vec<Relation>,
}

pub struct Relation {
    pub kind: String,
    pub aliases: Vec<Alias>,
}

pub struct Alias {
    pub kind: AliasKind,
    pub parent: Option<String>,
}

pub enum AliasKind {
    This,
    Named(String),
}
