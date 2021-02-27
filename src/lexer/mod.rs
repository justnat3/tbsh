#[warn(unused_doc_comments)]
#[derive(Debug)]
pub enum Token_ {
    Whitespace,
    Int(u64),
    Plus(char),
    Minus(char),
    Star(char),
    Slash(char),
    Equals(char),
    LParen(char),
    RParen(char),
    LBrace(char),
    RBrace(char),
    Comment(char),
    UnderScore(char),
    Char(char),
}


pub fn lex(input: &String) -> Result<Vec<Token_>, String> {
    let mut result = Vec::new();
    let mut input_text = input.chars();
    while let Some(c) = input_text.next() {
        match c {
            c if c.is_whitespace() => result.push(Token_::Whitespace),
            '0'..='9' => {
                // let int = get_int(c, &mut input_text);
                let err_string = vec!["Not an Int ".to_string(), c.to_string()].join("-");
                let int: u64 = c.to_string().parse::<u64>().expect(&err_string[..]);
                result.push(Token_::Int(int));
            }
            '+' => result.push(Token_::Plus(c)),
            '=' => result.push(Token_::Equals(c)),
            '-' => result.push(Token_::Minus(c)),
            '*' => result.push(Token_::Star(c)),
            '/' => result.push(Token_::Slash(c)),
            '\\' => result.push(Token_::Slash(c)),
            ')' => result.push(Token_::RParen(c)),
            '(' => result.push(Token_::LParen(c)),
            '}' => result.push(Token_::LBrace(c)),
            '{' => result.push(Token_::RBrace(c)),
            '#' => result.push(Token_::Comment(c)),
            '_' => result.push(Token_::UnderScore(c)),
            _ => {
                // we assume that the default is just a rand char
                // there should be no actual panic! here
                result.push(Token_::Char(c));
            }
        };
    }
    Ok(result)
}
