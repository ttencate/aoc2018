#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use regex::Regex;

const NUM_REGISTERS: usize = 4;

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
    fn new() -> State {
        State { registers: Registers([0; NUM_REGISTERS]) }
    }

    #[allow(dead_code)]
    fn with_registers(registers: [Value; NUM_REGISTERS]) -> State {
        State { registers: Registers(registers) }
    }

    fn fetch(&self, reg: Value) -> Option<Value> {
        let idx = self.register_index(reg)?;
        Some(self.registers.0[idx])
    }

    fn store(&mut self, reg: Value, val: Value) -> Option<()> {
        let idx = self.register_index(reg)?;
        self.registers.0[idx] = val;
        Some(())
    }

    fn register_index(&self, reg: Value) -> Option<usize> {
        if reg >= 0 && (reg as usize) < NUM_REGISTERS { Some(reg as usize) } else { None }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[{:?}]", self.registers)
    }
}

#[derive(Clone, Copy, Debug)]
struct Input(Value);

impl Input {
    fn reg(&self, state: &State) -> Option<Value> {
        state.fetch(self.0)
    }

    fn val(&self) -> Option<Value> {
        Some(self.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Output(Value);

impl Output {
    fn store(&self, state: &mut State, val: Value) -> Option<()> {
        state.store(self.0, val)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

const ALL_OPCODES: &[Opcode] = &[
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    a: Input,
    b: Input,
    c: Output,
}

impl Instruction {
    fn parse_line(line: &str) -> Option<(usize, Value, Value, Value)> {
        if let [Some(opcode_idx), Some(a), Some(b), Some(c)] = line.split_whitespace()
            .map(|v| v.parse::<Value>().ok())
            .collect::<Vec<Option<Value>>>()
            .as_slice()
        {
            Some((*opcode_idx as usize, *a, *b, *c))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn parse(line: &str) -> Option<Instruction> {
        if let [opcode_str, a, b, c] = line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            let opcode = ALL_OPCODES.iter().find(|opcode| opcode.to_string() == *opcode_str)?;
            Some(Instruction {
                opcode: *opcode,
                a: Input(a.parse::<Value>().ok()?),
                b: Input(b.parse::<Value>().ok()?),
                c: Output(c.parse::<Value>().ok()?),
            })
        } else {
            None
        }
    }

    fn parse_with_opcode_map(opcode_map: &HashMap<usize, Opcode>, line: &str) -> Option<Instruction> {
        Self::parse_line(line)
            .and_then(|(opcode_idx, a, b, c)| {
                Some(Instruction {
                    opcode: *(opcode_map.get(&opcode_idx)?),
                    a: Input(a),
                    b: Input(b),
                    c: Output(c),
                })
            })
    }

    fn parse_multiple(line: &str) -> Option<(usize, Vec<Instruction>)> {
        Self::parse_line(line)
            .map(|(opcode_idx, a, b, c)| {
                (
                    opcode_idx,
                    ALL_OPCODES.iter()
                        .map(|opcode| {
                            Instruction {
                                opcode: *opcode,
                                a: Input(a),
                                b: Input(b),
                                c: Output(c),
                            }
                        })
                        .collect()
                )
            })
    }

    fn execute(&self, state: &mut State) -> Option<()> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        match self.opcode {
            Opcode::Addr => c.store(state, a.reg(state)? + b.reg(state)?)?,
            Opcode::Addi => c.store(state, a.reg(state)? + b.val()?)?,
            Opcode::Mulr => c.store(state, a.reg(state)? * b.reg(state)?)?,
            Opcode::Muli => c.store(state, a.reg(state)? * b.val()?)?,
            Opcode::Banr => c.store(state, a.reg(state)? & b.reg(state)?)?,
            Opcode::Bani => c.store(state, a.reg(state)? & b.val()?)?,
            Opcode::Borr => c.store(state, a.reg(state)? | b.reg(state)?)?,
            Opcode::Bori => c.store(state, a.reg(state)? | b.val()?)?,
            Opcode::Setr => c.store(state, a.reg(state)?)?,
            Opcode::Seti => c.store(state, a.val()?)?,
            Opcode::Gtir => c.store(state, if a.val()? > b.reg(state)? { 1 } else { 0 })?,
            Opcode::Gtri => c.store(state, if a.reg(state)? > b.val()? { 1 } else { 0 })?,
            Opcode::Gtrr => c.store(state, if a.reg(state)? > b.reg(state)? { 1 } else { 0 })?,
            Opcode::Eqir => c.store(state, if a.val()? == b.reg(state)? { 1 } else { 0 })?,
            Opcode::Eqri => c.store(state, if a.reg(state)? == b.val()? { 1 } else { 0 })?,
            Opcode::Eqrr => c.store(state, if a.reg(state)? == b.reg(state)? { 1 } else { 0 })?,
        }
        Some(())
    }
}

#[allow(dead_code)]
#[inline]
fn test_instruction(regs_before: [Value; NUM_REGISTERS], instr_str: &str, regs_after: [Value; NUM_REGISTERS]) {
    let mut state = State::with_registers(regs_before);
    Instruction::parse(instr_str).unwrap().execute(&mut state);
    assert_eq!(state.registers.0, regs_after);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut answer = 0;
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }
        if let Some(regs_before) = Registers::parse(line) {
            let (_opcode_idx, instructions) = Instruction::parse_multiple(lines.next().unwrap()).unwrap();
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

fn part2(input: &str) -> Value {
    let mut lines = input.lines();
    let mut blank_lines = 0;

    let mut candidates: HashMap<usize, HashSet<Opcode>> =
        (0..ALL_OPCODES.len())
        .map(|idx| (idx, ALL_OPCODES.iter().map(|opcode| *opcode).collect()))
        .collect();

    while blank_lines < 3 {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            blank_lines += 1;
            continue;
        }
        blank_lines = 0;

        let regs_before = Registers::parse(line).unwrap();
        let (opcode_idx, instructions) = Instruction::parse_multiple(lines.next().unwrap()).unwrap();
        let regs_after = Registers::parse(lines.next().unwrap()).unwrap();

        for instr in instructions {
            let mut state = State { registers: regs_before.clone() };
            if instr.execute(&mut state).is_none() || state.registers != regs_after {
                candidates.get_mut(&opcode_idx).unwrap().remove(&instr.opcode);
            }
        }
    }

    for (k, v) in candidates.iter() { println!("{:2} -> {:?}", k, v); }

    let mut opcode_map = HashMap::<usize, Opcode>::new();
    while !candidates.is_empty() {
        let (&idx, opcodes) = candidates
            .iter()
            .find(|(_idx, opcodes)| opcodes.len() == 1)
            .unwrap();
        let identified_opcode = *opcodes.iter().next().unwrap();
        opcode_map.insert(idx, identified_opcode);
        for opcodes in candidates.values_mut() {
            opcodes.remove(&identified_opcode);
        }
        candidates.remove(&idx);
    }

    for (k, v) in opcode_map.iter() { println!("{:2} -> {:?}", k, v); }

    let mut state = State::new();
    for line in lines {
        Instruction::parse_with_opcode_map(&opcode_map, line).unwrap().execute(&mut state);
    }
    state.fetch(0).unwrap()
}

fn main() {
    aoc::main(part1, part2);
}
