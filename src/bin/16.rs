#[macro_use]
extern crate lazy_static;

use std::fmt::{Display, Formatter};
use regex::Regex;

const NUM_REGISTERS: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Register(usize);

impl Register {
    fn parse(s: &str) -> Option<Register> {
        let idx = s.parse::<usize>().ok()?;
        if idx < NUM_REGISTERS {
            Some(Register(idx))
        } else {
            None
        }
    }
}

type Value = i32;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Registers([Value; NUM_REGISTERS]);

impl Registers {
    fn parse(line: &str) -> Option<Registers> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<registers>.*)\]").unwrap();
        }
        if let [Some(a), Some(b), Some(c), Some(d)] =
            RE.captures(line)?
            .name("registers")
            .unwrap()
            .as_str()
            .split(",")
            .map(|v| v.trim().parse::<i32>().ok())
            .collect::<Vec<Option<i32>>>()
            .as_slice()
        {
            Some(Registers([*a, *b, *c, *d]))
        } else {
            None
        }
    }
}

struct State {
    registers: Registers,
}

impl State {
    fn fetch(&self, reg: Register) -> Value {
        self.registers.0[reg.0]
    }

    fn store(&mut self, reg: Register, val: Value) {
        self.registers.0[reg.0] = val;
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[{:?}]", self.registers)
    }
}

#[derive(Clone, Copy, Debug)]
enum Input {
    Register(Register),
    Value(Value),
}

impl Input {
    fn parse_register(v: &str) -> Option<Input> {
        Some(Input::Register(Register::parse(v)?))
    }

    fn parse_value(v: &str) -> Option<Input> {
        Some(Input::Value(v.parse::<Value>().ok()?))
    }

    fn eval(&self, state: &State) -> Value {
        match self {
            Input::Register(reg) => state.fetch(*reg),
            Input::Value(val) => *val,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Output {
    Register(Register),
}

impl Output {
    fn parse_register(v: &str) -> Option<Output> {
        Some(Output::Register(Register::parse(v)?))
    }

    fn store(&self, val: Value, state: &mut State) {
        match self {
            Output::Register(reg) => state.store(*reg, val),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Add(Input, Input, Output),
    Mul(Input, Input, Output),
    Ban(Input, Input, Output),
    Bor(Input, Input, Output),
    Set(Input, Output),
    Gt(Input, Input, Output),
    Eq(Input, Input, Output),
}

impl Instruction {
    fn parse(opcode: &str, a: &str, b: &str, c: &str) -> Option<Instruction> {
        let reg = Input::parse_register;
        let val = Input::parse_value;
        let out = Output::parse_register;
        match opcode {
            "addr" => Some(Instruction::Add(reg(a)?, reg(b)?, out(c)?)),
            "addi" => Some(Instruction::Add(reg(a)?, val(b)?, out(c)?)),
            "mulr" => Some(Instruction::Mul(reg(a)?, reg(b)?, out(c)?)),
            "muli" => Some(Instruction::Mul(reg(a)?, val(b)?, out(c)?)),
            "banr" => Some(Instruction::Ban(reg(a)?, reg(b)?, out(c)?)),
            "bani" => Some(Instruction::Ban(reg(a)?, val(b)?, out(c)?)),
            "borr" => Some(Instruction::Bor(reg(a)?, reg(b)?, out(c)?)),
            "bori" => Some(Instruction::Bor(reg(a)?, val(b)?, out(c)?)),
            "setr" => Some(Instruction::Set(reg(a)?, out(c)?)),
            "seti" => Some(Instruction::Set(val(a)?, out(c)?)),
            "gtir" => Some(Instruction::Gt(val(a)?, reg(b)?, out(c)?)),
            "gtri" => Some(Instruction::Gt(reg(a)?, val(b)?, out(c)?)),
            "gtrr" => Some(Instruction::Gt(reg(a)?, reg(b)?, out(c)?)),
            "eqir" => Some(Instruction::Eq(val(a)?, reg(b)?, out(c)?)),
            "eqri" => Some(Instruction::Eq(reg(a)?, val(b)?, out(c)?)),
            "eqrr" => Some(Instruction::Eq(reg(a)?, reg(b)?, out(c)?)),
            _ => None
        }
    }

    fn parse_multiple(line: &str) -> Vec<Instruction> {
        if let [_opcode, input_a, input_b, output] = line.split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
                .iter()
                .filter_map(|opcode| Self::parse(opcode, input_a, input_b, output))
                .collect()
        } else {
            vec![]
        }
    }

    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Add(a, b, c) => c.store(a.eval(state) + b.eval(state), state),
            Instruction::Mul(a, b, c) => c.store(a.eval(state) * b.eval(state), state),
            Instruction::Ban(a, b, c) => c.store(a.eval(state) & b.eval(state), state),
            Instruction::Bor(a, b, c) => c.store(a.eval(state) | b.eval(state), state),
            Instruction::Set(a, c) => c.store(a.eval(state), state),
            Instruction::Gt(a, b, c) => c.store(if a.eval(state) > b.eval(state) { 1 } else { 0 }, state),
            Instruction::Eq(a, b, c) => c.store(if a.eval(state) == b.eval(state) { 1 } else { 0 }, state),
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut answer = 0;
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }
        if let Some(regs_before) = Registers::parse(line) {
            let instructions = Instruction::parse_multiple(lines.next().unwrap());
            let regs_after = Registers::parse(lines.next().unwrap()).unwrap();
            if instructions
                .iter()
                .map(|instr| {
                    let mut state = State { registers: regs_before.clone() };
                    instr.execute(&mut state);
                    state
                })
                .filter(|state| state.registers == regs_after)
                .count()
                >= 3
            {
                answer += 1;
            }
        }
    }
    answer
}

#[test]
fn part1example() {
    assert_eq!(part1("Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"), 1);
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
