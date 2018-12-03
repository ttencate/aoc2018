extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let input = io::stdin();
    let mut candidate_ids = HashSet::new();
    let mut counts = HashMap::new();
    for line in input.lock().lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();
        let (id, x, y, w, h) = (
            captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(5).unwrap().as_str().parse::<u32>().unwrap());
        candidate_ids.insert(id);
        for cy in x..(x + w) {
            for cx in y..(y + h) {
                let key = (cx, cy);
                if let Some(other_id) = counts.get(&key) {
                    candidate_ids.remove(&id);
                    candidate_ids.remove(&other_id);
                }
                counts.insert(key, id);
            }
        }
    }
    println!("{}", candidate_ids.into_iter().next().unwrap());
}
