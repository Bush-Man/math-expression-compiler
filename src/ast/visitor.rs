use super::{Ast, BinaryExpr, ExprId, ExpressionKind, ItemId, ItemKind, LetStatement, NumberExpr, ParenthesizedExpr, StatementKind, StmtId};


pub trait Visitor{
    fn visit_item(&mut self,ast:&Ast,item_id:ItemId){
        self.do_visit_item(ast,item_id);
    }
    fn visit_statement(&mut self,ast: &Ast,stmt_id:StmtId){
        self.do_visit_statement(ast,stmt_id);
        
    }

    fn do_visit_item(&mut self,ast:&Ast,item_id:ItemId){
        let item = ast.query_item(item_id);
        match item.kind{
            ItemKind::Statement(stmt_id)=>{
                self.visit_statement(ast, stmt_id);
            }
        }
    }
    fn do_visit_statement(&mut self,ast: &Ast,stmt_id:StmtId){
        let stmt = ast.query_stmt(stmt_id);
        match &stmt.stmt_kind{
            StatementKind::Expression(expr_id)=>{
                self.visit_expression(ast,*expr_id);
            },
            StatementKind::Let(stmt)=>{
                self.visit_let_statement(ast,stmt);
            }
        }
    }
    fn visit_expression(&mut self,ast:&Ast, expr_id:ExprId){
        let expr = ast.query_expr(expr_id);
        match &expr.kind{
            ExpressionKind::Number(number)=>{
                self.visit_number(ast,number);
            },
            ExpressionKind::Binary(bin_expr)=>{
                self.visit_binary_expression(ast,bin_expr);
            }
            ExpressionKind::Parenthesized(parenthesized_expr) => {
                self.visit_parenthesized_expression(ast,parenthesized_expr);
            }
        }
    }
    fn visit_let_statement(&mut self,ast:&Ast,stmt:&LetStatement);
    fn visit_number(&mut self,ast: &Ast,number:&NumberExpr);
    fn visit_binary_expression(&mut self,ast: &Ast,bin_expr:&BinaryExpr){
        self.visit_expression(ast, bin_expr.left);
        self.visit_expression(ast, bin_expr.right);
    }
    fn visit_parenthesized_expression(&mut self,ast:&Ast,parenthesized_expr:&ParenthesizedExpr){
          self.visit_expression(ast, parenthesized_expr.expr);
    }
}