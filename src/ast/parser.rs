
use std::{borrow::Borrow, vec};

use crate::ast::lib::Id;

use super::{global_scope::GlobalScope, lexer::{Token, TokenKind}, Ast, BinOperator, BinOperatorAssiciativity, BinOperatorKind, Body, ExprId, Function, FunctionId, Item, Parameter, Statement, StatementKind, StmtId};

pub struct Parser<'a> {
   pub tokens: Vec<Token>,
   pub current: usize,
   pub ast: &'a mut Ast,
   pub scope: &'a mut GlobalScope
}
impl<'a> Parser<'a>{
    pub fn new(tokens:Vec<Token>,ast:&'a mut Ast,scope:&'a mut GlobalScope)->Self{
        Self { 
            tokens: tokens.into_iter().filter(|token|token.kind != TokenKind::Whitespace).collect(),
            current: 0,
            ast,
            scope
        }
    }

    pub fn parse(&mut self){
        self.parse_items();


    }
    fn is_at_end(&mut self)->bool{
        self.current_token().kind == TokenKind::Eof
    }

    fn parse_items(&mut self){
        while matches!(self.is_at_end(),false){
             self.parse_statement();
            
        }
    }
    fn parse_statement(&mut self){
        let current_token = self.current_token();
        match current_token.kind{
            TokenKind::Let =>{
                let stmt_id = self.parse_let_statement();
                self.ast.item_from_stmt_id(stmt_id);
            },
            TokenKind::Function =>{
                let function_id = self.parse_function().expect("function id in parse statemtnt");
                self.ast.item_from_function_id(function_id);

            },          
            _ => {
                let expr_id = self.parse_expression();
                self.ast.stmt_from_stmt_kind(super::StatementKind::Expression(expr_id));
                
            }
        }

    }
    fn parse_let_statement(&mut self)->StmtId{
        let token = self.consume_and_verify_token(TokenKind::Let);
        let identifier = self.current_token().clone();
        
        self.consume();
        self.consume_and_verify_token(TokenKind::Equals);
        let value = self.current_token().span.literal.clone();
        
        
        self.scope.add_global_variable(identifier.span.literal.clone(), value);
        let expr_id = self.parse_expression();
        let stmt = self.ast.save_let_statement(identifier,expr_id);

        return stmt.id;


    }
    fn parse_function(&mut self)->Option<FunctionId>{
        let current_token_keyword = self.consume_and_verify_token(TokenKind::Function);
        let function_name_token= self.consume_and_verify_token(TokenKind::Identifier);
        let current_token =self.current_token();
        let  open_paren;
        let  close_paren; 
        let mut parameters_vec=Vec::new();
        let open_brace;
        let mut body_vec:Vec<StmtId> =  Vec::new();
        let close_brace:Token;
        match current_token.kind{
            TokenKind::OpenParen=>{
                open_paren = self.consume_and_verify_token(TokenKind::OpenParen);
                if let Some(parameters )= self.parse_function_parameters(){
                 parameters_vec=parameters;
                }
                close_paren = self.consume_and_verify_token(TokenKind::CloseParen);
                open_brace = self.consume_and_verify_token(TokenKind::OpenBrace);
                if let Some(body_stmts) = self.parse_function_body(){
                    body_vec = body_stmts;
                }
                close_brace = self.consume_and_verify_token(TokenKind::CloseBrace);
                let function_body = Body::new(open_brace,body_vec,close_brace);
               

                let func = self.ast.save_function(function_name_token.span.literal, open_paren, close_paren, parameters_vec);

                
                return Some(func);            
            }
            _ =>{
               None
            }
        }
    



    }
    fn parse_function_parameters(&mut self)->Option<Vec<Parameter>>{
        let mut current_token = self.consume();
        match current_token.kind {
            TokenKind::CloseParen=>None,
            TokenKind::Identifier=>{
              let mut identifiers:Vec<Parameter> = Vec::new();
                while current_token.kind !=TokenKind::CloseParen{
                    if current_token.kind == TokenKind::Comma{
                        self.consume_and_verify_token(TokenKind::Comma);
                        current_token =self.current_token();
                    }
                    let parameter = Parameter::new(current_token.clone());
                    identifiers.push(parameter);

                }              
               return  Some(identifiers);
               
                
               
            },
            _ => None
            
        }
        

    }
    fn parse_function_body(&mut self)->Option<Vec<StmtId>>{
     todo!("implement parsing function body")

    }
    fn parse_expression(&mut self)->ExprId{
       return self.parse_binary_expression();
    }

    fn parse_binary_expression(&mut self)->ExprId{
        let left =  self.parse_primary();
        self.parse_binary_expression_recursive(left,0)


    }
    // 2 + 3 / 5 * 6
    fn parse_binary_expression_recursive(&mut self,mut left:ExprId, precedence:u8)->ExprId{
        while let Some(mut operator) =self.parse_binary_operator(){
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence{
                break;
            }
            self.consume();
            let mut right = self.parse_primary();
            while let Some(mut inner_operator) = self.parse_binary_operator(){
                let greater_precedence = inner_operator.precedence() > operator_precedence;
                let equal_precedence = inner_operator.precedence() == operator_precedence;
                if !greater_precedence && !(equal_precedence && inner_operator.assicativity() == BinOperatorAssiciativity::Right){
                    break;
                }

                 right = self.parse_binary_expression_recursive(
                    right,
                     std::cmp::max(operator_precedence,inner_operator.precedence())
                    );
            }
            left = self.ast.save_binary_expression(operator,left,right).id
        }
        left
    }

    fn parse_binary_operator(&mut self)->Option<BinOperator>{
        let token = self.current_token().clone();
        let kind = match token.kind{
            TokenKind::Plus => Some(BinOperatorKind::Plus),
            TokenKind::Minus => Some(BinOperatorKind::Minus),
            TokenKind::Asterisk => Some(BinOperatorKind::Multiply),
            TokenKind::Slash => Some(BinOperatorKind::Divide),
            _ =>None
        };

        return kind.map(|kind|BinOperator::new(kind, token));

    }
    fn parse_primary(&mut self)->ExprId{
        let current_token = self.consume().clone();
        
          return match current_token.kind{
            TokenKind::Number(number)=>self.ast.save_number_expression(current_token, number).id,
            TokenKind::OpenParen => {
                let open_paren =current_token;
                let expr_id = self.parse_expression();
                let close_paren = self.consume().clone();
                let expr = self.ast.save_parenthesized_expression(expr_id, open_paren, close_paren);
                expr.id
                   
                 
                   
            },
            TokenKind::CloseParen => {
                let expr_id = self.parse_expression();
                
                 expr_id
                   
                 
                   
            },
            
            
            _ => self.parse_binary_expression()
          }
            }
           

    

    fn consume_and_verify_token(&mut self,token_kind:TokenKind)->Token{
        let current_token = self.current_token().clone();
        if current_token.kind != token_kind{
        println!("Found Token:< {:?} >, Expected Token< {:?} >",current_token.kind,token_kind);

           

        }
        if current_token.kind != TokenKind::Eof{
        self.current+=1;

        }
        current_token
    }
    fn consume(&mut self)->&Token{
        let token = self.tokens[self.current].borrow();
        if token.kind != TokenKind::Eof{
        self.current+=1;
        }
        return &token;

    }
    fn peek(&mut self,offset:usize)->&Token{
        let index =self.current + offset;
        &self.tokens[index]
    }

    fn current_token(&mut self)->&Token{
        self.peek(0)
    }

}