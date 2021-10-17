use std::fs;
use std::io::Read;

pub fn run() -> usize {
    let file = fs::File::open("data/01.txt").unwrap();
    let s = fs::read_to_string(file).unwrap();
    println!("{}", s);
    1
}
