mod lexer;

fn main() {
    let source = String::from("a = 5;//comment");
    let token_stream = lexer::TokenStream::new(&source);
    for token in token_stream {
        println!("{:#?}", token);
    }
}