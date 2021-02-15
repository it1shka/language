use std::io::stdin;
use super::launcher::launch;

pub fn enter_repl() {
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).expect("Ooops!");
        if input == "exit" {
            break;
        }
        launch(&input);
        input.clear();
    }
}