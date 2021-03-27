use logos::Logos;

struct Lexer{}

#[derive(Logos, Debug, PartialEq)]
enum Token {

    #[token("[0-9]+", |lex| lex.slice().parse())]
    Int(u64),

    #[regex("[a-zA-z]")]
    Char(char),

    #[token(" ")]
    Whitespace,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("=")]
    Equals,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrace,

    #[token("]")]
    RBrace,

    #[token("##")]
    Comment,

    #[token("_")]
    UnderScore,

    #[token("&")]
    Ampersand,

    #[token("^")]
    Carot,

    #[token("%")]
    Mod,

    #[token("$")]
    Dollar,

    #[token("@")]
    At_,

    #[token("!")]
    Exclamation,

    #[token("!+")]
    NotEqual,

    #[token("++")]
    Increment,

    #[token("--")]
    Decrement,

    #[token("-=")]
    MinusEqual,

    #[token("<<")]
    ShiftLeft,

    #[token(">>")]
    ShiftRight,

    #[token(">")]
    LessThan,

    #[token("<")]
    GreaterThan,

    #[token("|")]
    Pipe,

    #[token("\\")]
    Escape,

    #[token("/")]
    Div,

    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token("?")]
    Question,

    #[token("'")]
    SingleQuote,

    #[token("\"")]
    DoubleQuote,

    #[token("{")]
    LeftCurly,

    #[token("}")]
    RightCurly,

    #[error]
    Error
}

impl Lexer {
    pub fn lex(path: String) {
        let mut results: Vec<Token>;
        let mut lex = Token::lexer("1  + 1");
        let cursor = 0;

        // loop through all of the token if there are any
        // assign the next token and store the previous one
        loop {
            if let next_token = lex.next() {
                match next_token {
                    Some(x) => { let next_token = Some(x); },
                    _ => { panic!("impl_lexer_1loop_next_token->{ No Token Found! }"); }
                }
            }
            cursor += 1;
        } // end of scope loop
        dbg!(results);
    }
}
