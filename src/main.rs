mod lexer;
mod parser;

fn main() {
    let source = String::from("i = 1; while(i < 100) { print(i); i = i + 1; }");
    let token_stream = lexer::TokenStream::new(&source);
    for token in token_stream {
        println!("{:#?}", token);
    }
}