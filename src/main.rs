mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::io;

fn main() {
    println!("Lexer REPL started");
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(line.as_bytes());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        match program {
            Ok(program) => println!("{program}"),
            Err(errors) => println!("errors"), // dbg!(errors),
        };
    }
}
