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
    // expects a char
    pub lhs: Integer,
    pub rhs: Integer,
    pub op: Tkn, // this will only work with certain enum fields
}

impl Expr {
    pub fn new_from_str(s: &str) -> Self {

        let _extracted: String  = Expr::extract_whitespace(&s);
        let new_str = _extracted.to_owned();
        println!("new_str->{:?}", new_str);
        let split_on_op = new_str.split("+").collect::<Vec<&str>>();
        println!("split_on_op_vec->{:?}", split_on_op);
        let lhs: Integer = Integer::new_from_str(&split_on_op[0]);
        let rhs: Integer = Integer::new_from_str(&split_on_op[1]);
        // '1 + 1'
        // should extract white space here because this function is expecting 
        // '1+1'

        //let get_operator = &new_str.chars().next().expect("Could not get operator");
        //let op = Tkn::new(&get_operator.to_string());
        //let op_str = &new_str.chars().position(|c| c == '+').unwrap().to_string();
        //assert_eq!(op_str, "+");
        
        let plus_index = s.to_string().find("+");
        let op_str: String = s.to_string()
            .chars()
            .skip(plus_index.unwrap() as usize)
            .take(1)
            .collect();

        let op = Tkn::new(&op_str.to_owned());
        println!("operator->{:?}", op);
        println!("operator->{:?}", op_str);
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
