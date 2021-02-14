mod lexer;
use lexer::stream::Stream;
use lexer::token::Token;

mod parser;
use parser::builder::Builder;

//mod runner;


use std::io::stdin;

fn main() {
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).expect("Oooops! Sorry!");
        
        let stream = Stream::new(&input);
        let mut builder = Builder::new(stream);
        let ast = builder.build();
        println!("{:?}", ast);

        input.clear();
    }
}