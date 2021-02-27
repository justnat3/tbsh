pub mod lexer;
pub mod utils;
pub mod parser;
// fn main() {

//     // skip first index, first index is useless
//     // let arguments: Vec<String> = std::env::args().skip(1).collect();
//     let args: Vec<String> = std::env::args().collect();

//     // Check if args are empty, in which case you can go ahead and print the help text.
//     if args.is_empty() {
//         println!("INVALID INPUT: Enter a filepath -> len: {:?}", args.len());
//     }

//     println!("{:?}", args[1]);
//     // this is a random char we set as the initial char
//     let curs = '\r';
//     // ideally we will have a file path here
//     let f_data = lexer::Lexer::get_lexer_data_buffer(&args[1]);
//     let mut lex = lexer::Lexer::new(f_data, curs, 32, 0);

//     // mostly a test to see if
//     for i in 0..lex.data_.len() {
//         println!("{:?} Char at -> {:?}", lex.cursor, lex.col);

//         lexer::Lexer::point_cursor_at_next_char(&mut lex);
//         lexer::Lexer::move_cursor(&mut lex);
//     }
// }
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let file: String = utils::get_lexer_data_buffer(&args[1]);
    let res = lexer::lex(&file);
    dbg!(res);
}


// #[cfg(test)]
// mod tests {
// #[test]
//     fn parse_expr_math() {
//         let file = include_str!("./expression.tbsh").to_owned();
//         let contents = utils::get_lexer_data_buffer(&file);
//         let lex_it = lexer::lex(&contents);
//         println!("{}", lex_it);
//         parser::parse(lex_it)
//     }
// }