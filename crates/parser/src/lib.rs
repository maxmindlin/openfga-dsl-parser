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
            let ty = self.parse_type()?;
            types.push(ty);
            self.next_token();
        }
        Ok(Document { types })
    }

    pub fn parse_type(&mut self) -> ParseResult<Type> {
        unimplemented!()
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
