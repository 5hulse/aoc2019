pub mod day01;
pub mod day02;

use std::fs;
use std::io::Read;

pub fn load_input(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
