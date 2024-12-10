use ast::{ lexer::{Lexer, Token}, parser::Parser, Ast};


mod ast;

fn main() {
    let input:&str = "function solveExpression(1,2,3,77,5,67696,8){}";

    let mut lexer = Lexer::new(input);
    let mut tokens:Vec<Token> = Vec::new(); 
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    let ast = &mut Ast::new();
    let mut parser = Parser::new(tokens, ast);
    print!("{:?}",parser.tokens);
    // parser.parse();
    // ast.visualize();

    // ast.evaluate();

   
}
