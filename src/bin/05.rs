fn annihilate(a: &u8, b: &u8) -> bool {
    a.eq_ignore_ascii_case(b) && a != b
}

fn reacted_length(input: impl Iterator<Item=u8>) -> usize {
    let mut stack = Vec::new();
    for byte in input.filter(u8::is_ascii_alphabetic) {
        if stack.last().map(|top| annihilate(top, &byte)).unwrap_or(false) {
            stack.pop();
        } else {
            stack.push(byte);
        }
    }
    stack.len()
}

fn remove_unit(input: impl Iterator<Item=u8>, removal: u8) -> impl Iterator<Item=u8> {
    input.filter(move |c| !c.eq_ignore_ascii_case(&removal))
}

fn part1(input: &str) -> usize {
    reacted_length(input.bytes())
}

#[test]
fn part1examples() {
    assert_eq!(part1("aA"), 0);
    assert_eq!(part1("abBA"), 0);
    assert_eq!(part1("abAB"), 4);
    assert_eq!(part1("aabAAB"), 6);
    assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
}

fn part2(input: &str) -> usize {
    (b'A'..(b'Z' + 1))
        .map(|removal| remove_unit(input.bytes(), removal))
        .map(reacted_length)
        .min()
        .unwrap()
}

#[test]
fn part2examples() {
    assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
}

fn main() {
    aoc::main!(5, part1, part2);
}
