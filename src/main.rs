use lexer::{Token};
use logos::Logos;

fn main() {
    let mut lex = Token::lexer(" type resource\n  relations\n    define page_viewer as viewer from owner but not blocked\n");

    println!("{:?}", lex.next());
    println!("{:?}", lex.span());
    println!("{:?}", lex.slice());

    loop {
        let next = lex.next();
        if next == None {
            break;
        }
        println!("{:?}", &next);
        println!("{:?}", lex.span());
        println!("{:?}", lex.slice());
    }


    // assert_eq!(lex.next(), Some(Token::Define));
    // assert_eq!(lex.span(), 0..6);
    // assert_eq!(lex.slice(), "define");

    // assert_eq!(lex.next(), Some(Token::Space));
    // assert_eq!(lex.span(), 6..7);
    // assert_eq!(lex.slice(), " ");

    // assert_eq!(lex.next(), Some(Token::From));
    // assert_eq!(lex.span(), 7..25);
    // assert_eq!(lex.slice(), "viewer from owner");

    // assert_eq!(lex.next(), None);
}