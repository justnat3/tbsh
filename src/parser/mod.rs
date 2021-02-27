use crate::lexer;
pub struct Parser {}

impl Parser {
    fn parse<I>(iter: I)
    where 
        I: Iterator<Item = lexer::Token_>
    {
        // IF THERE IS some->Token::Int->Token::Operator->Token::Int->delimiter->Nothing('\n') then parse_expression->Exper
        // we should 
        
    }

    fn parse_expression() {}
}
