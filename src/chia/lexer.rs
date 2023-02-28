use super::lang::find_reserved_token;
use crate::common::{
    position::{Position, PositionRange},
    token::Token,
};

pub struct Lexer<'a> {
    position: Position,
    src_code: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src_code: &'a str) -> Lexer<'a> {
        Lexer {
            position: Position::new(),
            src_code: src_code,
        }
    }

    fn peek(&self) -> Option<char> {
        self.src_code.chars().nth(self.position.index)
    }

    fn consume(&mut self) {
        if let Some(c) = self.peek() {
            match c {
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 0;
                }
                _ => {}
            }
            self.position.column += 1;
            self.position.index += 1;
        }
    }

    pub fn next_token(&mut self) -> Option<(Token<'a>, PositionRange)> {
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '\r' {
                self.consume();
            } else {
                break;
            }
        }
        let mut start_pos = self.position.clone();
        let mut end_pos = self.position.clone();
        let mut name = String::new();
        while let Some(c) = self.peek() {
            name.push(c);
            if find_reserved_token(&name).is_none() {
                name.pop();
                break;
            } else {
                end_pos = self.position.clone();
                self.consume();
            }
        }
        if let Some(token) = find_reserved_token(&name) {
            return Some((
                Token::Reserved(token),
                PositionRange {
                    start: start_pos,
                    end: end_pos,
                },
            ));
        }
        while let Some(c) = self.peek() {
            if find_reserved_token(&String::from(c)).is_some() || c.is_whitespace() {
                break;
            } else if c == '\r' {
                self.consume();
            } else {
                name.push(c);
                end_pos = self.position.clone();
                self.consume();
            }
        }
        if !name.is_empty() {
            return Some((
                Token::Identifier(name),
                PositionRange {
                    start: start_pos,
                    end: end_pos,
                },
            ));
        }
        None
    }
}
