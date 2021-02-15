mod lexer;
mod parser;
mod runner;
mod launcher;

use launcher::from_file::launch_from_file;
use launcher::repl::enter_repl;

use std::env;

fn main() {
    let file_path = env::args().nth(1);
    match file_path {
        None => enter_repl(),
        Some(path) => launch_from_file(&path)
    }
}