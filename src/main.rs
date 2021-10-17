use std::path::PathBuf;
use structopt::StructOpt;

use aoc2019;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    day: usize,
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => aoc2019::day01::run(),
        _ => println!("Unregonised day argument"),
    };
}
