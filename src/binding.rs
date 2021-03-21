mod expr;

pub struct VariableDef {
    name: String,
    val: expr::Expr,
}

impl VariableDef {
    pub fn new(s: &str) -> (&str, Self) {
        // we don't start with a keyword
        
        // start with var_name: datatype = thing
        //
    }
}
