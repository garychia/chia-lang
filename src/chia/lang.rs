use crate::common::reserved::{OperatorInfo, ReservedToken};
use std::option::Option;

const CHIA_RESERVED_TOKENS: [ReservedToken; 66] = [
    ReservedToken::Char(';'),
    ReservedToken::Char('\n'),
    ReservedToken::Char('{'),
    ReservedToken::Char('}'),
    ReservedToken::Char('\''),
    ReservedToken::Char('\"'),
    ReservedToken::Char('('),
    ReservedToken::Char(')'),
    ReservedToken::Operator(
        "!",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: false,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "~",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: false,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        ".",
        OperatorInfo {
            is_unary: true,
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "[",
        OperatorInfo {
            is_unary: true,
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "]",
        OperatorInfo {
            is_unary: true,
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "++",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "--",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "==",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "+",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "-",
        OperatorInfo {
            is_unary: true,
            is_prefix: true,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "*",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "/",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "%",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "&",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "|",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "^",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        ">",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "<",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        ">=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        ">>=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "<=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "<<=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "!=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "+=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "-=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "*=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "/=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "%=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "||",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "|=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "&&",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "&=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "^=",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "<<",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        ">>",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
        },
    ),
    ReservedToken::Operator(
        "?",
        OperatorInfo {
            is_unary: false,
            is_prefix: false,
            is_postfix: false,
            is_binary: false,
            is_ternary: true,
        },
    ),
    ReservedToken::Keyword("const"),
    ReservedToken::Keyword("volatile"),
    ReservedToken::Keyword("static"),
    ReservedToken::Keyword("extern"),
    ReservedToken::Keyword("return"),
    ReservedToken::Keyword("break"),
    ReservedToken::Keyword("continue"),
    ReservedToken::Keyword("if"),
    ReservedToken::Keyword("else"),
    ReservedToken::Keyword("do"),
    ReservedToken::Keyword("while"),
    ReservedToken::Keyword("for"),
    ReservedToken::Keyword("switch"),
    ReservedToken::Keyword("case"),
    ReservedToken::Keyword("default"),
    ReservedToken::Keyword("typedef"),
    ReservedToken::Keyword("enum"),
    ReservedToken::Keyword("struct"),
    ReservedToken::Keyword("register"),
    ReservedToken::Keyword("goto"),
    ReservedToken::Keyword("sizeof"),
];

pub fn find_reserved_token(s: &str) -> Option<&ReservedToken> {
    for token in &CHIA_RESERVED_TOKENS {
        match token {
            ReservedToken::Char(c) => {
                if s.len() == 1 && s.chars().next().unwrap() == *c {
                    return Some(&token);
                }
            }
            ReservedToken::Operator(op, _) => {
                if *op == s {
                    return Some(&token);
                }
            }
            ReservedToken::Keyword(keyword) => {
                if *keyword == s {
                    return Some(&token);
                }
            }
        }
    }
    None
}
