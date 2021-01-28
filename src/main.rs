mod lexer;
mod parser;

use lexer::TokenStream;
use lexer::TokenInfo;
use lexer::Token;

use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    loop {
        stdin().read_line(&mut user_input);
        let token_stream = TokenStream::new(&user_input);
        for TokenInfo{token, ..} in token_stream {
            println!("{:#?}", token)
        }
        println!("------------------------------------");
        user_input.clear();
    }
}