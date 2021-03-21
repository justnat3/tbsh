#[warn(unused_doc_comments)]

// Contribution of NYX
macro_rules! crash {
    ($var:expr, $($fmt:tt)*) => {
        {
            let v = $var;
            let msg = format!($($fmt)*, v);
            v.expect(msg.as_str())
        }
    };
    ($var:expr, $($fmt:tt)*) => {
        {
            let v = $var;
            v.expect(format!($($fmt)*, v).as_str())
        }
    };
}

/// We define this struct as an integer because integers are whole numbers
#[derive(Debug, PartialEq)]
pub struct Integer {
    pub kind: Tkn,
    pub int: u8,
}

/// Tkn::Char_ be able to extract the value and be able to hold the char kind
#[derive(Debug, PartialEq)]
pub struct Char_ {
    kind: Tkn,
    char: char,
}

impl Integer {
    /// so from 0 up to 9 and then multichar numbers can later be packed into this struct too
    pub fn new(val: u8) -> Self {
        // this is not entirely safe, as std::char::from_u32 would be prefered
        // previously the tests failed because first expr->kind == '\u{2}'
        let char_ = std::char::from_digit(val.into(), 10);

        Integer {
            kind: Tkn::Int(crash!(char_, "Invalid kind in Integer Struct {:?}")),
            int: val,
        }
    }
    ///we create our own function for char conversion because we want to return the struct for readablity
    pub fn new_from_char(c: char) -> Self {
        // parse a u8 from the string from string
        // alert user if char did not parse to u8 correctly
        let int: u8 = c.to_string().parse::<u8>().expect("Did not match u8 spec");
        Integer {
            kind: Tkn::Int(c),
            int,
        }
    }

    pub fn new_from_str(s: &str, inx: usize) -> Self {
        //  another thing we can do is split on the Whitespace token
        //  this is so that when we have multichar ints we can capture and parse
        //  the entire fucking int
        //
        // this is reading the entire string index is not specified
       

        println!("parsed: ");
        let ch = s.chars().nth(inx).unwrap();

        //let int: u8 = s.to_string().as_bytes()[inx];
        //let conv = int as char;
        println!("{:?}", ch);
        let int = ch 
            .to_string()
            .parse::<u8>()
            .expect("did not match u8 spec");

        println!("{}", int);
        let get_char: char = s.chars().nth(inx).unwrap();
        Integer {
            kind: Tkn::Int(get_char),
            int,
        }
    }
    /// The Tkn enum in this case Tkn::Int(_) can hold a single value representing what char->Int it
    /// infact is so we return a struct for integer based on the int value in the Tkn
    pub fn from_tkn(tkn: Tkn) -> Self {
        match tkn {
            Tkn::Int(val) => {
                let int: u8 = val
                    .to_string()
                    .parse::<u8>()
                    .expect("Did not match u8 spec");
                Integer {
                    kind: Tkn::Int(val),
                    int: int,
                }
            }
            _ => panic!("NOT A TKN::INT"),
        }
    }
}

/// Tkn is the base of lexer/parser here
#[derive(Debug, PartialEq)]
pub enum Tkn {
    Int(char),  // please use the Integer struct for this. Tkn is just a kind
    Char(char), // please use the Char struct for this. Tkn is just a kind
    Whitespace,
    Plus,
    Minus,
    Star,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comment,
    UnderScore,
    Ampersand,
    Carot,
    Mod,
    Dollar,
    At_,
    Exclamation,
    NotEqual,
    Increment,
    Decrement,
    MinusEqual,
    ShiftLeft,
    ShiftRight,
    LessThan,
    GreaterThan,
    Pipe,
    Escape,
    Div,
    Period,
    Comma,
    Question,
    SingleQuote,
    DoubleQuote,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
}

