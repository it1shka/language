mod lexer;
mod parser;
mod runner;

mod launcher;
use launcher::launcher::launch;

use std::io::stdin;

fn main() {
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).expect("Oooops! Sorry!");
        
        /*let stream = Stream::new(&input);
        let mut builder = Builder::new(stream);
        let ast = builder.build();
        println!("{:?}", ast);*/

        launch(&input);

        input.clear();
    }
}