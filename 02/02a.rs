use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin();
    let (twos, threes) = input.lock().lines().fold((0, 0), |state, id| {
        let mut counts = HashMap::new();
        id.unwrap().chars().for_each(|letter| *counts.entry(letter).or_insert(0) += 1);
        (
            state.0 + counts.values().any(|c| *c == 2) as u32,
            state.1 + counts.values().any(|c| *c == 3) as u32,
        )
    });
    println!("{}", twos * threes);
}
