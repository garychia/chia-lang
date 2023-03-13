use crate::common::{reserved::ReservedToken, token::Token};

use super::ast::node::{ASTNode, FnCall, FnDef, ProgramInfo, TypeInfo, TypeVarPair, VarDef};

pub struct ParserError<'a, 'b> {
    description: String,
    token: Option<&'a Token<'b>>,
}

impl<'a, 'b> ParserError<'a, 'b> {
    pub fn new(description: String, token: Option<&'a Token<'b>>) -> ParserError<'a, 'b> {
        ParserError { description, token }
    }
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
        if self.peek().is_some() {
            self.token_idx += 1;
        }
    }

    fn generate_expect_error(
        expected_item: &str,
        token: Option<&'a Token<'b>>,
    ) -> ParserError<'a, 'b> {
        ParserError::new(format!("Expected {}.", expected_item), token)
    }

    fn parse_tuple_type(&mut self) -> Result<Box<ASTNode<'a, 'b>>, ParserError<'a, 'b>> {
        let last_idx = self.token_idx;
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(Self::generate_expect_error("left paranthesis", self.peek()));
            }
        }
        let mut inner_types = Vec::new();
        let mut expect_type = true;
        loop {
            let token = self.peek();
            match token {
                Some(Token::Reserved(ReservedToken::Char(')'))) => {
                    if expect_type && !inner_types.is_empty() {
                        self.token_idx = last_idx;
                        return Err(Self::generate_expect_error("identifier", token));
                    }
                    self.consume();
                    break;
                }
                Some(Token::Reserved(ReservedToken::Char(','))) => {
                    if inner_types.is_empty() || expect_type {
                        self.token_idx = last_idx;
                        return Err(Self::generate_expect_error("identifier", token));
                    }
                    expect_type = true;
                    self.consume();
                }
                _ => {
                    if !expect_type {
                        self.token_idx = last_idx;
                        return Err(Self::generate_expect_error("')' or ','", token));
                    }
                    match self.parse_type() {
                        Ok(node) => {
                            expect_type = false;
                            inner_types.push(node)
                        }
                        _ => {
                            self.token_idx = last_idx;
                            return Err(Self::generate_expect_error("')' or ','", token));
                        }
                    }
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

    fn parse_keyword(&mut self, keyword: &str) -> Option<&'a Token<'b>> {
        match self.peek() {
            Some(token) => match token {
                Token::Reserved(ReservedToken::Keyword(token_keyword)) => {
                    if token_keyword == &keyword {
                        self.consume();
                        return Some(token);
                    }
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_static(&mut self) -> bool {
        self.parse_keyword("static").is_some()
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

    fn parse_expr_parantheses(&mut self) -> Result<Box<ASTNode<'a, 'b>>, ParserError<'a, 'b>> {
        let last_idx = self.token_idx;
        let token = self.peek();
        match token {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                self.token_idx = last_idx;
                return Err(Self::generate_expect_error("'('", token));
            }
        }
        let inner = self.parse_expr();
        match inner {
            Ok(_) => {}
            err => {
                self.token_idx = last_idx;
                return err;
            }
        }
        let token = self.peek();
        match token {
            Some(Token::Reserved(ReservedToken::Char(')'))) => self.consume(),
            _ => {
                self.token_idx = last_idx;
                return Err(Self::generate_expect_error("')'", token));
            }
        }
        inner
    }

    fn parse_prefix_operators(&mut self) -> Vec<&'a ReservedToken<'b>> {
        let mut prefix_operators = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Reserved(reserved_token) => match reserved_token {
                    ReservedToken::Operator(_, info) => match info.is_prefix {
                        true => prefix_operators.push(*reserved_token),
                        _ => break,
                    },
                    _ => break,
                },
                _ => break,
            }
            self.consume();
        }
        prefix_operators
    }

    fn parse_tuple_expr(&mut self) -> Result<Vec<Box<ASTNode<'a, 'b>>>, ParserError<'a, 'b>> {
        let last_idx = self.token_idx;
        match self.peek() {
            Some(Token::Reserved(ReservedToken::Char('('))) => self.consume(),
            _ => {
                return Err(Self::generate_expect_error("'('", self.peek()));
            }
        }
        let mut arguments = Vec::new();
        let mut expect_expr = true;
        loop {
            let token = self.peek();
            match token {
                Some(Token::Reserved(ReservedToken::Char(')'))) => {
                    if expect_expr && !arguments.is_empty() {
                        self.token_idx = last_idx;
                        return Err(Self::generate_expect_error("expression", token));
                    }
                    self.consume();
                    break;
                }
                Some(Token::Reserved(ReservedToken::Char(','))) => {
                    if expect_expr {
                        self.token_idx = last_idx;
                        return Err(Self::generate_expect_error("expression", token));
                    }
                    self.consume();
                    expect_expr = true;
                }
                _ => match self.parse_expr() {
                    Ok(arg) => arguments.push(arg),
                    Err(err) => {
                        self.token_idx = last_idx;
                        return Err(err);
                    }
                },
            }
        }
        Ok(arguments)
    }

    fn parse_operand(&mut self) -> Result<Option<Box<ASTNode<'a, 'b>>>, ParserError> {
        let mut operand = None;
        let last_idx = self.token_idx;
        if let Some(token) = self.peek() {
            operand = match token {
                Token::Identifier(_) => {
                    self.consume();
                    match self.peek() {
                        Some(Token::Reserved(ReservedToken::Char('('))) => {
                            match self.parse_tuple_expr() {
                                Ok(args) => Some(Box::new(ASTNode::new_function_call(
                                    FnCall::new(Box::new(ASTNode::new_identifier(token)), args),
                                ))),
                                Err(err) => {
                                    self.token_idx = last_idx;
                                    return Err(err);
                                }
                            }
                        }
                        _ => Some(Box::new(ASTNode::new_identifier(token))),
                    }
                }
                Token::Number(_) => Some(Box::new(ASTNode::new_number(token))),
                Token::Str(_) => Some(Box::new(ASTNode::new_string(token))),
                Token::Char(_) => Some(Box::new(ASTNode::new_char(token))),
                _ => None,
            };
        }
        if operand.is_some() {
            self.consume();
        }
        Ok(operand)
    }

    fn parse_postfix_operators(&mut self) -> Vec<&'a ReservedToken<'b>> {
        let mut postfix_operators = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Reserved(reserved_token) => match reserved_token {
                    ReservedToken::Operator(_, info) => match info.is_postfix {
                        true => postfix_operators.push(*reserved_token),
                        _ => break,
                    },
                    _ => break,
                },
                _ => break,
            }
            self.consume();
        }
        postfix_operators
    }

    fn parse_expr(&mut self) -> Result<Box<ASTNode<'a, 'b>>, ParserError<'a, 'b>> {
        let mut operands = Vec::new();
        let mut operators: Vec<&'a ReservedToken<'b>> = Vec::new();
        let mut operand_expected = true;
        let last_idx = self.token_idx;
        while self.peek().is_some() {
            if operand_expected {
                let prefix_operators = self.parse_prefix_operators();
                let operand = self.parse_operand();
                if let Ok(Some(mut operand_node)) = operand {
                    let postfix_operators = self.parse_postfix_operators();
                    for op in postfix_operators.iter() {
                        operand_node = Box::new(ASTNode::PostfixOperation(*op, operand_node));
                    }
                    for op in prefix_operators.iter().rev() {
                        operand_node = Box::new(ASTNode::PrefixOperation(*op, operand_node));
                    }
                    operands.push(operand_node);
                } else {
                    break;
                }
            } else {
                let mut succesful = false;
                if let Some(token) = self.peek() {
                    match token {
                        Token::Reserved(reserved_token) => match reserved_token {
                            ReservedToken::Operator(_, info) => {
                                if info.is_binary || info.is_ternary {
                                    succesful = true;
                                    self.consume();
                                    while let Some(other_op_token) = operators.pop() {
                                        match other_op_token {
                                            ReservedToken::Operator(_, other_info) => {
                                                if other_info.precedence.unwrap()
                                                    <= info.precedence.unwrap()
                                                {
                                                    let operand2 = operands.pop().unwrap();
                                                    let operand1 = operands.pop().unwrap();
                                                    operands.push(Box::new(
                                                        ASTNode::BinaryOperation(
                                                            other_op_token,
                                                            operand1,
                                                            operand2,
                                                        ),
                                                    ));
                                                } else {
                                                    operators.push(other_op_token);
                                                    break;
                                                }
                                            }
                                            _ => {
                                                panic!("Parser: operators are handled incorrectly.")
                                            }
                                        }
                                    }
                                    operators.push(reserved_token);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                if !succesful {
                    break;
                }
            }
            operand_expected = !operand_expected;
        }
        if operand_expected {
            let token = self.peek();
            self.token_idx = last_idx;
            return Err(Self::generate_expect_error("operand", token));
        }
        if operands.len() != operators.len() + 1 {
            panic!("Parser: the number of operands or operators is not correct.");
        }
        while let Some(op) = operators.pop() {
            let operand2 = operands.pop().unwrap();
            let operand1 = operands.pop().unwrap();
            operands.push(Box::new(ASTNode::BinaryOperation(op, operand1, operand2)));
        }
        Ok(operands.pop().unwrap())
    }

    fn parse_type(&mut self) -> Result<Box<ASTNode<'a, 'b>>, ParserError<'a, 'b>> {
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
    ) -> Result<(Box<ASTNode<'a, 'b>>, Box<ASTNode<'a, 'b>>), ParserError<'a, 'b>> {
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

    fn parse_arg_list(&mut self) -> Result<Vec<TypeVarPair<'a, 'b>>, ParserError<'a, 'b>> {
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

    fn parse_decl(&mut self) -> Result<Box<ASTNode<'a, 'b>>, ParserError<'a, 'b>> {
        let last_idx = self.token_idx;
        let type_id_result = self.parse_type_identifier();
        match type_id_result {
            Err(err) => {
                self.token_idx = last_idx;
                Err(err)
            }
            Ok((type_found, id)) => match self.peek() {
                Some(Token::Reserved(ReservedToken::Char('('))) => match self.parse_arg_list() {
                    Ok(args) => Ok(Box::new(ASTNode::new_function(FnDef::new(
                        type_found, id, args, None,
                    )))),
                    Err(err) => {
                        self.token_idx = last_idx;
                        Err(err)
                    }
                },
                Some(Token::Reserved(ReservedToken::Operator("=", _))) => {
                    self.consume();
                    match self.parse_expr() {
                        Ok(expr) => {
                            match self.peek() {
                                Some(Token::Reserved(ReservedToken::Char(';'))) => self.consume(),
                                _ => {
                                    let token = self.peek();
                                    self.token_idx = last_idx;
                                    return Err(ParserError {
                                        description: format!("Expected ';'."),
                                        token,
                                    });
                                }
                            }
                            Ok(Box::new(ASTNode::new_variable(VarDef::new(
                                TypeVarPair::new(type_found, id),
                                Some(expr),
                            ))))
                        }
                        Err(err) => {
                            self.token_idx = last_idx;
                            Err(err)
                        }
                    }
                }
                Some(Token::Reserved(ReservedToken::Operator(";", _))) => Ok(Box::new(
                    ASTNode::new_variable(VarDef::new(TypeVarPair::new(type_found, id), None)),
                )),
                _ => {
                    self.token_idx = last_idx;
                    Err(ParserError {
                        description: format!("Expected ';', '=' or '('."),
                        token: self.peek(),
                    })
                }
            },
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode<'a, 'b>, Vec<ParserError>> {
        let mut definitions = Vec::new();
        let mut errors = Vec::new();
        while self.peek().is_some() {
            match self.parse_decl() {
                Ok(node) => definitions.push(node),
                Err(err) => {
                    errors.push(err);
                    while let Some(c) = self.peek() {
                        match c {
                            Token::Reserved(&ReservedToken::Char(';')) => break,
                            _ => self.consume(),
                        }
                    }
                    self.consume();
                }
            }
        }
        if errors.is_empty() {
            Ok(ASTNode::Program(ProgramInfo::new(
                self.program_name.clone(),
                definitions,
            )))
        } else {
            Err(errors)
        }
    }
}
