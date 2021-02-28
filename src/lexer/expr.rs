use crate::lexer::Integer;
use crate::lexer::Tkn;

/// Expressions
/// 
/// We are just defining what exactly an expression is going to look like. We can define the expression in a couple of ways.
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
        let chars = s.chars().into_iter();
        for (i, char) in chars.enumerate() {
            // I need to call some lex here
        }
        Expr {
            lhs: Integer {
                kind: Tkn::Int('2'),
                int: 2,
            },
            rhs: Integer {
                kind: Tkn::Int('2'),
                int: 2,
            },
            op: Tkn::Plus,
        }
    }

    pub fn new(lhs: u8, rhs: u8, op: Tkn) -> Self {
        let lhs = Integer::new(lhs);
        let rhs = Integer::new(rhs);
        let op = match op {
            Tkn::Plus => Tkn::Plus,
            Tkn::Minus => Tkn::Plus,
            Tkn::Div => Tkn::Div,
            Tkn::Mod => Tkn::Mod,
            Tkn::Star => Tkn::Star,
            _ => panic!("Not a valid operator"),
        };
        Self { lhs, rhs, op }
    }
}
