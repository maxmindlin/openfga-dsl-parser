use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Tokens can be literal strings, of any length.

    #[token("type")]
    Type,

    #[token("relations")]
    Relations,

    #[token("define")]
    Define,

    #[token("as")]
    As,

    #[token("self")]
    This,

    #[token("or")]
    Or,

    #[token("and")]
    And,

    #[token("but not")]
    Difference,

    // #[regex(r"[a-z_]+\sfrom\s[a-z_]+")]
    #[token("from")]
    From,

    // // Or regular expressions.
    #[regex("[a-z_]+", priority = 1)]
    Text,

    // #[token(" ")]
    // Space,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n]+", logos::skip)]
    IsThisTheERROR,
}