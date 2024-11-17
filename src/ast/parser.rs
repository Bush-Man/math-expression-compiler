
use std::borrow::Borrow;

use super::{lexer::{Token, TokenKind}, Ast, BinOperator, BinOperatorAssiciativity, BinOperatorKind, ExprId, StmtId};

pub struct Parser<'a> {
   pub tokens: Vec<Token>,
   pub current: usize,
   pub ast: &'a mut Ast
}
impl<'a> Parser<'a>{
    pub fn new(tokens:Vec<Token>,ast:&'a mut Ast)->Self{
        Self { 
            tokens: tokens.into_iter().filter(|token|token.kind != TokenKind::Whitespace).collect(),
            current: 0,
            ast
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
            let stmt_id = self.parse_statement();
            self.ast.item_from_stmt_id(stmt_id);
        }
    }
    fn parse_statement(&mut self)->StmtId{
        let current_token = self.current_token();
        match current_token.kind{
            TokenKind::Let =>{
                return self.parse_let_statement();
            }
            _ => {
                todo!("Parse other statements types")
            }
        }

    }
    fn parse_let_statement(&mut self)->StmtId{
        self.consume_and_verify_token(TokenKind::Let);
        let identifier = self.current_token().clone();
        self.consume_and_verify_token(TokenKind::Equals);
        
        let expr_id = self.parse_expression();
        let stmt = self.ast.save_let_statement(identifier,expr_id);

        return stmt.id;


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
            _ => self.parse_binary_expression()
          }
            }
           

    

    fn consume_and_verify_token(&mut self,token_kind:TokenKind){
        // if self.current_token().kind != token_kind{
        //     todo!("consume and verify token: Diagnostic not matching token")

        // }
        // println!("{:?}",self.current_token());
        self.current+= 1;
    }
    fn consume(&mut self)->&Token{
        let token = self.tokens[self.current].borrow();
        self.current+=1;
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