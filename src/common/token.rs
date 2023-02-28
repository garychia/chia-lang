use super::reserved::ReservedToken;

pub enum Token<'a> {
    Identifier(String),
    Reserved(ReservedToken<'a>),
    Number(String, String),
    Str(String),
    Char(String),
    EOF,
}
