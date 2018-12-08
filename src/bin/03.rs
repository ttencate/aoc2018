extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn parse_input<'a>(input: &'a str) -> Box<impl Iterator<Item=Claim> + 'a> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    Box::new(input
        .lines()
        .map(move |line| {
            let captures = re.captures(&line).unwrap();
            Claim {
                id: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                x: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                y: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                w: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                h: captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            }
        }))
}

fn part1(input: &str) -> usize {
    let mut counts = HashMap::new();
    for claim in parse_input(input) {
        for cy in claim.x..(claim.x + claim.w) {
            for cx in claim.y..(claim.y + claim.h) {
                let key = (cx, cy);
                *counts.entry(key).or_insert(0) += 1;
            }
        }
    }
    counts.values().filter(|c| **c > 1).count()
}

#[test]
fn part1example() {
    assert_eq!(part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
}

fn part2(input: &str) -> u32 {
    let mut candidate_ids = HashSet::new();
    let mut ids = HashMap::new();
    for claim in parse_input(input) {
        candidate_ids.insert(claim.id);
        for cy in claim.x..(claim.x + claim.w) {
            for cx in claim.y..(claim.y + claim.h) {
                let key = (cx, cy);
                if let Some(other_id) = ids.get(&key) {
                    candidate_ids.remove(&claim.id);
                    candidate_ids.remove(&other_id);
                }
                ids.insert(key, claim.id);
            }
        }
    }
    candidate_ids.into_iter().next().unwrap()
}

#[test]
fn part2example() {
    assert_eq!(part2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
}

fn main() {
    aoc::main(part1, part2);
}
