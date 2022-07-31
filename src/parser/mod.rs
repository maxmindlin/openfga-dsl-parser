use std::fmt::Display;

use crate::ast::{Alias, AliasKind, Document, Relation, Type};
use crate::lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    lex: Lexer,
    curr: Token,
    peek: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    UnexpectedToken(TokenKind, TokenKind),
    UnexpectedKeyword(TokenKind),
    UnexpectedEOF,
}

impl Parser {
    pub fn new(mut lex: Lexer) -> Self {
        let curr = lex.next_token();
        let peek = lex.next_token();
        Self { lex, curr, peek }
    }

    pub fn parse_document(&mut self) -> ParseResult<Document> {
        let mut types = Vec::new();
        while self.curr.kind() != TokenKind::EOF {
            if self.curr.kind() != TokenKind::Type {
                return Err(ParserError::UnexpectedToken(
                    TokenKind::Type,
                    self.curr.kind(),
                ));
            }
            let ty = self.parse_type()?;
            types.push(ty);
            self.next_token();
        }
        Ok(Document { types })
    }

    fn parse_type(&mut self) -> ParseResult<Type> {
        self.expect_peek(TokenKind::Text)?;
        let kind = self.curr.literal().to_string();
        let mut relations = Vec::new();

        if self.peek.kind() != TokenKind::EOF && self.peek.kind() != TokenKind::Type {
            self.expect_peek(TokenKind::Relations)?;

            while self.peek.kind() == TokenKind::Define {
                self.next_token();
                let rel = self.parse_relation()?;
                relations.push(rel);
            }
        }

        Ok(Type { kind, relations })
    }

    fn parse_relation(&mut self) -> ParseResult<Relation> {
        self.expect_peek(TokenKind::Text)?;
        let kind = self.curr.literal().to_string();
        self.next_token();
        if self.curr.kind() != TokenKind::As {
            // NOTE this might be invalid syntax...
            return Ok(Relation {
                kind,
                aliases: Vec::new(),
            });
        }

        self.next_token();
        let mut aliases = Vec::new();
        let first_alias = self.parse_alias()?;
        aliases.push(first_alias);
        while self.peek.kind() == TokenKind::Or
            || self.peek.kind() == TokenKind::But
        {
            let alias = if self.peek.kind() == TokenKind::But {
                self.next_token();
                self.parse_but_not()?
            } else {
                self.next_token();
                self.next_token();
                self.parse_alias()?
            };
            aliases.push(alias)
        }

        Ok(Relation { kind, aliases })
    }

    fn parse_alias(&mut self) -> ParseResult<Alias> {
        let kind = match self.curr.kind() {
            TokenKind::This => AliasKind::This,
            TokenKind::Text => AliasKind::Named(self.curr.literal().to_string()),
            TokenKind::EOF => return Err(ParserError::UnexpectedEOF),
            _ => return Err(ParserError::UnexpectedKeyword(self.curr.kind())),
        };

        let parent = self.parse_alias_parent()?;
        Ok(Alias { kind, parent })
    }

    fn parse_but_not(&mut self) -> ParseResult<Alias> {
        self.expect_peek(TokenKind::Not)?;
        self.expect_peek(TokenKind::Text)?;
        let kind = AliasKind::Negative(self.curr.literal().to_string());
        let parent = self.parse_alias_parent()?;

        Ok(Alias { kind, parent })
    }

    fn parse_alias_parent(&mut self) -> ParseResult<Option<String>> {
        if self.peek.kind() == TokenKind::From {
            self.next_token();
            self.expect_peek(TokenKind::Text)?;
            let parent = Some(self.curr.literal().to_string());
            Ok(parent)
        } else {
            Ok(None)
        }
    }

    fn next_token(&mut self) {
        let prev = std::mem::replace(&mut self.peek, self.lex.next_token());
        self.curr = prev;
    }

