use ast::{Document, Type};
use lexer::{Lexer, token::{Token, TokenKind}};

pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    lex: Lexer,
    curr: Token,
    peek: Token,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(TokenKind, TokenKind),
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
                return Err(ParserError::UnexpectedToken(TokenKind::Type, self.curr.kind()));
            }
            let ty = self.parse_type()?;
            types.push(ty);
            self.next_token();
        }
        Ok(Document { types })
    }

    pub fn parse_type(&mut self) -> ParseResult<Type> {
        self.expect_peek(TokenKind::Text)?;
        let kind = self.curr.literal().to_string();
        let relations = Vec::new();

        if self.peek.kind() != TokenKind::EOF
            && self.peek.kind() != TokenKind::Type
        {
            self.expect_peek(TokenKind::Relations)?;

            // parse relations
        }

        Ok(Type { kind, relations })
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
            ]
        };
        assert_eq!(exp, parser.parse_document().unwrap());
    }
}
