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

#[cfg(test)]
mod tests {
    use crate::common::reserved::{OperatorInfo, ReservedToken};

    use super::{NumberInfo, Token};

    #[test]
    fn test_number_info_to_string() {
        let whole_number = "120";
        assert_eq!(
            NumberInfo {
                whole_number,
                fractional_part: None
            }
            .to_string(),
            format!("(Number, whole number: {})", whole_number)
        );

        let whole_number = "7923";
        let fractional_part = "12443";
        assert_eq!(
            NumberInfo {
                whole_number,
                fractional_part: Some(fractional_part)
            }
            .to_string(),
            format!(
                "(Number, whole number: {}, fractional part: {})",
                whole_number, fractional_part
            )
        );
    }

    fn test_token_to_string() {
        let id_name = "my_id1";
        assert_eq!(
            Token::Identifier(id_name).to_string(),
            format!("(Identifier, Name: {})", id_name)
        );
        let id_name = "my_id2";
        assert_eq!(
            Token::Identifier(id_name).to_string(),
            format!("(Identifier, Name: {})", id_name)
        );
        let c = 'a';
        let reserved_token = ReservedToken::Char(c);
        assert_eq!(
            Token::Reserved(&reserved_token).to_string(),
            format!("(Reserved Char, Value: '{}')", c)
        );
        let c = '\n';
        let reserved_token = ReservedToken::Char(c);
        assert_eq!(
            Token::Reserved(&reserved_token).to_string(),
            format!("(Reserved Char, Value: '\\n')")
        );
        let keyword = "return";
        let reserved_token = ReservedToken::Keyword(keyword);
        assert_eq!(
            Token::Reserved(&reserved_token).to_string(),
            format!("(Keyword, Name: {})", keyword)
        );
        let keyword = "mut";
        let reserved_token = ReservedToken::Keyword(keyword);
        assert_eq!(
            Token::Reserved(&reserved_token).to_string(),
            format!("(Keyword, Name: {})", keyword)
        );
        let op = "++";
        let op_info = OperatorInfo {
            is_binary: false,
            is_postfix: false,
            is_unary: false,
            is_prefix: false,
            is_ternary: true,
        };
        let reserved_token = ReservedToken::Operator(op, op_info);
        assert_eq!(
            Token::Reserved(&reserved_token).to_string(),
            format!("(Operator, Value: '{}')", op)
        );
        let whole_number = "3";
        let fractional_part = "1415";
        let num_info = NumberInfo {
            whole_number,
            fractional_part: Some(fractional_part),
        };
        assert_eq!(
            Token::Number(num_info).to_string(),
            format!(
                "(Number, whole number: {}, fractional part: {})",
                whole_number, fractional_part
            )
        );
        let char_literal = "'a'";
        assert_eq!(
            Token::Char(char_literal).to_string(),
            format!("(Char Literal, Value: {})", char_literal)
        );
        let char_literal = "'\\n'";
        assert_eq!(
            Token::Char(char_literal).to_string(),
            format!("(Char Literal, Value: {})", char_literal)
        );
        let str_literal = "\"This is a test.\"";
        assert_eq!(
            Token::Str(str_literal).to_string(),
            format!("(String Literal, Value: {})", str_literal)
        );
    }
}
