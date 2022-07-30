pub mod ast;
pub mod json;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e2e_json_serialize() {
        let i = "type organization
  relations
    define member as self
type document
  relations
    define 0af-doc_owner as self
    define _editor as self or 0af-doc_owner
    define -parent as self
    define can_share as 0af-doc_owner or _editor or 0af-doc_owner from -parent";

        let lex = lexer::Lexer::new(i);
        let mut parser = parser::Parser::new(lex);
        let doc = parser.parse_document().unwrap();

        let jsont = json::JsonTransformer::new(&doc);
        let json = jsont.serialize();
        println!("{json}");
    }
}
