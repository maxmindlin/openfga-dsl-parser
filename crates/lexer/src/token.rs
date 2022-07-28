#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    // keywords
    Type,
    Relations,
    Define,
    As,
    This,
    Or,
    And,
    From,

    Text,

    Newline,
    EOF,

    Illegal,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    lit: String,
    kind: TokenKind,
    // span: Span,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Span {
//     start: usize,
//     end: usize,
// }

impl Token {
    pub fn new(lit: String, kind: TokenKind) -> Self {
        Self {
            lit,
            kind,
            // span: Span { start, end },
        }
    }

    pub fn literal(&self) -> &str {
        &self.lit
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

impl TokenKind {
    pub fn is_to_keyword(literal: &str) -> Option<Self> {
        match literal {
            "type" => Some(Self::Type),
            "relations" => Some(Self::Relations),
            "define" => Some(Self::Define),
            "as" => Some(Self::As),
            "self" => Some(Self::This),
            "or" => Some(Self::Or),
            "and" => Some(Self::And),
            "from" => Some(Self::From),
            _ => None,
        }
    }
}
