use std::str::FromStr;

use super::reserved::ReservedToken;

struct NumberInfo<'a> {
    whole_number: &'a str,
    fractional_part: &'a str,
}

impl<'a> ToString for NumberInfo<'a> {
    fn to_string(&self) -> String {
        format!(
            "(Number, whole number: {}, fractional part: {}",
            self.whole_number, self.fractional_part
        )
    }
}

pub enum Token<'a> {
    Identifier(&'a str),
    Reserved(&'a ReservedToken<'a>),
    Number(NumberInfo<'a>),
    Str(&'a str),
    Char(&'a str),
}

impl<'a> ToString for Token<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::Identifier(s) => format!("(Identifier, name: {})", s),
            Self::Reserved(token) => match token {
                ReservedToken::Char(c) => format!("(Reserved Char, value: {})", c),
                ReservedToken::Keyword(keyword) => format!("(Keyword, name: {})", keyword),
                ReservedToken::Operator(op, _) => format!("(Operator, value: {})", op),
            },
            Self::Number(info) => info.to_string(),
            Self::Char(c) => format!("(Char Literal, value: {})", c),
            Self::Str(s) => format!("(String Literal, value: {})", s),
        }
    }
}
