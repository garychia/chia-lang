mod chia;
mod lexer;

fn main() {
    println!("Chia Compiler -- Version: 0.0.1");
    if let Some(token) = chia::lang::find_reserved_token("{") {
        match &token {
            lexer::reserved::ReservedToken::Char(c) => println!("Reserved Char: {}", c),
            &lexer::reserved::ReservedToken::Operator(op, _) => println!("Operator: {}", op),
            lexer::reserved::ReservedToken::Keyword(keyword) => println!("Keyword: {}", keyword),
        }
    }
}