impl Tkn {
    pub fn new_from_char(val: char) -> Self {
        match val {
            '0'..='9' => Self::Int(val),
            '=' => Self::Equals,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '_' => Self::UnderScore,
            ')' => Self::RParen,
            '(' => Self::LParen,
            '*' => Self::Star,
            '&' => Self::Ampersand,
            '^' => Self::Carot,
            '%' => Self::Mod,
            '$' => Self::Dollar,
            '#' => Self::Comment,
            '@' => Self::At_,
            '!' => Self::Exclamation,
            '<' => Self::GreaterThan,
            '>' => Self::LessThan,
            '|' => Self::Pipe,
            '\\' => Self::Escape,
            '/' => Self::Div,
            '.' => Self::Period,
            ',' => Self::Comma,
            '?' => Self::Question,
            '\'' => Self::SingleQuote,
            '\"' => Self::DoubleQuote,
            ']' => Self::RightBracket,
            '[' => Self::LeftBracket,
            '{' => Self::RightCurly,
            '}' => Self::LeftCurly,
            _ => Self::Char(val),
        }
    }

    pub fn new(val: &str) -> Self {
        if val.len() == 1 {
            let val = val.chars().next().unwrap();
            match val {
                // Integer::new_from_char(val, Tkn::Int) later on
                '0'..='9' => Self::Int(val),
                '=' => Self::Equals,
                '+' => Self::Plus,
                '-' => Self::Minus,
                '_' => Self::UnderScore,
                ')' => Self::RParen,
                '(' => Self::LParen,
                '*' => Self::Star,
                '&' => Self::Ampersand,
                '^' => Self::Carot,
                '%' => Self::Mod,
                '$' => Self::Dollar,
                '#' => Self::Comment,
                '@' => Self::At_,
                '!' => Self::Exclamation,
                '<' => Self::GreaterThan,
                '>' => Self::LessThan,
                '|' => Self::Pipe,
                '\\' => Self::Escape,
                '/' => Self::Div,
                '.' => Self::Period,
                ',' => Self::Comma,
                '?' => Self::Question,
                '\'' => Self::SingleQuote,
                '\"' => Self::DoubleQuote,
                ']' => Self::RightBracket,
                '[' => Self::LeftBracket,
                '{' => Self::RightCurly,
                '}' => Self::LeftCurly,
                _ => Self::Char(val),
            }
        } else {
            match val {
                "!=" => Self::NotEqual,
                "++" => Self::Increment,
                "-=" => Self::MinusEqual,
                "<<" => Self::ShiftLeft,
                ">>" => Self::ShiftRight,
                _ => panic!("NOT VALID CHAR or &STR-SLICE {}", val),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::expr::Expr;
    use crate::lexer::Integer;
    use crate::lexer::Tkn;
    #[test]
    fn expr_parse() {
        let the_string: String = String::from("1+1");
        let lexed = Expr::new_from_str(&the_string);
        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int('1'),
                int: 1,
            },
            rhs: Integer {
                kind: Tkn::Int('1'),
                int: 1,
            },
            op: Tkn::Plus,
        };

        assert_eq!(lexed, match_);
    }

    #[test]
    //#[ignore]
    fn expr_parse_whitespace() {
        let the_string: String = String::from("1 + 1");
        let lexed = Expr::new_from_str(&the_string);
        // return char to get rid of new_from_str

        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int('1'),
                int: 1,
            },
            rhs: Integer {
                kind: Tkn::Int('1'),
                int: 1,
            },
            op: Tkn::Plus,
        };
        assert_eq!(lexed, match_);
    }

    #[test]
    fn parse_plus() {
        assert_eq!(Tkn::Plus, Tkn::new("+"));
    }

    #[test]
    fn parse_mod() {
        assert_eq!(Tkn::Mod, Tkn::new("%"));
    }

    #[test]
    fn parse_star() {
        assert_eq!(Tkn::Star, Tkn::new("*"));
    }

    #[test]
    fn parse_int() {
        assert_eq!(Tkn::Int('2'), Tkn::new("2"));
    }

    #[test]
    fn parse_char() {
        assert_eq!(Tkn::Char('a'), Tkn::new("a"));
    }

    #[test]
    fn parse_div() {
        assert_eq!(Tkn::Div, Tkn::new("/"));
    }

    #[test]
    fn parse_comment() {
        assert_eq!(Tkn::Comment, Tkn::new("#"));
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            Expr::new(2, 2, Tkn::Plus),
            Expr {
                lhs: Integer::new_from_char('2'),
                rhs: Integer::new_from_char('2'),
                op: Tkn::Plus
            }
        );
    }
}
