use std::fs::File;
use std::io::prelude::*;
mod q1b;

fn main() {
    let mut file = File::open("/home/gradf/Desktop/advent-of-code-2023/day1/tests/1b.txt")
        .expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("{}", q1b::solve(&contents));
}
