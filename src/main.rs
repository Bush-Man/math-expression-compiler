use ast::{lexer::{Lexer, Token}, parser::Parser, Ast};


mod ast;

fn main() {
    let input:&str = "let answer = 1-1-1";
    let mut lexer = Lexer::new(input);
    let mut tokens:Vec<Token> = Vec::new(); 
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }

    let mut ast = &mut Ast::new();
    let mut parser = Parser::new(tokens, ast);
    parser.parse();
    ast.visualize();
   
}
