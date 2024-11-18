use std::borrow::{Borrow, BorrowMut};

use evaluator::ExpressionEvaluator;
use lexer::{Token, TokenKind};
use lib::{Id, IdVec};
use printer::Printer;
use text::TextSpan;
use visitor::Visitor;

use crate::id_gen;

pub mod lexer;
pub mod text;
pub mod parser;
pub mod lib;
pub mod visitor;
pub mod printer;
pub mod evaluator;


id_gen!(ItemId);
id_gen!(StmtId);
id_gen!(ExprId);
id_gen!(VariableId);

#[derive(Debug)]
pub struct Ast{
   pub items: IdVec<ItemId,Item>,
   pub statements: IdVec<StmtId,Statement>,
   pub expressions: IdVec<ExprId,Expression>
}


#[derive(Debug,Clone,Copy)]
pub enum ItemKind{
    Statement(StmtId)
}
#[derive(Debug,Clone,Copy)]
pub struct Item{
  pub kind:ItemKind,
  pub id: ItemId
  
}

impl Item{
    pub fn new(kind:ItemKind,id:ItemId)->Self{
        Self{kind,id}
    }
}
#[derive(Debug,Clone)]
pub enum StatementKind{
    Let(LetStatement),
    Expression(ExprId)
   
}
#[derive(Debug,Clone)]
pub struct Statement{
   pub id:StmtId,
   pub stmt_kind:StatementKind

}
impl Statement{
  pub fn new(kind:StatementKind,id:StmtId)->Self{
    Self { stmt_kind: kind, id }
  }

}

#[derive(Debug,Clone)]
pub enum ExpressionKind{
    Number(NumberExpr),
    Binary(BinaryExpr)
}
#[derive(Debug,Clone)]
pub struct NumberExpr{
    pub number:i64,
    pub token:Token

}
#[derive(Debug,Clone)]
pub struct Expression{
    pub kind:ExpressionKind,
    pub id: ExprId
}

impl Expression{
    pub fn new(kind:ExpressionKind,id:ExprId)->Self{
        Self { kind , id }
    }
}

#[derive(Debug,Clone)]
pub struct BinaryExpr{
   pub left:ExprId,
   pub operator: BinOperator,
   pub right:ExprId
}
impl BinaryExpr{
    pub fn new(left:ExprId,operator:BinOperator,right:ExprId)->Self{
        Self { left , operator , right }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum BinOperatorAssiciativity{
    Left,
    Right
}
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct BinOperator{
   pub kind:BinOperatorKind,
   pub token:Token
}
impl BinOperator{
    pub fn new(kind:BinOperatorKind,token:Token)->Self{
        Self { kind, token }
    }

    pub fn precedence(&mut self)->u8{
        return match self.kind{
            BinOperatorKind::Multiply => 19,
            BinOperatorKind::Divide => 19,
            BinOperatorKind::Plus => 18,
            BinOperatorKind::Minus => 18,
          
        }

    }

    pub fn assicativity(&self)->BinOperatorAssiciativity{
        return BinOperatorAssiciativity::Left
    }
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum BinOperatorKind{
    Plus,
    Minus,
    Multiply,
    Divide
}
#[derive(Debug,Clone)]
pub struct LetStatement{
    pub let_keyword:TokenKind,
    pub identifier:Token,
    pub initializer:ExprId,
    pub  variable_id:VariableId

}

impl Ast{

    pub fn new()->Self{
        Self { 
             items: IdVec::new(),
             statements: IdVec::new(), 
             expressions: IdVec::new() 
            }
    }
    pub fn query_item(&self,item_id:ItemId)->&Item{
        return self.items.get(item_id);
    }
     pub fn query_stmt(&self,stmt_id: StmtId)->&Statement{
        return self.statements.get(stmt_id);
    }

     pub fn query_expr(&self,expr_id: ExprId)->&Expression{
        return self.expressions.get(expr_id);
    }

    pub fn item_from_stmt_id(&mut self , stmt_id: StmtId)->&Item{
        let kind =ItemKind::Statement(stmt_id);
        let item = Item::new(kind, ItemId::new(0));
        let id = self.items.push(item);
         self.items.get_mut(id).id = id;

         return self.items.get(id);



    }
    pub fn stmt_from_stmt_kind(&mut self,kind:StatementKind)->&Statement{
     let stmt = Statement::new(kind, StmtId::new(0));
     let id = self.statements.push(stmt);
     self.statements.get_mut(id).id = id;

    return self.statements.get(id);
    }

    pub fn save_let_statement(
        &mut self,
    identifier:Token,
    initializer:ExprId
    )->&Statement{
       self.stmt_from_stmt_kind(StatementKind::Let(LetStatement {
         let_keyword: TokenKind::Let,
          identifier,
          initializer,
           variable_id:VariableId::new(0)}))}

    pub fn expr_from_kind(&mut self,kind:ExpressionKind)->&Expression{
        let expression = Expression::new(kind, ExprId::new(0));
        let expr_id = self.expressions.push(expression);
        self.expressions.get_mut(expr_id).id = expr_id;
        return &self.expressions.get(expr_id);
    }

    pub fn save_number_expression(&mut self,token:Token,number:i64)->&Expression{
       return self.expr_from_kind(ExpressionKind::Number(NumberExpr{ number,token}));
        
    }

    pub fn save_binary_expression(&mut self,operator:BinOperator,left:ExprId,right:ExprId)->&Expression{
        return self.expr_from_kind(ExpressionKind::Binary(BinaryExpr { left, operator, right }))
    }


    pub fn visit(&mut self,visitor:&mut dyn Visitor){
       for item in self.items.iter(){
        visitor.visit_item(self,item.id);
       }
     
    }
    pub fn visualize(&mut self){
        let mut printer = Printer::new(0);
        self.visit(&mut printer);

    }
    pub fn evaluate(&mut self){
        let mut evaluator = ExpressionEvaluator::new();
        self.visit(&mut evaluator);
        println!("{}{}"," ".repeat(50),".".repeat(50));
        println!("{}"," ".repeat(20));


        println!("{} Answer: {:?}"," ".repeat(50),evaluator.result.unwrap());
        
        println!("{}"," ".repeat(20));

        println!("{}{}"," ".repeat(50),".".repeat(50));


    }
}