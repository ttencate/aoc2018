use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let (twos, threes) = input.lines().fold((0, 0), |state, id| {
        let mut counts = HashMap::new();
        id.chars().for_each(|letter| *counts.entry(letter).or_insert(0) += 1);
        (
            state.0 + counts.values().any(|c| *c == 2) as u32,
            state.1 + counts.values().any(|c| *c == 3) as u32,
        )
    });
    twos * threes
}

#[test]
fn part1example() {
    assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
}

fn part2(input: &str) -> String {
    let mut counts = HashMap::new();
    for id in input.lines() {
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
    String::from_utf8(answer).unwrap()
}

#[test]
fn part2example() {
    assert_eq!(part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
}

fn main() {
    aoc::main!(2, part1, part2);
}
