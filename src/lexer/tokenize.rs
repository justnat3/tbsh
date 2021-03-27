use logos::Logos;

pub struct Lexer {}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Int(u64),

    /*
    if there is a string with whitespace
    you will end up with DoubleQuote String String String String DoubleQuote

    EXAMPLE:
    " kinda like this string "

    Something to keep in mind here since we do not read whitespace, we throw it to Error Varient
    */
    #[regex("[a-zA-z]+")]
    String_,

   // #[regex(" +")]
   // Whitespace,

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

    /// Something to note here if you have a string with whitespace
    /// the whitespace will ignored then the string will just contain a bunch of substrings
    ///
    /// &"a_number = \"What up brotado chip\""
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error
}

impl Lexer {
    /// Lex spits out tokens
    /// Lex Takes in a "buffer" which is just a string slice
    #[allow(dead_code)]
    pub fn lex(buffer: &str) -> Vec<Token> {
        let mut results: Vec<Token> = vec![];
        let mut lex = Token::lexer(buffer);
        loop {
            let some_token = lex.next();
            if some_token.is_none() { break; } else { results.push(some_token.unwrap()); };
        }
        results
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn lexer_test_assignment_with_expression() {
        let lex = Lexer::lex(&"a_number=1+1");
        // we can run cargo -- --nocapture to get this debug output
        dbg!(&lex);
        assert_eq!(
            lex,
            [
                Token::String_,
                Token::Equals,
                Token::Int(1),
                Token::Plus,
                Token::Int(1),
            ]
        )
    }
    #[test]
    fn lexer_expression_with_lots_of_spaces() {
        let lex = Lexer::lex(&"102020    /   122");
        // we can run cargo -- --nocapture to get this debug output
        dbg!(&lex);
        assert_eq!(
            lex,
            [
                Token::Int(102020),
                Token::Div,
                Token::Int(122),
            ]
        )
    }
    #[test]
    fn lexer_test_assignment_of_string() {
        let lex = Lexer::lex(&"a_number = \"What up brotado chip\"");
        // we can run cargo -- --nocapture to get this debug output
        dbg!(&lex);
        assert_eq!(
            lex,
            [
                Token::String_,
                Token::Equals,
                Token::DoubleQuote,
                Token::String_,
                Token::String_,
                Token::String_,
                Token::String_,
                Token::DoubleQuote,
            ]
        )
    }
}
