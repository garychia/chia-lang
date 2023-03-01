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

    fn read_until(&mut self, f: fn(Option<char>, char) -> bool) -> Option<PositionRange> {
        let pos_before = self.position.clone();
        let mut last_char: Option<char> = None;
        let mut last_pos = self.position.clone();
        while let Some(current) = self.peek() {
            last_pos = self.position.clone();
            self.consume();
            if f(last_char, current) {
                break;
            }
            last_char = Some(current);
        }
        if last_char.is_some() {
            return Some(PositionRange {
                start: pos_before,
                end: last_pos,
            });
        }
        self.position = pos_before;
        None
    }

    fn scan_str(&mut self, start_pos: Position) -> Option<(Token<'a>, PositionRange)> {
        let first_char = self.src_code.chars().nth(start_pos.index);
        match first_char {
            Some(c) => {
                if c != '\"' {
                    return None;
                }
            }
            _ => return None,
        }
        if let Some(range) = self.read_until(|last_char, current| {
            current == '\"' && (last_char.is_none() || last_char.unwrap() != '\\')
        }) {
            return Some((
                Token::Str(&self.src_code[start_pos.index..range.end.index + 1]),
                PositionRange {
                    start: start_pos,
                    end: range.end,
                },
            ));
        }
        None
    }

    fn scan_char(&mut self, start_pos: Position) -> Option<(Token<'a>, PositionRange)> {
        let first_char = self.src_code.chars().nth(start_pos.index);
        if first_char.is_none() || first_char.unwrap() != '\'' {
            return None;
        }
        if let Some(range) = self.read_until(|last_char, current| {
            current == '\'' && (last_char.is_none() || last_char.unwrap() != '\\')
        }) {
            return Some((
                Token::Char(&self.src_code[start_pos.index..range.end.index + 1]),
                PositionRange {
                    start: start_pos,
                    end: range.end,
                },
            ));
        }
        None
    }

    pub fn next_token(&mut self) -> Option<(Token<'a>, PositionRange)> {
        while let Some(c) = self.peek() {
            match c.is_whitespace() || c == '\r' {
                true => self.consume(),
                _ => break,
            }
        }
        let start_pos = self.position.clone();
        let mut last_pos: Option<Position> = None;
        while self.peek().is_some() {
            if find_reserved_token(&self.src_code[start_pos.index..self.position.index + 1])
                .is_none()
            {
                break;
            } else {
                last_pos = Some(self.position.clone());
                self.consume();
            }
        }
        if let Some(last) = last_pos {
            let result = self.scan_str(start_pos.clone());
            if result.is_some() {
                return result;
            }
            let result = self.scan_char(start_pos.clone());
            if result.is_some() {
                return result;
            }
            return Some((
                Token::Reserved(
                    find_reserved_token(&self.src_code[start_pos.index..self.position.index])
                        .unwrap(),
                ),
                PositionRange {
                    start: start_pos,
                    end: last,
                },
            ));
        }
        while let Some(c) = self.peek() {
            if find_reserved_token(&String::from(c)).is_some() || c.is_whitespace() {
                break;
            } else if c == '\r' {
                self.consume();
            } else {
                last_pos = Some(self.position.clone());
                self.consume();
            }
        }
        if let Some(last) = last_pos {
            if let Some(token) =
                find_reserved_token(&self.src_code[start_pos.index..self.position.index])
            {
                return Some((
                    Token::Reserved(token),
                    PositionRange {
                        start: start_pos,
                        end: last,
                    },
                ));
            }
            return Some((
                Token::Identifier(&self.src_code[start_pos.index..self.position.index]),
                PositionRange {
                    start: start_pos,
                    end: last,
                },
            ));
        }
        None
    }
}
