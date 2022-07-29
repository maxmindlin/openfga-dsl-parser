pub mod token;

use token::*;

#[derive(Default)]
pub struct Lexer {
    input: Vec<char>, // todo: make this an iterable so we dont have to collect
    pos: usize,
    read_pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            ..Default::default()
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.next() {
            Some(c) => {
                if c.is_whitespace() {
                    self.next_token()
                } else if is_valid_text(&c) {
                    let lit = self.read_text();
                    match TokenKind::is_to_keyword(&lit) {
                        Some(keyword) => Token::new(lit, keyword),
                        None => Token::new(lit, TokenKind::Text),
                    }
                } else {
                    Token::new(c.to_string(), TokenKind::Illegal)
                }
            }
            None => Token::new("".into(), TokenKind::EOF),
        }
    }

    fn next(&mut self) -> Option<&char> {
        let out = self.input.get(self.read_pos);
        self.pos = self.read_pos;
        self.read_pos += 1;
        out
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.read_pos)
    }

    fn read_text(&mut self) -> String {
        let start = self.pos;
        let mut i = vec![self.input[start]];
        while self.peek().is_some() && is_valid_text(self.peek().unwrap()) {
            i.push(*self.next().unwrap());
        }
        i.iter().collect()
    }
}

fn is_valid_text(c: &char) -> bool {
    c.is_alphanumeric() || *c == '_' || *c == '-'
}

// use logos::Logos;
//
// #[derive(Logos, Debug, PartialEq)]
// pub enum Token {
//     // Tokens can be literal strings, of any length.
//
//     #[token("type")]
//     Type,
//
//     #[token("relations")]
//     Relations,
//
//     #[token("define")]
//     Define,
//
//     #[token("as")]
//     As,
//
//     #[token("self")]
//     This,
//
//     #[token("or")]
//     Or,
//
//     #[token("and")]
//     And,
//
//     #[token("but not")]
//     Difference,
//
//     // #[regex(r"[a-z_]+\sfrom\s[a-z_]+")]
//     #[token("from")]
//     From,
//
//     // // Or regular expressions.
//     #[regex("[a-z_]+", priority = 1)]
//     Text,
//
//     // #[token(" ")]
//     // Space,
//
//     // Logos requires one token variant to handle errors,
//     // it can be named anything you wish.
//     #[error]
//     // We can also use this variant to define whitespace,
//     // or any other matches we wish to skip.
//     #[regex(r"[ \t\n]+", logos::skip)]
//     Error,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_type() {
        let i = "type document";
        let mut l = Lexer::new(i);
        assert_eq!(l.next_token(), Token::new("type".into(), TokenKind::Type));
        assert_eq!(
            l.next_token(),
            Token::new("document".into(), TokenKind::Text)
        );
        assert_eq!(l.next_token(), Token::new("".into(), TokenKind::EOF));
    }

    #[test]
    fn parse_type_newline() {
        let i = "type document
";
        let mut l = Lexer::new(i);
        assert_eq!(l.next_token(), Token::new("type".into(), TokenKind::Type));
        assert_eq!(
            l.next_token(),
            Token::new("document".into(), TokenKind::Text)
        );
        assert_eq!(l.next_token(), Token::new("".into(), TokenKind::EOF));
    }

    #[test]
    fn full() {
        let i = "type document
    relations
        define parent as self or thing or other_thing from parent";

        let mut l = Lexer::new(i);
        assert_eq!(l.next_token(), Token::new("type".into(), TokenKind::Type));
        assert_eq!(
            l.next_token(),
            Token::new("document".into(), TokenKind::Text)
        );
        assert_eq!(
            l.next_token(),
            Token::new("relations".into(), TokenKind::Relations)
        );
        assert_eq!(
            l.next_token(),
            Token::new("define".into(), TokenKind::Define)
        );
        assert_eq!(l.next_token(), Token::new("parent".into(), TokenKind::Text));
        assert_eq!(l.next_token(), Token::new("as".into(), TokenKind::As));
        assert_eq!(l.next_token(), Token::new("self".into(), TokenKind::This));
        assert_eq!(l.next_token(), Token::new("or".into(), TokenKind::Or));
        assert_eq!(l.next_token(), Token::new("thing".into(), TokenKind::Text));
        assert_eq!(l.next_token(), Token::new("or".into(), TokenKind::Or));
        assert_eq!(
            l.next_token(),
            Token::new("other_thing".into(), TokenKind::Text)
        );
        assert_eq!(l.next_token(), Token::new("from".into(), TokenKind::From));
        assert_eq!(l.next_token(), Token::new("parent".into(), TokenKind::Text));
        assert_eq!(l.next_token(), Token::new("".into(), TokenKind::EOF));
    }
}
