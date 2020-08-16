use std::{env::args, fs, process::exit, io::{self, Write}};

mod lexer;
mod parser;
mod evaluator;

use evaluator::environment;

fn main() {
    
    let mut args: Vec<String> = args()
        .collect();

    if args.len() == 2 {

        interpreter(match fs::read_to_string(args.pop().unwrap()) {
            Ok(i)  => i,
            Err(e) => {eprintln!("{}", e); exit(3)},
        });

    } else {

        loop {

            let mut code = String::new();
            
            print!("> ");
            io::stdout().flush().unwrap();

            if let Err(e) = io::stdin().read_line(&mut code) {
                eprintln!("{}", e); exit(4) 
            }

            if code != "exit\n" {
                interpreter(code)
            } else {
                break;
            }
        }

    }

}

fn interpreter(code: String) {
    let mut lexer = lexer::new(&code);

    let tokens = lexer.get_tokens();

    let mut parser = parser::new(tokens);

    let ast = parser.parse_program();

    let env = environment::new();

    let result = evaluator::run_program(ast, env);

    println!("{}", match result {
        Ok(i)  => i,
        Err(e) => {eprintln!("{}", e); exit(2);}
    });
}
