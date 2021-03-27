use crate::lexer::utils;
use crate::lexer::expr::Expr;
use crate::lexer::token::Tkn;
use crate::lexer::token::Integer;

/// Variable define struct
pub struct VariableDef {
    name: String,
    val: Expr,
}

impl VariableDef {
    pub fn new(s: &str) -> (&str, Self) {
        /*
         * "a = 1 % 1" extract all of the whitespace out of the statement "a=1%1"
         * split on the operator to get the left and right hand of the expression
         * return a VarableDef { name: a, val: Expr { lhs: 1, rhs: 1, op: Tkn::_Mod } }

                        INPUT        OUTPUT
                    +-------------+------------
                    |  a = 1 % 1  |   a=1%1    |
                    +-------------+------------+
        **/
        let _extracted = utils::extract_whitespace(&s);

        let _name: Vec<&str> = _extracted.split("=").collect::<Vec<&str>>();

        let ops: [char; 4] = ['+','-','/','*'];
        let op = s.chars().find(|x| ops.contains(x));

        // Grab and parse the operator into a token
        let op = Tkn::new(&op.expect(&format!("Expected Operator: Valid Operators -> {:?}", ops)).to_string());

        // get right-side, and the left-side then stick it in a vec<&str>
        let split_on_op: Vec<&str> = _extracted.split::<_>(&op.into_str().into_owned()).collect::<Vec<&str>>();

        // Take both sides and put then into the integer strcut
        let lhs = Integer::new_from_str(&split_on_op[0]);
        let rhs = Integer::new_from_str(&split_on_op[1]);

        Self { name: _name, val: Expr { lhs, op, rhs }  }
    }
}