    fn expect_peek(&mut self, expected: TokenKind) -> ParseResult<()> {
        if self.peek.kind() == expected {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(expected, self.peek.kind()))
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserError::*;
        match self {
            UnexpectedToken(exp, got) => write!(f, "Unexpected token: expected {exp:?}, got {got:?}"),
            UnexpectedKeyword(got) => write!(f, "Unexpected keyword: {got:?}"),
            UnexpectedEOF => write!(f, "received an unexpected EOF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_types() {
        let i = "type document
type org";
        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        let exp = Document {
            types: vec![
                Type {
                    kind: "document".into(),
                    relations: Vec::new(),
                },
                Type {
                    kind: "org".into(),
                    relations: Vec::new(),
                },
            ],
        };
        assert_eq!(Ok(exp), parser.parse_document());
    }

    #[test]
    fn can_parse_relation_self() {
        let i = "define write as self";
        let exp = Relation {
            kind: "write".into(),
            aliases: vec![Alias {
                kind: AliasKind::This,
                parent: None,
            }],
        };

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(Ok(exp), parser.parse_relation());
    }

    #[test]
    fn can_parse_but_not_alias() {
        let i = "define write as self but not owner from parent";
        let exp = Relation {
            kind: "write".into(),
            aliases: vec![
                Alias {
                    kind: AliasKind::This,
                    parent: None,
                },
                Alias {
                    kind: AliasKind::Negative("owner".into()),
                    parent: Some("parent".into()),
                },
            ],
        };

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(Ok(exp), parser.parse_relation());
    }

    #[test]
    fn error_eof_missing_relation_type() {
        let i = "define write as";
        let exp = Err(ParserError::UnexpectedEOF);

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(exp, parser.parse_relation());
    }

    #[test]
    fn error_expected_keyword_relation_type() {
        let i = "define write as type";
        let exp = Err(ParserError::UnexpectedKeyword(TokenKind::Type));

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(exp, parser.parse_relation());
    }

    #[test]
    fn can_parse_relation_multiple_alias() {
        let i = "define write as self or owner or thing";
        let exp = Relation {
            kind: "write".into(),
            aliases: vec![
                Alias {
                    kind: AliasKind::This,
                    parent: None,
                },
                Alias {
                    kind: AliasKind::Named("owner".into()),
                    parent: None,
                },
                Alias {
                    kind: AliasKind::Named("thing".into()),
                    parent: None,
                },
            ],
        };

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(Ok(exp), parser.parse_relation());
    }

    #[test]
    fn can_parse_relation_parent_alias() {
        let i = "define write as self or owner from parent or thing";
        let exp = Relation {
            kind: "write".into(),
            aliases: vec![
                Alias {
                    kind: AliasKind::This,
                    parent: None,
                },
                Alias {
                    kind: AliasKind::Named("owner".into()),
                    parent: Some("parent".into()),
                },
                Alias {
                    kind: AliasKind::Named("thing".into()),
                    parent: None,
                },
            ],
        };

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(Ok(exp), parser.parse_relation());
    }

    #[test]
    fn can_parse_doc() {
        let i = "type organization
  relations
    define member as self
type document
  relations
    define owner as self
    define can_share as owner or editor or owner from parent";
        let exp = Document {
            types: vec![
                Type {
                    kind: "organization".into(),
                    relations: vec![Relation {
                        kind: "member".into(),
                        aliases: vec![Alias {
                            kind: AliasKind::This,
                            parent: None,
                        }],
                    }],
                },
                Type {
                    kind: "document".into(),
                    relations: vec![
                        Relation {
                            kind: "owner".into(),
                            aliases: vec![Alias {
                                kind: AliasKind::This,
                                parent: None,
                            }],
                        },
                        Relation {
                            kind: "can_share".into(),
                            aliases: vec![
                                Alias {
                                    kind: AliasKind::Named("owner".into()),
                                    parent: None,
                                },
                                Alias {
                                    kind: AliasKind::Named("editor".into()),
                                    parent: None,
                                },
                                Alias {
                                    kind: AliasKind::Named("owner".into()),
                                    parent: Some("parent".into()),
                                },
                            ],
                        },
                    ],
                },
            ],
        };

        let lex = Lexer::new(i);
        let mut parser = Parser::new(lex);
        assert_eq!(Ok(exp), parser.parse_document());
    }
}
