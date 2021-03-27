#[warn(unused_doc_comments)]
#[allow(unused_macros)]
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
            kind: Tkn::Int(char_.unwrap().to_string()),
            int: val,
        }
    }
    ///we create our own function for char conversion because we want to return the struct for readablity
    pub fn new_from_char(c: char) -> Self {
        // lex a u8 from the string from string
        // alert user if char did not lex to u8 correctly
        let int: u8 = c.to_string().parse::<u8>().expect("lexer::Integer::new_from_char->Did not match u8 spec");
        Integer {
            kind: Tkn::Int(c.to_string()),
            int,
        }
    }

    pub fn new_from_str(s: &str) -> Self {
        // this function is now fed a string_slice that represent a integer
        let int = s
            .to_string()
            .parse::<u8>()
            .expect("lexer::Integer::new_from_str->did not match u8 spec");

        Integer {
            kind: Tkn::Int(s.to_string()),
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
                    int,
                }
            }
            _ => panic!("NOT A TKN::INT"),
        }
    }
}

/// Tkn is the base of lexer/lexr here
#[derive(Debug, PartialEq)]
pub enum Tkn {
    Int(String),  // please use the Integer struct for this. Tkn is just a kind
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
    LeftCurly,
    RightCurly,
}

impl<'tkn> Tkn {
    pub fn into_str(&'tkn self) -> std::borrow::Cow<'tkn, str>{
        match self {
           Self::Int(s) => s.as_ref(),
           Self::Char(c) => return c.to_string().into(),
           Self::Whitespace => " ",
           Self::Plus => "+",
           Self::Minus => "-",
           Self::Star => "*",
           Self::Equals => "=",
           Self::LParen => "(",
           Self::RParen => ")",
           Self::LBrace => "[",
           Self::RBrace => "]",
           Self::Comment => "#",
           Self::UnderScore => "_",
           Self::Ampersand => "&",
           Self::Carot => "^",
           Self::Mod => "%",
           Self::Dollar => "$",
           Self::At_ => "@",
           Self::Exclamation => "!",
           Self::NotEqual => "!=",
           Self::Increment => "++",
           Self::Decrement => "--",
           Self::MinusEqual => "-=",
           Self::ShiftLeft => "<<",
           Self::ShiftRight => ">>",
           Self::LessThan => ">",
           Self::GreaterThan => "<",
           Self::Pipe => "|",
           Self::Escape => "\\",
           Self::Div => "/",
           Self::Period => ".",
           Self::Comma => ",",
           Self::Question => "?",
           Self::SingleQuote => "'",
           Self::DoubleQuote => "\"",
           Self::LeftCurly => "{",
           Self::RightCurly => "}",
        }.into()
    }

    pub fn new(val: &str) -> Self {
        if val.len() == 1 {
            let val = val.chars().next().unwrap();
            match val {
                // Integer::new_from_char(val, Tkn::Int) later on
                '0'..='9' => Self::Int(val.to_string()),
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
    use crate::lexer::VariableDef;
    #[test]
    fn expr_lex() {
        let the_string: String = String::from("1+1");
        let lexed = Expr::new_from_str(&the_string);
        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int("1".to_string()),
                int: 1,
            },
            rhs: Integer {
                kind: Tkn::Int("1".to_string()),
                int: 1,
            },
            op: Tkn::Plus,
        };

        assert_eq!(lexed, match_);
    }

    #[test]
    //#[ignore]
    fn expr_lex_whitespace() {
        let the_string: String = String::from("1 + 1");
        let lexed = Expr::new_from_str(&the_string);
        // return char to get rid of new_from_str

        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int("1".to_string()),
                int: 1,
            },
            rhs: Integer {
                kind: Tkn::Int("1".to_string()),
                int: 1,
            },
            op: Tkn::Plus,
        };
        assert_eq!(lexed, match_);
    }

    #[test]
    //#[ignore]
    fn expr_lex_whitespace_extra_whitespace() {
        let the_string: String = String::from("100      /   58");
        let lexed = Expr::new_from_str(&the_string);
        // return char to get rid of new_from_str

        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int("100".to_string()),
                int: 100,
            },
            rhs: Integer {
                kind: Tkn::Int("58".to_string()),
                int: 58,
            },
            op: Tkn::Div,
        };
        assert_eq!(lexed, match_);
    }



    #[test]
    fn expr_lex_whitespace_multichar_int() {
        let the_string: String = String::from("122 + 12");
        let lexed = Expr::new_from_str(&the_string);
        // return char to get rid of new_from_str

        let match_ = Expr {
            lhs: Integer {
                kind: Tkn::Int("122".to_string()),
                int: 122,
            },
            rhs: Integer {
                kind: Tkn::Int("12".to_string()),
                int: 12,
            },
            op: Tkn::Plus,
        };
        assert_eq!(lexed, match_);
    }


    #[test]
    fn lex_variable_def() {
        let the_string: String = String::from("122 + 12");
        let lexed = Expr::new_from_str(&the_string);
        VariableDef {
            name: r#"a"#.to_string(),
            val: Expr {
                lhs: Integer {
                    kind: Tkn::Int("122".to_string()),
                    int: 122,
                },
                rhs: Integer {
                    kind: Tkn::Int("12".to_string()),
                    int: 12,
                },
                op: Tkn::Plus,
            assert_eq!(lexed, match_),
            }
        }
    }

    #[test]
    fn lex_plus() {
        assert_eq!(Tkn::Plus, Tkn::new("+"));
    }

    #[test]
    fn lex_mod() {
        assert_eq!(Tkn::Mod, Tkn::new("%"));
    }

    #[test]
    fn lex_star() {
        assert_eq!(Tkn::Star, Tkn::new("*"));
    }

    #[test]
    fn lex_int() {
        assert_eq!(Tkn::Int("2".to_string()), Tkn::new("2"));
    }

    #[test]
    fn lex_char() {
        assert_eq!(Tkn::Char('a'), Tkn::new("a"));
    }

    #[test]
    fn lex_div() {
        assert_eq!(Tkn::Div, Tkn::new("/"));
    }

    #[test]
    fn lex_comment() {
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
