use std::{env, fs};

mod lexer;
mod parser;

fn main() {
    
    let path = env::args()
        .skip(1)
        .next()
        .unwrap();

    let code = fs::read_to_string(path).unwrap();

    let mut lexer = lexer::new(&code);

    let tokens = lexer.get_tokens();

    let mut parser = parser::new(&tokens);

    let ast = parser.parse_program();

    println!("{:?}", ast);
}
