use crate::common::reserved::{OperatorInfo, ReservedToken};
use std::option::Option;

const MULTIPLICATION_PRECEDENCE: Option<u32> = Some(10);
const ADDITION_PRECEDENCE: Option<u32> = Some(15);
const BIT_SHIFT_PRECEDENCE: Option<u32> = Some(20);
const RELATIONAL_UNEQUAL_PRECEDENCE: Option<u32> = Some(25);
const RELATIONAL_EQUAL_PRECEDENCE: Option<u32> = Some(30);
const BITWISE_AND_PRECEDENCE: Option<u32> = Some(35);
const BITWISE_XOR_PRECEDENCE: Option<u32> = Some(40);
const BITWISE_OR_PRECEDENCE: Option<u32> = Some(45);
const LOGICAL_AND_PRECEDENCE: Option<u32> = Some(50);
const LOGICAL_OR_PRECEDENCE: Option<u32> = Some(55);
const TERNARY_PRECEDENCE: Option<u32> = Some(60);
const ASSIGNMENT_PRECEDENCE: Option<u32> = Some(65);

const CHIA_RESERVED_TOKENS: [ReservedToken; 70] = [
    ReservedToken::Char(';'),
    ReservedToken::Char(':'),
    ReservedToken::Char(','),
    ReservedToken::Char('\n'),
    ReservedToken::Char('\r'),
    ReservedToken::Char('{'),
    ReservedToken::Char('}'),
    ReservedToken::Char('\''),
    ReservedToken::Char('\"'),
    ReservedToken::Char('('),
    ReservedToken::Char(')'),
    ReservedToken::Operator(
        "!",
        OperatorInfo {
            is_prefix: true,
            is_postfix: false,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "~",
        OperatorInfo {
            is_prefix: true,
            is_postfix: false,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        ".",
        OperatorInfo {
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "::",
        OperatorInfo {
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "[",
        OperatorInfo {
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "]",
        OperatorInfo {
            is_prefix: false,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "++",
        OperatorInfo {
            is_prefix: true,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "--",
        OperatorInfo {
            is_prefix: true,
            is_postfix: true,
            is_binary: false,
            is_ternary: false,
            precedence: None,
        },
    ),
    ReservedToken::Operator(
        "=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "==",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_EQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "+",
        OperatorInfo {
            is_prefix: true,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ADDITION_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "-",
        OperatorInfo {
            is_prefix: true,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ADDITION_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "*",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: MULTIPLICATION_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "/",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: MULTIPLICATION_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "%",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: MULTIPLICATION_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "&",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: BITWISE_AND_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "|",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: BITWISE_OR_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "^",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: BITWISE_XOR_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        ">",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_UNEQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "<",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_UNEQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        ">=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_UNEQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        ">>=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "<=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_UNEQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "<<=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "!=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: RELATIONAL_EQUAL_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "+=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "-=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "*=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "/=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "%=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "||",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: LOGICAL_OR_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "|=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "&&",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: LOGICAL_AND_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "&=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "^=",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: ASSIGNMENT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "<<",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: BIT_SHIFT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        ">>",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: true,
            is_ternary: false,
            precedence: BIT_SHIFT_PRECEDENCE,
        },
    ),
    ReservedToken::Operator(
        "?",
        OperatorInfo {
            is_prefix: false,
            is_postfix: false,
            is_binary: false,
            is_ternary: true,
            precedence: TERNARY_PRECEDENCE,
        },
    ),
    ReservedToken::Keyword("mut"),
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

pub fn find_reserved_token<'a>(s: &str) -> Option<&'a ReservedToken<'a>> {
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
