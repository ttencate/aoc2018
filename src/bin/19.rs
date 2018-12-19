use aoc::vm::*;

fn part1(input: &str) -> Value {
    let program = Program::parse(input);
    let mut state = State::new(6);
    program.execute(&mut state);
    state.fetch(0).unwrap()
}

#[test]
fn part1example() {
    assert_eq!(part1("#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"), 6);
}

fn sum_of_divisors(r3: u64) -> u64 {
    (1..=r3).filter(|r4| r3 % r4 == 0).sum()
}

#[test]
fn test_sum_of_divisors() {
    assert_eq!(sum_of_divisors(974), 1464);
}

fn part2(_input: &str) -> u64 {
    sum_of_divisors(10551374)
}

fn main() {
    aoc::main(part1, part2);
}
