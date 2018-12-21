use aoc::vm::*;
use std::collections::HashSet;

fn part1(input: &str) -> Value {
    let prog = Program::parse(input);
    let mut state = State::new(6);
    while state.ip() != prog.instructions().len() - 1 {
        prog.execute_one(&mut state);
    }
    state.fetch(1).unwrap()
}

fn part2(_input: &str) -> Value {
    let mut seen = HashSet::new();
    let mut prev = 0;
    for b in opt_iter() {
        if seen.contains(&b) {
            return prev;
        }
        prev = b;
        seen.insert(b);
    }
    panic!();
}

struct OptIter {
    b: Value
}

impl Iterator for OptIter {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        let mut c: Value = self.b | 0x10000;
        self.b = 10605201;
        loop {
            self.b = ((self.b + (c & 0xff)) * 65899) & 0xffffff;
            if c < 0x100 {
                break;
            }
            c /= 256;
        }
        Some(self.b)
    }
}

fn opt_iter() -> OptIter {
    OptIter { b: 0 }
}

#[cfg(test)]
struct RealIter {
    prog: Program,
    state: State,
}

#[cfg(test)]
fn real_iter(input: &str) -> RealIter {
    let prog = Program::parse(input);
    let state = State::new(6);
    RealIter { prog, state }
}

#[cfg(test)]
impl Iterator for RealIter {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        loop {
            self.prog.execute_one(&mut self.state);
            if self.state.ip() == self.prog.instructions().len() - 1 {
                break Some(self.state.fetch(1).unwrap());
            }
        }
    }
}

#[test]
fn test_sequence() {
    let opt = opt_iter();
    let real = real_iter("#ip 3
seti 123 0 1
bani 1 456 1
eqri 1 72 1
addr 1 3 3
seti 0 0 3
seti 0 0 1
bori 1 65536 2
seti 10605201 9 1
bani 2 255 5
addr 1 5 1
bani 1 16777215 1
muli 1 65899 1
bani 1 16777215 1
gtir 256 2 5
addr 5 3 3
addi 3 1 3
seti 27 3 3
seti 0 3 5
addi 5 1 4
muli 4 256 4
gtrr 4 2 4
addr 4 3 3
addi 3 1 3
seti 25 3 3
addi 5 1 5
seti 17 5 3
setr 5 5 2
seti 7 6 3
eqrr 1 0 5
addr 5 3 3
seti 5 8 3");
    let count = 10;
    assert_eq!(opt.take(count).collect::<Vec<Value>>(), real.take(count).collect::<Vec<Value>>());
}

fn main() {
    aoc::main(part1, part2);
}
