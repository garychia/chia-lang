pub struct OperatorInfo {
    pub is_prefix: bool,
    pub is_postfix: bool,
    pub is_binary: bool,
    pub is_ternary: bool,
    pub precedence: Option<u32>,
}

pub enum ReservedToken<'a> {
    Keyword(&'a str),
    Operator(&'a str, OperatorInfo),
    Char(char),
}
