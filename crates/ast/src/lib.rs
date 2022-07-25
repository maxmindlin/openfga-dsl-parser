pub struct Document {
    pub types: Vec<Type>,
}

pub struct Type {
    pub kind: String,
    pub relations: Vec<Relation>,
}

pub struct Relation {
    pub kind: String,
    pub alias: Vec<Alias>,
    pub from: String, // this seems to be an enum, need to read up.
}

pub enum Alias {
    This,
    Named(String),
}
