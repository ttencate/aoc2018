use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use super::*;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Variable {
    InstructionPointer(),
    Named(String),
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Variable::InstructionPointer() => write!(f, "ip"),
            Variable::Named(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Label(usize);

impl Display for Label {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Band,
    Bor,
    Gt,
    Eq,
    LEq,
    NEq,
}

impl Operator {
    fn is_conditional(&self) -> bool {
        match self {
            Operator::Gt | Operator::Eq | Operator::LEq | Operator::NEq => true,
            _ => false,
        }
    }

    fn invert(&self) -> Operator {
        match self {
            Operator::Gt => Operator::LEq,
            Operator::Eq => Operator::NEq,
            Operator::LEq => Operator::Gt,
            Operator::NEq => Operator::Eq,
            _ => panic!("operator {} has no inverse", self),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Add => "+",
            Operator::Mul => "*",
            Operator::Band => "&",
            Operator::Bor => "|",
            Operator::Gt => ">",
            Operator::Eq => "==",
            Operator::LEq => "<=",
            Operator::NEq => "!=",
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Operand {
    Value(Value),
    Variable(Variable),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Operand::Value(val) => write!(f, "{}", val),
            Operand::Variable(var) => write!(f, "{}", var),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Expression {
    Value(Value),
    Variable(Variable),
    BinaryOp(Operand, Operator, Operand),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Expression::Value(val) => write!(f, "{}", val),
            Expression::Variable(var) => write!(f, "{}", var),
            Expression::BinaryOp(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

#[derive(Clone)]
enum Statement {
    Assignment(Variable, Expression),
    If(Expression, Box<Block>),
    Goto(Label),
    Exit(),
}

#[derive(Clone)]
struct LabelledStatement {
    idx: usize,
    label: Option<Label>,
    depth: usize,
    stat: Statement,
}

impl Display for LabelledStatement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let label = if let Some(label) = &self.label {
            format!("{:>2}:", label.to_string())
        } else {
            "".to_string()
        };
        write!(f, "{:5}", label)?;
        let indent = "    ".repeat(self.depth);
        write!(f, "{}", indent)?;
        match &self.stat {
            Statement::Assignment(var, expr) => write!(f, "{} = {};", var, expr)?,
            Statement::If(cond, block) => write!(f, "if {} {{\n{}     {}}}", cond, block, indent)?,
            Statement::Goto(label) => write!(f, "goto {};", label)?,
            Statement::Exit() => write!(f, "exit();")?,
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Block {
    statements: Vec<LabelledStatement>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}\n", statement)?;
        }
        Ok(())
    }
}

pub trait Decompile {
    fn decompile(&self) -> Block;
}

impl Decompile for Program {
    fn decompile(&self) -> Block {
        Decompiler::new(self).run()
    }
}

struct Decompiler<'a> {
    program: &'a Program,
}

impl<'a> Decompiler<'a> {
    fn new(program: &Program) -> Decompiler {
        Decompiler {
            program: program,
        }
    }

    fn run(&mut self) -> Block {
        let statements = self.program.instructions()
            .iter()
            .enumerate()
            .map(|(idx, instr)| self.instruction_to_labelled_statement(idx, instr))
            .collect();
        let mut program = Block { statements: statements };
        self.add_gotos(&mut program);
        self.strip_unused_labels(&mut program);
        self.add_ifs(&mut program);
        self.strip_unused_labels(&mut program);
        program
    }

    fn instruction_to_labelled_statement(&self, idx: usize, instr: &Instruction) -> LabelledStatement {
        let val = |val| Operand::Value(val);
        let var = |val| Operand::Variable(self.var(val));
        let ass = |lval, expr| LabelledStatement { idx: idx, label: Some(Label(idx)), depth: 0, stat: Statement::Assignment(lval, expr) };
        let bin_op = |lhs, op, rhs| Expression::BinaryOp(lhs, op, rhs);
        let a = instr.a().raw();
        let b = instr.b().raw();
        let out = self.var(instr.c().raw());
        match instr.opcode() {
            Opcode::Addr => ass(out, bin_op(var(a), Operator::Add, var(b))),
            Opcode::Addi => ass(out, bin_op(var(a), Operator::Add, val(b))),
            Opcode::Mulr => ass(out, bin_op(var(a), Operator::Mul, var(b))),
            Opcode::Muli => ass(out, bin_op(var(a), Operator::Mul, val(b))),
            Opcode::Banr => ass(out, bin_op(var(a), Operator::Band, var(b))),
            Opcode::Bani => ass(out, bin_op(var(a), Operator::Band, val(b))),
            Opcode::Borr => ass(out, bin_op(var(a), Operator::Bor, var(b))),
            Opcode::Bori => ass(out, bin_op(var(a), Operator::Bor, val(b))),
            Opcode::Setr => ass(out, Expression::Variable(self.var(a))),
            Opcode::Seti => ass(out, Expression::Value(self.val(a))),
            Opcode::Gtir => ass(out, bin_op(val(a), Operator::Gt, var(b))),
            Opcode::Gtri => ass(out, bin_op(var(a), Operator::Gt, val(b))),
            Opcode::Gtrr => ass(out, bin_op(var(a), Operator::Gt, var(b))),
            Opcode::Eqir => ass(out, bin_op(val(a), Operator::Eq, var(b))),
            Opcode::Eqri => ass(out, bin_op(var(a), Operator::Eq, val(b))),
            Opcode::Eqrr => ass(out, bin_op(var(a), Operator::Eq, var(b))),
        }
    }

    fn add_gotos(&self, program: &mut Block) {
        for labelled_statement in &mut program.statements {
            let mut goto_statement = None;
            match &labelled_statement.stat {
                Statement::Assignment(Variable::InstructionPointer(), Expression::Value(val)) => {
                    goto_statement = Some(self.goto(*val + 1));
                }
                Statement::Assignment(Variable::InstructionPointer(), Expression::BinaryOp(lhs, op, rhs)) => {
                    if *op == Operator::Add {
                        match (lhs, rhs) {
                            (Operand::Value(val), Operand::Variable(Variable::InstructionPointer())) => {
                                goto_statement = Some(self.goto(labelled_statement.idx as i32 + val + 1));
                            }
                            (Operand::Variable(Variable::InstructionPointer()), Operand::Value(val)) => {
                                goto_statement = Some(self.goto(labelled_statement.idx as i32 + val + 1));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            if let Some(goto_statement) = goto_statement {
                labelled_statement.stat = goto_statement;
            }
        }
    }

    fn find_used_labels(&self, block: &Block, used_labels: &mut HashSet<Label>) {
        for labelled_statement in &block.statements {
            match &labelled_statement.stat {
                Statement::Goto(label) => {
                    used_labels.insert(label.clone());
                }
                Statement::If(_, body) => {
                    self.find_used_labels(body, used_labels);
                }
                _ => {}
            }
        }
    }

    fn strip_unused_labels(&self, program: &mut Block) {
        let mut used_labels = HashSet::new();
        self.find_used_labels(program, &mut used_labels);
        for statement in &mut program.statements {
            if let Some(label) = &statement.label {
                if !used_labels.contains(&label) {
                    statement.label = None;
                }
            }
        }
    }

    fn add_ifs(&self, program: &mut Block) {
        let mut i = 0;
        while i < program.statements.len() - 2 {
            let fst = &program.statements[i];
            let snd = &program.statements[i + 1];
            let thd = &program.statements[i + 2];
            if let Statement::Assignment(Variable::Named(fst_var), Expression::BinaryOp(lhs, op, rhs)) = &fst.stat {
                if op.is_conditional() {
                    // TODO other order
                    if let Statement::Assignment(Variable::InstructionPointer(), Expression::BinaryOp(Operand::Variable(Variable::Named(snd_var)), Operator::Add, Operand::Variable(Variable::InstructionPointer()))) = &snd.stat {
                        if fst_var == snd_var {
                            if let Statement::Goto(_) = &thd.stat {
                                if snd.label.is_none() {
                                    program.statements[i] = LabelledStatement {
                                        idx: fst.idx,
                                        label: fst.label,
                                        depth: fst.depth,
                                        stat: Statement::If(Expression::BinaryOp(lhs.clone(), op.invert(), rhs.clone()), Box::new(Block {
                                            statements: vec![
                                                LabelledStatement {
                                                    idx: thd.idx,
                                                    label: thd.label,
                                                    depth: thd.depth + 1,
                                                    stat: thd.stat.clone(),
                                                },
                                            ],
                                        })),
                                    };
                                    program.statements.remove(i + 2);
                                    program.statements.remove(i + 1);
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn var(&self, val: Value) -> Variable {
        if Some(val as usize) == self.program.ip_register() {
            Variable::InstructionPointer()
        } else {
            Variable::Named((('a' as u8 + val as u8) as char).to_string())
        }
    }

    fn val(&self, val: Value) -> Value {
        val
    }

    fn goto(&self, idx: Value) -> Statement {
        if idx >= 0 && idx < self.program.instructions().len() as i32 {
            Statement::Goto(Label(idx as usize))
        } else {
            Statement::Exit()
        }
    }
}

#[test]
fn test_without_ip() {
    assert_eq!(Program::parse("seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5").decompile().to_string(), "     b = 5;
     c = 6;
     a = a + 1;
     d = b + c;
     a = b;
     e = 8;
     f = 9;
");
}

#[test]
fn test_with_ip() {
    assert_eq!(Program::parse("#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5").decompile().to_string(), "     b = 5;
     c = 6;
     goto 4;
     d = b + c;
 4:  ip = b;
     e = 8;
     f = 9;
");
}

#[test]
fn test_if() {
    assert_eq!(Program::parse("#ip 3
seti 123 0 1
bani 1 456 1
eqri 1 72 1
addr 1 3 3
seti 0 0 3
seti 0 0 1").decompile().to_string(), "     b = 123;
 1:  b = b & 456;
     if b != 72 {
         goto 1;
     }
     b = 0;
");
}
