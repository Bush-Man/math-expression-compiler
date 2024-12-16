use std::{borrow::{Borrow, BorrowMut}, vec};

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
pub mod global_scope;


id_gen!(ItemId);
id_gen!(StmtId);
id_gen!(ExprId);
id_gen!(VariableId);
id_gen!(FunctionId);

#[derive(Debug)]
pub struct Ast{
   pub items: IdVec<ItemId,Item>,
   pub statements: IdVec<StmtId,Statement>,
   pub expressions: IdVec<ExprId,Expression>,
   pub functions: IdVec<FunctionId,Function>
}

#[derive(Debug,Clone,Copy)]
pub enum ItemKind{
    Statement(StmtId),
    Function(FunctionId)
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
#[derive(Debug)]
pub struct Function{
   pub name:String,
   pub open_paren:Token,
   pub parameters:Vec<Parameter>,
   pub close_paren:Token,
   //pub body:Body,
}
impl Function{
    pub fn new( name:String,open_paren:Token,close_paren:Token,parameters:Vec<Parameter>)->Self{
        Self {
             name,
              open_paren,
               parameters: parameters,
               close_paren,
                // body: body,
             }
}
}
#[derive(Debug,Clone)]
pub struct Parameter{
    pub identifier:Token,
    pub value:i64

}

impl Parameter{
    pub fn new(identifier:Token,value:i64)->Self{
        Self {identifier,value }
    }
}
#[derive(Debug,Clone)]
pub struct Body{
   pub  open_brace:Token,
   pub  statements:Vec<StmtId>,
   pub return_value: Option<i64>,
   pub close_brace:Token
}

impl Body{
    pub fn new(open_brace:Token,statements:Vec<StmtId>,close_brace:Token)->Self{
       Self { open_brace, statements, return_value: None, close_brace }
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
    Binary(BinaryExpr),
    Parenthesized(ParenthesizedExpr),
    Assignment(AssignExpr)
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

#[derive(Debug,Clone)]
pub struct ParenthesizedExpr{
   pub open_paren:Token,
   pub expr:ExprId,
   pub close_paren:Token

}
impl ParenthesizedExpr{
    pub fn new(open_paren:Token,expr:ExprId,close_paren:Token)->Self{
        Self { open_paren, expr, close_paren }
    }
}

#[derive(Debug,Clone)]
pub struct AssignExpr{
   pub let_keyword:Token,
   pub equals:Token,
   pub expr:ExprId
}
impl AssignExpr{
    pub fn new(let_keyword:Token,expr:ExprId,equals:Token)->Self{
        Self { let_keyword, equals , expr}
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
             expressions: IdVec::new(),
             functions:IdVec::new()
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
    pub fn item_from_function_id(&mut self,function_id:FunctionId)->&Item{
        let item_kind = ItemKind::Function(function_id);
        let new_item = Item::new(item_kind, ItemId::new(0));
        let item_id = self.items.push(new_item);
        self.items.get_mut(item_id).id = item_id;
        return self.items.get(item_id);
    }
    pub fn stmt_from_stmt_kind(&mut self,kind:StatementKind)->&Statement{
     let stmt = Statement::new(kind, StmtId::new(0));
     let id = self.statements.push(stmt);
     self.statements.get_mut(id).id = id;

    return self.statements.get(id);
    }
    pub fn save_function(&mut self,name:String,open_paren:Token,close_paren:Token,parameters:Vec<Parameter>)->FunctionId{
        let function = Function::new(name, open_paren, close_paren, parameters);
        let function_id = self.functions.push(function);
        print!("function id:{:?}",function_id);
        return function_id;

        
        

    }

    pub fn save_expression_statement(&mut self,expr_id:ExprId)->&Statement{
     let stmt = Statement::new(StatementKind::Expression(expr_id), StmtId::new(0));
     let id = self.statements.push(stmt);
     self.statements.get_mut(id).id = id;
     return self.statements.get(id);
    }


    pub fn save_parenthesized_expression(&mut self,expr_id:ExprId,open_paren:Token,close_paren:Token)->&Expression{
        self.expr_from_kind(ExpressionKind::Parenthesized(ParenthesizedExpr::new(open_paren, expr_id, close_paren)))
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
           variable_id:VariableId::new(0)}))
    }

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
   

    pub fn save_assignment_expression(&mut self,equals:Token,let_keyword:Token,expr:ExprId)->&Expression{
        return self.expr_from_kind(ExpressionKind::Assignment(AssignExpr{let_keyword,expr,equals}));

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
        if let Some(result) = evaluator.result{
        println!("{}{}"," ".repeat(10),".".repeat(90));
        println!("{}"," ".repeat(20));


        println!("{} Answer: {:?}"," ".repeat(50),result);
        
        println!("{}"," ".repeat(20));

        println!("{}{}"," ".repeat(10),".".repeat(90));
        }
       
       


    }
}