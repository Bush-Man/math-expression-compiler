use super::{visitor::Visitor, BinOperatorKind};

 pub struct ExpressionEvaluator{
    pub result:Option<i64>
}


impl ExpressionEvaluator{
    pub fn new()->Self{
        Self { result: None }
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
        }
    }

    fn visit_binary_expression(&mut self,ast: &super::Ast,bin_expr:&super::BinaryExpr){
        self.visit_expression(ast, bin_expr.left);
        let left = self.result.unwrap();
        // println!("Left: {:?}",left);
        self.visit_expression(ast, bin_expr.right);
        let right = self.result.unwrap();
        self.result = Some(match bin_expr.operator.kind{
           BinOperatorKind::Plus => left + right,
           BinOperatorKind::Minus => left - right,
           BinOperatorKind::Multiply => left * right,
           BinOperatorKind::Divide => left / right,

        });
    }
    
    fn visit_let_statement(&mut self,ast:&super::Ast,stmt:&super::LetStatement) {
        self.visit_expression(ast, stmt.initializer);

    }
    
    fn visit_number(&mut self,ast: &super::Ast,number:&super::NumberExpr) {
        self.result = Some(number.number);
        
    }
}