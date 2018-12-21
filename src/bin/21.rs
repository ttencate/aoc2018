use aoc::vm::*;
use aoc::vm::decompiler::Decompile;

fn part1(input: &str) -> String {
    "\n".to_string() + &Program::parse(input).decompile().to_string()
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

#[test]
fn part2example() {
    assert_eq!(part2(""), "TODO");
}

fn main() {
    aoc::main(part1, part2);
}
