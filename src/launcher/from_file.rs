use super::launcher::launch;

use std::env;
use std::fs;

pub fn launch_from_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Can't read file!");
    launch(&contents);
}