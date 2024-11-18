use ast::{ lexer::{Lexer, Token}, parser::Parser, Ast};


mod ast;

fn main() {
    let input:&str = "3 * (4+1-(4/2))";
    let mut lexer = Lexer::new(input);
    let mut tokens:Vec<Token> = Vec::new(); 
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    
    let ast = &mut Ast::new();
    let mut parser = Parser::new(tokens, ast);
    parser.parse();
    ast.visualize();

    ast.evaluate();

   
}
