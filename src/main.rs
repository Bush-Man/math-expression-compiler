use ast::{ global_scope::{self, GlobalScope}, lexer::{Lexer, Token}, parser::Parser, Ast};


mod ast;

fn main() {
    // global_scope.add_global_variable(String::from("a"), String::from("5"));
    
        
        let input:&str = "
        function multiply(){
          let b = 6
          let a = 7
        }";
        
        let mut lexer = Lexer::new(input);
        let mut tokens:Vec<Token> = Vec::new(); 
        while let Some(token) = lexer.next_token(){
            tokens.push(token);
        }
    let global_scope = &mut GlobalScope::new();
    let ast = &mut Ast::new();
    let mut parser:Parser<'_> = Parser::new(tokens, ast,global_scope);
    parser.parse();
    // println!("parser tokens: {:?}",parser.tokens);
    println!("global variables: {:?}",global_scope.get_all_global_variables());
    // // ast.visualize();

    // // ast.evaluate();

   
}
