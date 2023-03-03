use crate::common::{reserved::ReservedToken, token::Token};

use super::ast::node::{ASTNode, ProgramInfo};

pub struct ParserError<'a, 'b> {
    description: String,
    token: Option<&'a Token<'b>>,
}

pub struct Parser<'a, 'b> {
    program_name: String,
    token_idx: usize,
    tokens: Vec<&'a Token<'b>>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(program_name: String, tokens: Vec<&'a Token<'b>>) -> Parser<'a, 'b> {
        Parser {
            program_name,
            token_idx: 0,
            tokens,
        }
    }

    fn peek(&self) -> Option<&'a Token<'b>> {
        if self.token_idx < self.tokens.len() {
            return Some(self.tokens[self.token_idx]);
        }
        None
    }

    fn consume(&mut self) {
        self.token_idx += 1;
    }

    fn parse_tuple_type(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(ParserError {
                    description: format!("Expected a left parenthesis."),
                    token: self.peek(),
                })
            }
        }
        let last_idx = self.token_idx;
        let mut inner_types = Vec::new();
        loop {
            let mut reach_end = false;
            let token = self.peek();
            match token {
                Some(Token::Reserved(ReservedToken::Char(')'))) => reach_end = true,
                Some(Token::Reserved(ReservedToken::Char(','))) => {
                    if inner_types.is_empty() {
                        self.token_idx = last_idx;
                        return Err(ParserError {
                            description: format!("Expected an identifier."),
                            token,
                        });
                    }
                }
                _ => {
                    self.token_idx = last_idx;
                    return Err(ParserError {
                        description: format!("Expected ')' or ','."),
                        token,
                    });
                },
            }
            self.consume();
            if reach_end {
                break;
            }
            match self.parse_type() {
                Ok(node) => inner_types.push(node),
                Err(err) => {
                    self.token_idx = last_idx;
                    return Err(err);
                }
            }
        }
        Ok(Box::new(ASTNode::Tuple(inner_types)))
    }

    fn parse_type(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        match self.peek() {
            Some(token) => match token {
                Token::Identifier(_) => {
                    self.consume();
                    Ok(Box::new(ASTNode::Identifier(token)))
                }
                Token::Reserved(ReservedToken::Char('(')) => match self.parse_tuple_type() {
                    Ok(node) => Ok(node),
                    Err(err) => Err(err),
                },
                _ => Err(ParserError {
                    description: format!("Expected an identifier or tuple."),
                    token: self.peek(),
                }),
            },
            _ => Err(ParserError {
                description: format!("Expected an identifier or tuple."),
                token: None,
            }),
        }
    }

    fn parse_var_def(&mut self) -> Result<Option<Box<ASTNode<'a>>>, ParserError<'a, 'b>> {
        Ok(None)
    }

    pub fn parse(&mut self) -> ASTNode<'a> {
        let mut definitions = Vec::new();
        loop {
            match self.parse_var_def() {
                Ok(None) => break,
                Ok(Some(node)) => definitions.push(node),
                _ => break,
            }
        }
        ASTNode::Program(ProgramInfo::new(self.program_name.clone(), definitions))
    }
}
