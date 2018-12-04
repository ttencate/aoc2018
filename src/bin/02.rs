use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

fn main() {
    let input = io::stdin();
    let lines = Vec::from_iter(input.lock().lines().filter_map(Result::ok));

    let (twos, threes) = lines.iter().fold((0, 0), |state, id| {
        let mut counts = HashMap::new();
        id.chars().for_each(|letter| *counts.entry(letter).or_insert(0) += 1);
        (
            state.0 + counts.values().any(|c| *c == 2) as u32,
            state.1 + counts.values().any(|c| *c == 3) as u32,
        )
    });
    println!("{}", twos * threes);

    let mut counts = HashMap::new();
    for id in lines {
        (0..id.len()).for_each(|skip_index| {
            let mut key: Vec<u8> = id.bytes().collect();
            key[skip_index] = '_' as u8;
            *counts.entry(key).or_insert(0) += 1;
        });
    };
    let answer = counts.into_iter()
        .find(|(_, value)| *value == 2)
        .unwrap()
        .0
        .into_iter()
        .filter(|c| *c != '_' as u8)
        .collect();
    println!("{}", String::from_utf8(answer).unwrap());
}
