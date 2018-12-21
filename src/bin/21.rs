use aoc::vm::*;

fn part1(input: &str) -> Value {
    // "\n".to_string() + &Program::parse(input).decompile().to_string()
    let prog = Program::parse(input);
    let mut state = State::new(6);
    while state.ip() != prog.instructions().len() - 1 {
        prog.execute_one(&mut state);
    }
    state.fetch(1).unwrap()
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
