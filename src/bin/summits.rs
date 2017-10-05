extern crate dynamic_programming;

use std::io;
use dynamic_programming::longest_increasing_subsequence;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sequence: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
    let a = longest_increasing_subsequence(sequence.as_slice());
    println!("{:?}", a);
}