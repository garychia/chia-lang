use super::lang::find_reserved_token;
use crate::common::{
    position::{Position, PositionRange},
    token::{NumberInfo, Token},
};

pub struct LexerError {
    description: String,
    position_range: PositionRange,
}

impl ToString for LexerError {
    fn to_string(&self) -> String {
        format!(
            "{} (Position: {})",
            self.description,
            self.position_range.to_string()
        )
    }
}

pub struct Lexer<'a> {
    position: Position,
    src_code: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src_code: &'a str) -> Lexer<'a> {
        Lexer {
            position: Position::new(),
            src_code,
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

    fn read_until(
        &mut self,
        condition: fn(Option<char>, char) -> bool,
        until_end: bool,
    ) -> Option<PositionRange> {
        let start_pos = self.position.clone();
        let mut last_char: Option<char> = None;
        let mut last_pos = self.position.clone();
        let mut found = false;
        while let Some(current) = self.peek() {
            last_pos = self.position.clone();
            self.consume();
            if condition(last_char, current) {
                found = true;
                break;
            }
            last_char = Some(current);
        }
        if found || until_end {
            return Some(PositionRange {
                start: start_pos,
                end: last_pos,
            });
        }
        self.position = start_pos;
        None
    }

    fn scan_str(
        &mut self,
        start_pos: Position,
    ) -> Result<Option<(Token<'a>, PositionRange)>, LexerError> {
        let first_char = self.src_code.chars().nth(start_pos.index);
        if first_char.is_none() || first_char.unwrap() != '\"' {
            return Ok(None);
        }
        if let Some(range) = self.read_until(
            |last_char, current| {
                current == '\"' && (last_char.is_none() || last_char.unwrap() != '\\')
            },
            false,
        ) {
            return Ok(Some((
                Token::Str(&self.src_code[start_pos.index..range.end.index + 1]),
                PositionRange {
                    start: start_pos,
                    end: range.end,
                },
            )));
        }
        Err(LexerError {
            description: format!("A string literal must be closed with '\"'."),
            position_range: PositionRange {
                start: start_pos.clone(),
                end: start_pos,
            },
        })
    }

    fn scan_char(
        &mut self,
        start_pos: Position,
    ) -> Result<Option<(Token<'a>, PositionRange)>, LexerError> {
        let first_char = self.src_code.chars().nth(start_pos.index);
        if first_char.is_none() || first_char.unwrap() != '\'' {
            return Ok(None);
        }
        if let Some(range) = self.read_until(
            |last_char, current| {
                current == '\'' && (last_char.is_none() || last_char.unwrap() != '\\')
            },
            false,
        ) {
            return Ok(Some((
                Token::Char(&self.src_code[start_pos.index..range.end.index + 1]),
                PositionRange {
                    start: start_pos,
                    end: range.end,
                },
            )));
        }
        Err(LexerError {
            description: format!("A char literal must be closed with '\''."),
            position_range: PositionRange {
                start: start_pos.clone(),
                end: start_pos,
            },
        })
    }

    fn scan_number(&mut self, whole_number: bool) -> Option<PositionRange> {
        let pos_before = self.position.clone();
        let mut last_pos = self.position.clone();
        let mut end_idx = None;
        while let Some(current) = self.peek() {
            if find_reserved_token(&String::from(current)).is_some() || current.is_whitespace() {
                end_idx = Some(self.position.index);
                break;
            } else if current == '.' {
                if !whole_number {
                    end_idx = Some(self.position.index);
                }
                break;
            } else if current == 'f' {
                if whole_number {
                    break;
                }
                last_pos = self.position.clone();
                self.consume();
                match self.peek() {
                    Some(c) => {
                        if find_reserved_token(&String::from(c)).is_some() || c.is_whitespace() {
                            end_idx = Some(self.position.index);
                        }
                    }
                    _ => end_idx = Some(self.position.index),
                }
                break;
            } else if !current.is_digit(10) {
                break;
            }
            last_pos = self.position.clone();
            self.consume();
        }
        if end_idx.is_some() {
            return Some(PositionRange {
                start: pos_before,
                end: last_pos,
            });
        }
        self.position = pos_before;
        None
    }

    fn handle_result(&mut self, result: &Result<Option<(Token<'a>, PositionRange)>, LexerError>) {
        if result.is_ok() {
            return;
        }
        while let Some(c) = self.peek() {
            match c {
                ';' => break,
                _ => self.consume(),
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Option<(Token<'a>, PositionRange)>, LexerError> {
        while let Some(c) = self.peek() {
            match (c.is_whitespace() || c == '\r') && c != '\n' {
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
            self.handle_result(&result);
            match result {
                Ok(None) => {}
                _ => return result,
            }
            let result = self.scan_char(start_pos.clone());
            self.handle_result(&result);
            match result {
                Ok(None) => {}
                _ => return result,
            }
            return Ok(Some((
                Token::Reserved(
                    find_reserved_token(&self.src_code[start_pos.index..self.position.index])
                        .unwrap(),
                ),
                PositionRange {
                    start: start_pos,
                    end: last,
                },
            )));
        }
        match self.scan_number(true) {
            Some(whole_range) => match self.peek() {
                Some('.') => {
                    self.consume();
                    match self.scan_number(false) {
                        Some(fractional_range) => {
                            return Ok(Some((
                                Token::Number(NumberInfo {
                                    whole_number: &self.src_code
                                        [whole_range.start.index..whole_range.end.index + 1],
                                    fractional_part: Some(
                                        &self.src_code[fractional_range.start.index
                                            ..fractional_range.end.index + 1],
                                    ),
                                }),
                                PositionRange {
                                    start: whole_range.start,
                                    end: fractional_range.end,
                                },
                            )));
                        }
                        _ => {
                            return Err(LexerError {
                                description: format!("Number literal is invalid."),
                                position_range: PositionRange {
                                    start: start_pos,
                                    end: self.position.clone(),
                                },
                            });
                        }
                    }
                }
                _ => {
                    return Ok(Some((
                        Token::Number(NumberInfo {
                            whole_number: &self.src_code
                                [whole_range.start.index..whole_range.end.index + 1],
                            fractional_part: None,
                        }),
                        whole_range,
                    )));
                }
            },
            _ => {}
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
                return Ok(Some((
                    Token::Reserved(token),
                    PositionRange {
                        start: start_pos,
                        end: last,
                    },
                )));
            }
            if self
                .src_code
                .chars()
                .nth(start_pos.index)
                .unwrap()
                .is_digit(10)
            {
                return Err(LexerError {
                    description: format!("Invalid identifier found."),
                    position_range: PositionRange {
                        start: start_pos,
                        end: last,
                    },
                });
            }
            return Ok(Some((
                Token::Identifier(&self.src_code[start_pos.index..self.position.index]),
                PositionRange {
                    start: start_pos,
                    end: last,
                },
            )));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::position::{Position, PositionRange};

    use super::{Lexer, LexerError};

    #[test]
    fn test_lexer_error_to_string() {
        let description = "TEST";
        let range = PositionRange {
            start: Position {
                line: 1,
                column: 24,
                index: 30,
            },
            end: Position {
                line: 2,
                column: 2,
                index: 78,
            },
        };
        assert_eq!(
            LexerError {
                description: String::from(description),
                position_range: range.clone(),
            }
            .to_string(),
            format!("{} (Position: {})", description, range.to_string())
        );
    }

    #[test]
    fn test_lexer_peek_and_consume() {
        let program = "() my_func();\r\n/* TESTING */";
        let mut lexer = Lexer::new(program);
        for c in program.chars() {
            assert_eq!(lexer.peek(), Some(c));
            lexer.consume();
        }
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn test_lexer_read_until() {
        let program = "\"This is a string.\\\"abc\"";
        let start_pos = Position::new();
        let mut end_pos = Position::new();
        end_pos.index = program.len() - 1;
        let end_pos = end_pos;

        let mut lexer = Lexer::new(program);
        let condition = |last_c, c| match last_c {
            Some(last_c) => last_c != '\\' && c == '"',
            _ => c == '"',
        };
        let result = lexer.read_until(condition, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.start.index, start_pos.index);
        assert_eq!(result.end.index, start_pos.index);

        let mut lexer = Lexer::new(program);
        lexer.consume();
        let result = lexer.read_until(condition, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.start.index, start_pos.index + 1);
        assert_eq!(result.end.index, end_pos.index);

        let mut lexer = Lexer::new(program);
        let condition = |_, c| c == ';';
        assert!(lexer.read_until(condition, false).is_none());
        let mut lexer = Lexer::new(program);
        let result = lexer.read_until(condition, true);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.start.index, start_pos.index);
        assert_eq!(result.end.index, end_pos.index);
    }
}
