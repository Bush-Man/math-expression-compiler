use core::panic;

use super::{visitor::Visitor, BinOperatorKind};

 pub struct ExpressionEvaluator{
    pub result:Option<i64>,
    pub value:Option<i64>
}


impl ExpressionEvaluator{
    pub fn new()->Self{
        Self { result: None ,value:None}
    }
}

impl Visitor for ExpressionEvaluator{
    fn visit_item(&mut self,ast:&super::Ast,item_id:super::ItemId){
        self.do_visit_item(ast,item_id);
    }

    fn visit_statement(&mut self,ast: &super::Ast,stmt_id:super::StmtId){
        self.do_visit_statement(ast,stmt_id);
    
    }

    fn do_visit_item(&mut self,ast:&super::Ast,item_id:super::ItemId){
        let item = ast.query_item(item_id);
        match item.kind{
            super::ItemKind::Statement(stmt_id)=>{
                self.visit_statement(ast, stmt_id);
            }
            super::ItemKind::Function(function_id) => todo!(),
        }
    }

    fn do_visit_statement(&mut self,ast: &super::Ast,stmt_id:super::StmtId){
        let stmt = ast.query_stmt(stmt_id);
        match &stmt.stmt_kind{
            super::StatementKind::Expression(expr_id)=>{
                self.visit_expression(ast,*expr_id);
            },
            super::StatementKind::Let(stmt)=>{
                self.visit_let_statement(ast,stmt);
            }
        }
    }

    fn visit_expression(&mut self,ast:&super::Ast, expr_id:super::ExprId){
        let expr = ast.query_expr(expr_id);
        match &expr.kind{
            super::ExpressionKind::Number(number)=>{
                self.visit_number(ast,number);
            },
            super::ExpressionKind::Binary(bin_expr)=>{
                self.visit_binary_expression(ast,bin_expr);
            }
            super::ExpressionKind::Parenthesized(parenthesized_expr) => {
                self.visit_parenthesized_expression(ast, parenthesized_expr);
            },
            super::ExpressionKind::Assignment(assign_expr) => {
                self.visit_assignment_expression(ast, assign_expr);
            }
        }
    }

    fn visit_binary_expression(&mut self,ast: &super::Ast,bin_expr:&super::BinaryExpr){
        self.visit_expression(ast, bin_expr.left);
        let left = self.value.unwrap();
        self.visit_expression(ast, bin_expr.right);
        let right = self.value.unwrap();
       
           self.result =Some( match bin_expr.operator.kind{
            BinOperatorKind::Minus => left - right,
           BinOperatorKind::Plus => left + right,
           BinOperatorKind::Multiply =>left * right,
           BinOperatorKind::Divide => left / right,
          
        });
    }
    
    fn visit_let_statement(&mut self,ast:&super::Ast,stmt:&super::LetStatement) {
        self.visit_expression(ast, stmt.initializer);

    }
    
    fn visit_number(&mut self,_ast: &super::Ast,number:&super::NumberExpr) {
        self.value= Some(number.number);
        
    }
    fn visit_parenthesized_expression(&mut self,ast:&super::Ast,parenthesized_expr:&super::ParenthesizedExpr) {
        self.visit_expression(ast, parenthesized_expr.expr);
    }
}