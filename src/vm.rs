use lazy_static::lazy_static;

use regex::Regex;
use std::fmt::{Display, Formatter};

pub type Value = i32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registers(Vec<Value>);

impl Registers {
    pub fn parse(line: &str) -> Option<Registers> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<registers>.*)\]").unwrap();
        }
        let vals = RE.captures(line)?
            .name("registers").unwrap().as_str()
            .split(",")
            .map(|v| v.trim().parse::<i32>().ok())
            .collect::<Vec<Option<i32>>>();
        if vals.iter().all(|val| val.is_some()) {
            Some(Registers(vals.iter().map(|val| val.unwrap()).collect()))
        } else {
            None
        }
    }
}

pub struct State {
    registers: Registers,
    ip: usize,
}

impl State {
    pub fn new(num_registers: usize) -> State {
        State { registers: Registers(vec![0; num_registers]), ip: 0 }
    }

    pub fn with_registers(registers: &Registers) -> State {
        State { registers: registers.clone(), ip: 0 }
    }

    pub fn fetch(&self, reg: Value) -> Option<Value> {
        self.registers.0.get(reg as usize).map(|v| *v)
    }

    pub fn store(&mut self, reg: Value, val: Value) -> Option<()> {
        if let Some(reg_ref) = self.registers.0.get_mut(reg as usize) {
            *reg_ref = val;
            Some(())
        } else {
            None
        }
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
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
pub enum Opcode {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
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

impl Opcode {
    pub fn all() -> &'static [Opcode] {
        ALL_OPCODES
    }
}

impl ToString for Opcode {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    opcode: Opcode,
    a: Input,
    b: Input,
    c: Output,
}

impl Instruction {
    pub fn new(opcode: Opcode, a: Value, b: Value, c: Value) -> Instruction {
        Instruction { opcode: opcode, a: Input(a), b: Input(b), c: Output(c) }
    }

    pub fn parse(line: &str) -> Option<Instruction> {
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

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn execute(&self, state: &mut State) -> Option<()> {
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

enum Directive {
    Ip(usize),
}

impl Directive {
    pub fn parse(line: &str) -> Option<Directive> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#ip (?P<register>\d+)$").unwrap();
        }
        Some(Directive::Ip(RE.captures(line)?.name("register").unwrap().as_str().parse::<usize>().ok()?))
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    ip_register: Option<usize>,
}

impl Program {
    pub fn parse(input: &str) -> Program {
        let mut instructions = Vec::new();
        let mut ip_register = None;
        for line in input.lines() {
            if let Some(instruction) = Instruction::parse(line) {
                instructions.push(instruction);
            } else if let Some(directive) = Directive::parse(line) {
                match directive {
                    Directive::Ip(register) => {
                        if ip_register.is_some() {
                            panic!("cannot have multiple #ip directives in program");
                        }
                        ip_register = Some(register);
                    }
                }
            }
        }
        Program { instructions: instructions, ip_register: ip_register }
    }

    pub fn execute(&self, state: &mut State) {
        while state.ip < self.instructions.len() {
            if let Some(ip_register) = self.ip_register {
                state.store(ip_register as Value, state.ip as Value).expect("#ip register out of range");
            }
            self.instructions[state.ip].execute(state);
            if let Some(ip_register) = self.ip_register {
                state.ip = state.fetch(ip_register as Value).expect("#ip register out of range") as usize;
            }
            state.ip += 1;
        }
    }
}
