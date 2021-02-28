// pub fn lex(input: &String) -> Vec<Tkn> {
//     let mut results = Vec::new();

//     //parse tkns from string


//     results
// }
// pub fn lex(input: &String) -> Vec<Option<Tkn>> {
//     let mut result = Vec::new();
//     let mut input_text = input.chars();
//     while let Some(c) = input_text.next() {
//         // rip all of that code
//     }
//     result
// }

// #[cfg(test)]
// mod tests {
//     use crate::lexer::lexer;
//     use crate::lexer::token::*;

//     #[test]
//     fn parse_string() {
//         let the_string: String = String::from("1 + 1");
//         let lexed = lexer::lex(&the_string);
//         let match_: Vec<Tkn> = vec![
//             Tkn::Int,
//             Tkn::Whitespace,
//             Oper::Plus,
//             Tkn::Whitespace,
//             Tkn::Int,
//         ];
//         assert_eq!(lexed, match_);
//     }
// }