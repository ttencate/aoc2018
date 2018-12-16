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
            .name("registers").unwrap().as_str()
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

    fn val(&self, _state: &State) -> Option<Value> {
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
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

const ALL_OPCODES: &[Opcode] = &[
    Opcode::Addr, Opcode::Addi,
    Opcode::Mulr, Opcode::Muli,
    Opcode::Banr, Opcode::Bani,
    Opcode::Borr, Opcode::Bori,
    Opcode::Setr, Opcode::Seti,
    Opcode::Gtir, Opcode::Gtri, Opcode::Gtrr,
    Opcode::Eqir, Opcode::Eqri, Opcode::Eqrr,
];

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    a: Input,
    b: Input,
    c: Output,
}

impl Instruction {
    fn new(opcode: Opcode, a: Value, b: Value, c: Value) -> Instruction {
        Instruction { opcode: opcode, a: Input(a), b: Input(b), c: Output(c) }
    }

    #[allow(dead_code)]
    fn parse(line: &str) -> Option<Instruction> {
        if let [opcode_str, a, b, c] = line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            let opcode = ALL_OPCODES.iter().find(|opcode| opcode.to_string() == *opcode_str)?;
            Some(Instruction::new(
                    *opcode,
                    a.parse::<Value>().ok()?,
                    b.parse::<Value>().ok()?,
                    c.parse::<Value>().ok()?))
        } else {
            None
        }
    }

    fn execute(&self, state: &mut State) -> Option<()> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        match self.opcode {
            Opcode::Addr => c.store(state, a.reg(state)? + b.reg(state)?)?,
            Opcode::Addi => c.store(state, a.reg(state)? + b.val(state)?)?,
            Opcode::Mulr => c.store(state, a.reg(state)? * b.reg(state)?)?,
            Opcode::Muli => c.store(state, a.reg(state)? * b.val(state)?)?,
            Opcode::Banr => c.store(state, a.reg(state)? & b.reg(state)?)?,
            Opcode::Bani => c.store(state, a.reg(state)? & b.val(state)?)?,
            Opcode::Borr => c.store(state, a.reg(state)? | b.reg(state)?)?,
            Opcode::Bori => c.store(state, a.reg(state)? | b.val(state)?)?,
            Opcode::Setr => c.store(state, a.reg(state)?)?,
            Opcode::Seti => c.store(state, a.val(state)?)?,
            Opcode::Gtir => c.store(state, if a.val(state)? > b.reg(state)? { 1 } else { 0 })?,
            Opcode::Gtri => c.store(state, if a.reg(state)? > b.val(state)? { 1 } else { 0 })?,
            Opcode::Gtrr => c.store(state, if a.reg(state)? > b.reg(state)? { 1 } else { 0 })?,
            Opcode::Eqir => c.store(state, if a.val(state)? == b.reg(state)? { 1 } else { 0 })?,
            Opcode::Eqri => c.store(state, if a.reg(state)? == b.val(state)? { 1 } else { 0 })?,
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

struct UnidentifiedInstruction {
    opcode_idx: usize,
    a: Value,
    b: Value,
    c: Value,
}

impl UnidentifiedInstruction {
    fn parse(line: &str) -> Option<UnidentifiedInstruction> {
        if let [Some(opcode_idx), Some(a), Some(b), Some(c)] = line.split_whitespace()
            .map(|v| v.parse::<Value>().ok())
            .collect::<Vec<Option<Value>>>()
            .as_slice()
        {
            Some(UnidentifiedInstruction { opcode_idx: *opcode_idx as usize, a: *a, b: *b, c: *c })
        } else {
            None
        }
    }

    fn with_opcode(&self, opcode: Opcode) -> Instruction {
        Instruction::new(opcode, self.a, self.b, self.c)
    }

    fn with_all_opcodes(&self) -> Vec<Instruction> {
        ALL_OPCODES.iter().map(|opcode| Instruction::new(*opcode, self.a, self.b, self.c)).collect()
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
            let instructions = UnidentifiedInstruction::parse(lines.next().unwrap()).unwrap()
                .with_all_opcodes();
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
        let unidentified_instruction = UnidentifiedInstruction::parse(lines.next().unwrap()).unwrap();
        let regs_after = Registers::parse(lines.next().unwrap()).unwrap();

        for instruction in unidentified_instruction.with_all_opcodes() {
            let mut state = State { registers: regs_before.clone() };
            if instruction.execute(&mut state).is_none() || state.registers != regs_after {
                candidates.get_mut(&unidentified_instruction.opcode_idx).unwrap().remove(&instruction.opcode);
            }
        }
    }

    let mut opcode_map = HashMap::<usize, Opcode>::new();
    while !candidates.is_empty() {
        let (&idx, opcodes) = candidates.iter().find(|(_idx, opcodes)| opcodes.len() == 1).unwrap();
        let identified_opcode = *opcodes.iter().next().unwrap();
        opcode_map.insert(idx, identified_opcode);
        for opcodes in candidates.values_mut() {
            opcodes.remove(&identified_opcode);
        }
        candidates.remove(&idx);
    }

    let mut state = State::new();
    for line in lines {
        let unidentified_instruction = UnidentifiedInstruction::parse(line).unwrap();
        let instruction = unidentified_instruction.with_opcode(opcode_map[&unidentified_instruction.opcode_idx]);
        instruction.execute(&mut state);
    }
    state.fetch(0).unwrap()
}

fn main() {
    aoc::main(part1, part2);
}
