extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let input = io::stdin();
    let mut counts = HashMap::new();
    for line in input.lock().lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();
        let (_id, x, y, w, h) = (
            captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(5).unwrap().as_str().parse::<u32>().unwrap());
        for cy in x..(x + w) {
            for cx in y..(y + h) {
                let key = (cx, cy);
                *counts.entry(key).or_insert(0) += 1;
            }
        }
    }
    println!("{}", counts.values().filter(|c| **c > 1).count());
}
