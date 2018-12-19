use aoc::vm::*;
use std::collections::{HashMap, HashSet};

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
        Opcode::all().iter().map(|opcode| Instruction::new(*opcode, self.a, self.b, self.c)).collect()
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
                    let mut state = State::with_registers(&regs_before);
                    instr.execute(&mut state);
                    state
                })
                .filter(|state| *state.registers() == regs_after)
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
        (0..Opcode::all().len())
        .map(|idx| (idx, Opcode::all().iter().map(|opcode| *opcode).collect()))
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
            let mut state = State::with_registers(&regs_before);
            if instruction.execute(&mut state).is_none() || *state.registers() != regs_after {
                candidates.get_mut(&unidentified_instruction.opcode_idx).unwrap().remove(&instruction.opcode());
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
