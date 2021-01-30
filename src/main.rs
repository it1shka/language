mod lexer;
use lexer::stream::Stream;
use lexer::token::Token;

use std::io::stdin;

fn main() {
    let mut user_input = String::new();
    loop {
        stdin().read_line(&mut user_input).expect("err");
        let mut stream = Stream::new(&user_input);
        
        loop {
            let nxt = stream.next().unwrap();
            let (line, col) = stream.get_pos();
            if let Ok(Token::EOF) = nxt {
                break;
            }
            if let Err(err) = nxt {
                println!("{}", err);
                break;
            }
            if let Ok(some) = nxt {
                println!("{:#?}", some);
                println!("{}:{}", line, col);
            }
            
            println!("----------------------------");
        }

        println!("\n\n\n\n");

        user_input.clear();
    }
}