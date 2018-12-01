use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin();
    let sum: i32 = input.lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .sum();
    println!("{}", sum);
}
