use crate::lexer::stream::Stream;
use crate::parser::builder::Builder;
use crate::runner::interpreter::Engine;

use std::time::Instant;

pub fn launch(code: &str) {
    let stream = Stream::new(code);
    let mut builder = Builder::new(stream);
    let ast = builder.build();
    match ast {
        Ok(program) => {
            let start = Instant::now();
            let mut engine = Engine::new();
            match engine.run(&program) {
                Ok(()) => {
                    let time_wasted = start.elapsed().as_millis();
                    println!("Finished with time: {}ms", time_wasted);
                },
                Err(exec_error) => {
                    println!("From execution: {}", exec_error);
                }
            }
        },
        Err(parser_error) => {
            println!("From parser: {}", parser_error);
        }
    }
}