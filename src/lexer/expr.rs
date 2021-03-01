use crate::lexer::Integer;
use crate::lexer::Tkn;

/// Exprs
///
/// We are just defining what exactly an Expr is going to look like. We can define the Expr in a couple of ways.
/// Expr also implements some nice creature comforts for the compiler
/// ```
///Expr::new(2, 2, Tkn::Plus),
///Expr {
///    lhs: Integer::new_from_char('2'),
///    rhs: Integer::new_from_char('2'),
///    op: Tkn::Plus
///}
///```
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Integer,
    pub rhs: Integer,
    pub op: Tkn, // this will only work with certain enum fields
}

impl Expr {
    pub fn new_from_str(s: &str) -> Self {
        // get Integer from &str
        let lhs: Integer = Integer::new_from_str(&s, 0);
        let rhs: Integer = Integer::new_from_str(&s, 2);

        let get_operator = &s.chars().skip(1).next().expect("Could not get operator");
        let op = Tkn::new(&get_operator.to_string());

        // we just wanted to get the operator damnit rust
        // let get_operator: &u8 = &s.as_bytes()[1];
        // let chard_operator: char = *get_operator as char;
        // let op: Tkn = Tkn::new(&chard_operator.to_string());

        Expr { lhs, rhs, op }
    }

    pub fn new(lhs: u8, rhs: u8, op: Tkn) -> Self {
        let lhs = Integer::new(lhs);
        let rhs = Integer::new(rhs);
        Self { lhs, rhs, op }
    }
}
