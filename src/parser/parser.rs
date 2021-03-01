// use crate::lexer::*;

// pub struct Parser {}

// pub fn parse<I>(iter: I)
// where
//     I: Iterator<Item = Tkn>,
// {
//     // IF THERE IS some->Tkn::Int->Tkn::Operator->Tkn::Int->delimiter->Nothing('\n') then parse_expression->Exper
//     let mut c = 0;
//     let mut results = Vec::new();

//     let thing: Vec<Tkn> = iter.collect::<Vec<Tkn>>();
//     println!("{:?}", thing);
//     loop {
//         println!("{}{}", thing.len(), c);
//         if c + 1 >= thing.len() {
//             break;
//         }
//         // This does not work because at the next iteration of course the next element at the end is not going to be a int
//         if matches!(thing[c], Tkn::Int(_))
//             // && matches!(thing[c + 1], lexer::Tkn::Whitespace)
//             && matches!(thing[c + 2], Op::Plus)
//             // && matches!(thing[c + 3], lexer::Tkn::Whitespace)
//             && matches!(thing[c + 4], Tkn::Int(_))
//         {
//             results.push((&thing[c], &thing[c + 2], &thing[c + 4]));
//         } else {
//             println!("None Found");
//         }
//         c += 1;
//     }
//     println!("{:?}", results);
//     &results;
// }
