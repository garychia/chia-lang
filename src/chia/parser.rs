use crate::common::{reserved::ReservedToken, token::Token};

use super::ast::node::{ASTNode, FnDef, ProgramInfo, TypeInfo, TypeVarPair, VarDef, FnCall};

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
        let last_idx = self.token_idx;
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(ParserError {
                    description: format!("Expected a left parenthesis."),
                    token: self.peek(),
                })
            }
        }
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
                }
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

    fn parse_type_qualifiers(&mut self) -> (bool, bool) {
        let mut is_mut = false;
        let mut is_volatile = false;
        while let Some(token) = self.peek() {
            match token {
                Token::Reserved(ReservedToken::Keyword("mut")) => is_mut = true,
                Token::Reserved(ReservedToken::Keyword("volatile")) => is_volatile = true,
                _ => break,
            }
            self.consume();
        }
        (is_mut, is_volatile)
    }

    fn parse_static(&mut self) -> bool {
        let mut is_static = false;
        while let Some(token) = self.peek() {
            match token {
                Token::Reserved(ReservedToken::Keyword("static")) => is_static = true,
                _ => break,
            }
            self.consume();
        }
        is_static
    }

    fn peek_identifier(&mut self) -> Option<&'a Token<'b>> {
        match self.peek() {
            Some(token) => match token {
                Token::Identifier(_) => Some(token),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_expr_parantheses(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(ParserError {
                    description: format!("Expected '('."),
                    token: self.peek(),
                })
            }
        }
        let inner = self.parse_expr();
        match inner {
            Ok(_) => {}
            err => return err,
        }
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char(')'))) => self.consume(),
            _ => {
                return Err(ParserError {
                    description: format!("Expected ')'."),
                    token: self.peek(),
                })
            }
        }
        inner
    }

    fn parse_expr(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        let mut operands = Vec::new();
        let mut operators = Vec::new();
        let mut operand_expected = true;
        let idx_before = self.token_idx;
        while self.peek().is_some() {
            if operand_expected {
                let mut prefix_operators = Vec::new();
                while let Some(token) = self.peek() {
                    match token {
                        Token::Reserved(reserved_token) => {
                            match reserved_token { 
                                ReservedToken::Operator(op, info) => {
                                    match info.is_prefix {
                                        true => prefix_operators.push(reserved_token),
                                        _ => break,
                                    }
                                }
                                _ => break,
                            }
                        }
                        _ => break,
                    }
                    self.consume();
                }

                let mut operand = None;

                if let Some(token) = self.peek() {
                    operand = match token {
                        Token::Identifier(_) => Some(ASTNode::new_identifier(token)),
                        Token::Number(_) => Some(ASTNode::new_number(token)),
                        Token::Str(_) => Some(ASTNode::String(token)),
                        _ => {
                            self.token_idx = idx_before;
                            return Err(ParserError { description: format!("Expected an identifier, number or literal."), token: Some(token)});
                        }
                    }
                }

                let mut postfix_operators = Vec::new();
                while let Some(token) = self.peek() {
                    match token {
                        Token::Reserved(reserved_token) => {
                            match reserved_token {
                                ReservedToken::Operator(op, info) => {
                                    match info.is_postfix {
                                        true => postfix_operators.push(reserved_token),
                                        _ => break,
                                    }
                                }
                                _ => break,
                            }
                        }
                        _ => break,
                    }
                    self.consume();
                }
                let mut operand_node = Box::new(operand.unwrap());
                for op in postfix_operators.iter() {
                    operand_node = Box::new(ASTNode::PostfixOperation(operand_node));
                }
                for op in prefix_operators.iter().rev() {
                    operand_node = Box::new(ASTNode::PrefixOperation(operand_node));
                }
                operands.push(operand_node);
            }
            else {
                let token = self.peek();
                let mut succesful = false;
                if let Some(token) = self.peek() {
                    match token {
                        Token::Reserved(reserved_token) => {
                            match reserved_token {
                                ReservedToken::Operator(_, info) => {
                                    if info.is_binary {
                                        succesful = true;
                                        self.consume();
                                        operators.push(reserved_token);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if !succesful {
                    self.token_idx = idx_before;
                    return Err(ParserError { description: format!("Expected an operator."), token: token });
                }
            }
            self.consume();
            operand_expected = !operand_expected;
        }
        while let Some(op) = operators.pop() {
            let operand1 = operands.pop().unwrap();
            let operand2 = operands.pop().unwrap();
            operands.push(Box::new(ASTNode::BinaryOperation(op, operand1, operand2)));
        }
        Ok(operands.pop().unwrap())
    }

    fn parse_type(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        let is_static = self.parse_static();
        let (is_mut, is_volatile) = self.parse_type_qualifiers();
        let mut base_type = None;
        if let Some(identifier) = self.peek_identifier() {
            self.consume();
            base_type = Some(Box::new(ASTNode::Type(TypeInfo::new(
                is_static,
                is_mut,
                is_volatile,
                false,
                Box::new(ASTNode::Identifier(identifier)),
            ))));
        } else {
            match self.peek() {
                Some(Token::Reserved(ReservedToken::Char('('))) => match self.parse_tuple_type() {
                    Ok(node) => base_type = Some(node),
                    Err(err) => return Err(err),
                },
                _ => {}
            }
        }
        match base_type {
            None => Err(ParserError {
                description: format!("Expected an identifier or tuple."),
                token: self.peek(),
            }),
            Some(_) => {
                while let Some(token) = self.peek() {
                    match token {
                        Token::Reserved(ReservedToken::Operator("*", _)) => {
                            self.consume();
                            let (is_mut, is_volatile) = self.parse_type_qualifiers();
                            base_type = match base_type {
                                None => {
                                    return Err(ParserError {
                                        description: format!("Invalid pointer notation"),
                                        token: Some(token),
                                    })
                                }
                                Some(base) => Some(Box::new(ASTNode::new_type(TypeInfo::new(
                                    false,
                                    is_mut,
                                    is_volatile,
                                    true,
                                    base,
                                )))),
                            };
                        }
                        _ => break,
                    }
                }
                match base_type {
                    Some(base) => Ok(base),
                    _ => Err(ParserError {
                        description: format!("Serious internal error occurred when parsing types."),
                        token: None,
                    }),
                }
            }
        }
    }

    fn parse_type_identifier(
        &mut self,
    ) -> Result<(Box<ASTNode<'a>>, Box<ASTNode<'a>>), ParserError<'a, 'b>> {
        let type_result = self.parse_type();
        match type_result {
            Err(err) => Err(err),
            Ok(type_found) => {
                let identifier = self.peek_identifier();
                match identifier {
                    None => Err(ParserError {
                        description: format!("Expected an identifier."),
                        token: self.peek(),
                    }),
                    Some(id) => {
                        self.consume();
                        Ok((type_found, Box::new(ASTNode::new_identifier(id))))
                    }
                }
            }
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<TypeVarPair<'a>>, ParserError<'a, 'b>> {
        let idx_before = self.token_idx;
        let mut args = Vec::new();
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(ParserError {
                    description: format!("Expected '('."),
                    token: self.peek(),
                })
            }
        }
        while let Some(token) = self.peek() {
            match token {
                Token::Reserved(ReservedToken::Char(')')) => {
                    self.consume();
                    break;
                }
                _ => {}
            }
            match self.parse_type_identifier() {
                Ok((type_found, id)) => args.push(TypeVarPair::new(type_found, id)),
                Err(err) => {
                    self.token_idx = idx_before;
                    return Err(err);
                }
            }
            match self.peek() {
                Some(Token::Reserved(ReservedToken::Char(','))) => self.consume(),
                Some(Token::Reserved(ReservedToken::Char(')'))) => {}
                token => {
                    self.token_idx = idx_before;
                    return Err(ParserError {
                        description: format!("Expected ',' or ')'."),
                        token,
                    });
                }
            }
        }
        Ok(args)
    }

    fn parse_decl(&mut self) -> Result<Box<ASTNode<'a>>, ParserError<'a, 'b>> {
        let type_id_result = self.parse_type_identifier();
        match type_id_result {
            Err(err) => Err(err),
            Ok((type_found, id)) => match self.peek() {
                Some(Token::Reserved(ReservedToken::Char('('))) => match self.parse_arg_list() {
                    Ok(args) => Ok(Box::new(ASTNode::new_function(FnDef::new(
                        type_found, id, args, None,
                    )))),
                    Err(err) => Err(err),
                },
                Some(Token::Reserved(ReservedToken::Operator("=", _))) => Err(ParserError {
                    description: format!("Not implemented"),
                    token: None,
                }),
                Some(Token::Reserved(ReservedToken::Operator(";", _))) => Ok(Box::new(
                    ASTNode::new_variable(VarDef::new(TypeVarPair::new(type_found, id), None)),
                )),
                _ => Err(ParserError {
                    description: format!("Expected ';', '=' or '('."),
                    token: self.peek(),
                }),
            },
        }
    }

    pub fn parse(&mut self) -> ASTNode<'a> {
        let mut definitions = Vec::new();
        ASTNode::Program(ProgramInfo::new(self.program_name.clone(), definitions))
    }
}
