use super::reserved::ReservedToken;

pub struct NumberInfo<'a> {
    pub whole_number: &'a str,
    pub fractional_part: Option<&'a str>,
}

impl<'a> ToString for NumberInfo<'a> {
    fn to_string(&self) -> String {
        match self.fractional_part {
            Some(f) => format!(
                "(Number, whole number: {}, fractional part: {})",
                self.whole_number, f
            ),
            _ => format!("(Number, whole number: {})", self.whole_number),
        }
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
            Self::Identifier(s) => format!("(Identifier, Name: {})", s),
            Self::Reserved(token) => match token {
                ReservedToken::Char(c) => {
                    if *c != '\n' {
                        return format!("(Reserved Char, Value: '{}')", c);
                    }
                    format!("(Reserved Char, Value: '\\n')")
                }
                ReservedToken::Keyword(keyword) => format!("(Keyword, Name: {})", keyword),
                ReservedToken::Operator(op, _) => format!("(Operator, Value: '{}')", op),
            },
            Self::Number(info) => info.to_string(),
            Self::Char(c) => format!("(Char Literal, Value: {})", c),
            Self::Str(s) => format!("(String Literal, Value: {})", s),
        }
    }
}
