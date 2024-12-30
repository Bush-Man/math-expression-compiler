use std::{borrow::Borrow, vec};

use crate::ast::lib::Id;

use super::{
    global_scope::GlobalScope, lexer::{Token, TokenKind}, Ast, BinOperator, BinOperatorAssiciativity, BinOperatorKind, Body, ExprId, Function, FunctionId, Item, Parameter, Statement, StatementKind, StmtId
};

pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub ast: &'a mut Ast,
    pub scope: &'a mut GlobalScope,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, ast: &'a mut Ast, scope: &'a mut GlobalScope) -> Self {
        Self { 
            tokens: tokens.into_iter().filter(|token| token.kind != TokenKind::Whitespace).collect(),
            current: 0,
            ast,
            scope,
        }
    }

    pub fn parse(&mut self) {
        self.parse_items();
    }

    fn is_at_end(&self) -> bool {
        self.current_token().kind == TokenKind::Eof
    }

    fn parse_items(&mut self) {
        while !self.is_at_end() {
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self)->Option<StmtId> {
        let current_token = self.current_token();
        match current_token.kind {
            TokenKind::Let => {
                let stmt_id = self.parse_let_statement();
                self.ast.item_from_stmt_id(stmt_id);
                Some(stmt_id)
            }
            TokenKind::Function => {
                if let Some(function_id) = self.parse_function() {
                    self.ast.item_from_function_id(function_id);
                    None
                } else {
                    panic!("Failed to parse function statement");
                }
            }
            _ => {
                let expr_id = self.parse_expression();
                let stmt_id = self.ast.stmt_from_stmt_kind(StatementKind::Expression(expr_id)).id;
                Some(stmt_id)
            }
        }
    }

    fn parse_let_statement(&mut self) -> StmtId {
        self.consume_and_verify_token(TokenKind::Let);
        let identifier = self.consume_and_verify_token(TokenKind::Identifier);
        self.consume_and_verify_token(TokenKind::Equals);
        let expr_id = self.parse_expression();
         let value = self.current_token().span.literal.clone();
        self.scope.add_global_variable(identifier.span.literal.clone(), value);
        let stmt = self.ast.save_let_statement(identifier, expr_id);
        stmt.id
    }

    fn parse_function(&mut self) -> Option<FunctionId> {
        self.consume_and_verify_token(TokenKind::Function);
        let function_name_token = self.consume_and_verify_token(TokenKind::Identifier);
        let open_paren = self.consume_and_verify_token(TokenKind::OpenParen);

        let parameters_vec = self.parse_function_parameters().unwrap_or_default();
        let close_paren = self.consume_and_verify_token(TokenKind::CloseParen);
        let open_brace = self.consume_and_verify_token(TokenKind::OpenBrace);

        let body_vec = self.parse_function_body().unwrap_or_default();
        let close_brace = self.consume_and_verify_token(TokenKind::CloseBrace);

        let function_body = Body::new(open_brace,body_vec,close_brace);
        print!("{:?}",function_body);

        let func = self.ast.save_function(function_name_token.span.literal, open_paren,close_paren,parameters_vec);
        Some(func)
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::CloseParen {
            if self.current_token().kind == TokenKind::Comma {
                self.consume_and_verify_token(TokenKind::Comma);
            }
            let parameter_token = self.consume_and_verify_token(TokenKind::Identifier);
            parameters.push(Parameter::new(parameter_token));
        }
        Some(parameters)
    }

    fn parse_function_body(&mut self) -> Option<Vec<StmtId>> {
        let mut body_vec:Vec<StmtId> = Vec::new();
        while self.current_token().kind != TokenKind::CloseBrace {
            let stmt_id = self.parse_statement().unwrap();
            body_vec.push(stmt_id);
        }
        Some(body_vec)
    }

    fn parse_expression(&mut self) -> ExprId {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> ExprId {
        let left = self.parse_primary();
        self.parse_binary_expression_recursive(left, 0)
    }

    fn parse_binary_expression_recursive(&mut self, mut left: ExprId, precedence: u8) -> ExprId {
        while let Some(mut operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let mut right = self.parse_primary();
            while let Some(mut inner_operator) = self.parse_binary_operator() {
                let higher_precedence = inner_operator.precedence() > operator_precedence;
                let equal_precedence = inner_operator.precedence() == operator_precedence;
                if !higher_precedence && !(equal_precedence && inner_operator.assicativity() == BinOperatorAssiciativity::Right) {
                    break;
                }
                right = self.parse_binary_expression_recursive(right, inner_operator.precedence());
            }
            left = self.ast.save_binary_expression(operator, left, right).id;
        }
        left
    }

    fn parse_binary_operator(&mut self) -> Option<BinOperator> {
        let token = self.current_token().clone();
        let kind = match token.kind {
            TokenKind::Plus => Some(BinOperatorKind::Plus),
            TokenKind::Minus => Some(BinOperatorKind::Minus),
            TokenKind::Asterisk => Some(BinOperatorKind::Multiply),
            TokenKind::Slash => Some(BinOperatorKind::Divide),
            _ => None,
        };
        kind.map(|kind| BinOperator::new(kind, token))
    }

    fn parse_primary(&mut self) -> ExprId {
        let current_token = self.consume().clone();
        match current_token.kind {
            TokenKind::Number(number) => self.ast.save_number_expression(current_token, number).id,
            TokenKind::OpenParen => {
                let expr_id = self.parse_expression();
                self.consume_and_verify_token(TokenKind::CloseParen);
                expr_id
            },
            _ => panic!("Unexpected token:{:?} : {:?}", current_token.span.literal,current_token.kind),
        }
    }

    fn consume_and_verify_token(&mut self, token_kind: TokenKind) -> Token {
        let current_token = self.current_token().clone();
        if current_token.kind != token_kind {
            panic!("Found {:?}, expected {:?}", current_token.kind, token_kind);
        }
        self.consume();
        current_token
    }

    fn consume(&mut self) -> &Token {
        let token = self.tokens[self.current].borrow();
        if token.kind != TokenKind::Eof {
            self.current += 1;
        }
        token
    }

    fn peek(&self, offset: usize) -> &Token {
        let index = self.current + offset;
        &self.tokens[index]
    }

    fn current_token(&self) -> &Token {
        self.peek(0)
    }
}
