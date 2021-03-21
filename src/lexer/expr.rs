use crate::lexer::Integer;
use crate::lexer::Tkn;

/// Exprs
///-
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
        // get index of operator in expression
        let ops: [char; 4] = ['+','-','/','*'];
        let op = s.chars().find(|x| ops.contains(x));

        // Grab and parse the operator into a token
        let op = Tkn::new(&op.expect(&format!("Expected Operator: Valid Operators -> {:?}", ops)).to_string());

        // get all the whitespace out of the expression
        let _extracted: String  = Expr::extract_whitespace(&s).to_owned();

        // get right-side, and the left-side then stick it in a vec<&str>
        let split_on_op = _extracted.split(&op.into_str().into_owned()).collect::<Vec<&str>>();

        // Take both sides and put then into the integer strcut
        dbg!(&_extracted);
        let lhs: Integer = Integer::new_from_str(&split_on_op[0]);
        let rhs: Integer = Integer::new_from_str(&split_on_op[1]);

        Expr { lhs, rhs, op }
    }

    pub fn extract_whitespace(s: &str) -> String {
        if !s.to_string().contains(char::is_whitespace) {
            return s.to_string();
        }
        let results = s.split_whitespace().collect::<String>();
        // we return a String because we do not want to return a reference
        // the ref data is owned by the function
        results
    }

    pub fn new(lhs: u8, rhs: u8, op: Tkn) -> Self {
        let lhs = Integer::new(lhs);
        let rhs = Integer::new(rhs);
        Self { lhs, rhs, op }
    }
}
