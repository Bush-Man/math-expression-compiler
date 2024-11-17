use std::any::{Any};

use super::visitor::Visitor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Printer {
    pub indent: usize,
}

impl Printer {
    pub fn new(indent: usize) -> Self {
        Self { indent }
    }

    pub fn print_with_indent(&self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
    pub fn print_same_line(&self,data:&str){
        print!(" {}",data);
    }

    fn enter_scope(&mut self) {
        self.indent += 2;
    }

    fn exit_scope(&mut self) {
        if self.indent >= 2 {
            self.indent -= 2;
        }
    }
}

impl Visitor for Printer {
    
    fn visit_statement(&mut self, ast: &super::Ast, stmt_id: super::StmtId) {
        self.print_same_line("Statement >");
        self.do_visit_statement(ast, stmt_id);

    }

     fn visit_let_statement(&mut self, ast: &super::Ast, stmt: &super::LetStatement) {
        self.print_same_line("Let Statement > ");

        self.print_same_line("Identifier > ");
        self.print_with_indent(&format!(" {} ", &stmt.identifier.span.literal));
        self.exit_scope();

        self.print_same_line("Initializer:");
        // self.enter_scope();
        self.visit_expression(ast, stmt.initializer);
        // self.exit_scope();

    }

    fn visit_binary_expression(&mut self, ast: &super::Ast, bin_expr: &super::BinaryExpr) {
        self.print_with_indent("Binary Expression: {");
       self.enter_scope();
        self.print_with_indent("Left:");
        self.enter_scope();
        self.visit_expression(ast, bin_expr.left);
        self.print_with_indent(&format!("Operator: {:?}",bin_expr.operator.kind));
        self.enter_scope();
         self.exit_scope();
        self.print_with_indent("Right:");
        self.enter_scope();
        self.visit_expression(ast, bin_expr.right);
        self.exit_scope();
        self.print_with_indent("}");

    }

   

    /// Visits a number expression and prints its value.
    fn visit_number(&mut self, _ast: &super::Ast, number: &super::NumberExpr) {
        self.print_with_indent("Number:");
        self.enter_scope();
        self.print_with_indent(&number.number.to_string());
        self.exit_scope();
    }
}
