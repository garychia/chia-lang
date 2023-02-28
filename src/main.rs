mod chia;
mod common;

const VERSION: (u32, u32, u32) = (0, 0, 1);

fn main() {
    println!(
        "Chia Compiler -- Version: {}.{}.{}",
        VERSION.0, VERSION.1, VERSION.2
    );
    if let Some(token) = chia::lang::find_reserved_token("{") {
        match &token {
            common::reserved::ReservedToken::Char(c) => println!("Reserved Char: {}", c),
            common::reserved::ReservedToken::Operator(op, _) => println!("Operator: {}", op),
            common::reserved::ReservedToken::Keyword(keyword) => println!("Keyword: {}", keyword),
        }
    }
}
