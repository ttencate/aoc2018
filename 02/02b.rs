use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin();
    let mut counts = HashMap::new();
    input.lock().lines().for_each(|line| {
        let id = line.unwrap();
        (0..id.len()).for_each(|skip_index| {
            let mut key: Vec<u8> = id.bytes().collect();
            key[skip_index] = '_' as u8;
            *counts.entry(key).or_insert(0) += 1;
        });
    });
    let answer = counts.into_iter()
        .find(|(_, value)| *value == 2)
        .unwrap()
        .0
        .into_iter()
        .filter(|c| *c != '_' as u8)
        .collect();
    println!("{}", String::from_utf8(answer).unwrap());
}
