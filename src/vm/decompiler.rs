use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Variable {
    InstructionPointer(),
    Named(char),
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

    fn is_commutative(&self) -> bool {
        match self {
            Operator::Add | Operator::Mul | Operator::Band | Operator::Bor | Operator::Eq | Operator::NEq => true,
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
    OpAssignment(Variable, Operator, Operand),
    IfElse(Expression, Box<Block>, Box<Block>),
    DoWhile(Box<Block>, Expression),
    Goto(Label),
    ConditionalGoto(Expression, Label),
    Exit(),
    NoOp(),
}

#[derive(Clone)]
struct LabelledStatement {
    idx: usize,
    label: Option<Label>,
    stat: Statement,
}

#[derive(Clone)]
pub struct Block {
    depth: usize, // TODO reconstruct while printing
    statements: Vec<LabelledStatement>,
}

impl Block {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn find_label_index(&self, label: &Label) -> Option<usize> {
        (0..self.statements.len()).find(|&i| self.statements[i].label == Some(*label))
    }

    fn extract_range(&self, range: Range<usize>) -> Box<Block> {
        let mut extracted = Block {
            depth: self.depth,
            statements: range.map(|i| self.statements[i].clone()).collect(),
        };
        extracted.increase_depth();
        Box::new(extracted)
    }

    fn increase_depth(&mut self) {
        self.depth += 1;
        for statement in &mut self.statements {
            match &mut statement.stat {
                Statement::IfElse(_, tbody, fbody) => {
                    tbody.increase_depth();
                    fbody.increase_depth();
                }
                Statement::DoWhile(body, _) => body.increase_depth(),
                _ => {}
            }
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let indent = "    ".repeat(self.depth);
        for labelled_statement in &self.statements {
            let label = if let Some(label) = &labelled_statement.label {
                format!("{:>2}:", label.to_string())
            } else {
                "".to_string()
            };
            write!(f, "{:5}", label)?;
            write!(f, "{}", indent)?;
            match &labelled_statement.stat {
                Statement::Assignment(var, expr) => write!(f, "{} = {};", var, expr)?,
                Statement::OpAssignment(var, op, oper) => write!(f, "{} {}= {};", var, op, oper)?,
                Statement::IfElse(cond, tbody, fbody) => if fbody.is_empty() {
                    write!(f, "if {} {{\n{}     {}}}", cond, tbody, indent)?
                } else {
                    write!(f, "if {} {{\n{}     {}}} else {{\n{}     {}}}", cond, tbody, indent, fbody, indent)?
                },
                Statement::DoWhile(body, cond) => write!(f, "do {{\n{}     {}}} while {};", body, indent, cond)?,
                Statement::Goto(label) => write!(f, "goto {};", label)?,
                Statement::ConditionalGoto(cond, label) => write!(f, "if {} {{ goto {}; }}", cond, label)?,
                Statement::Exit() => write!(f, "exit();")?,
                Statement::NoOp() => {}
            }
            write!(f, "\n")?;
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
        let mut program = Block { depth: 0, statements: statements };
        self.add_op_assignments(&mut program);
        self.add_gotos(&mut program);
        self.strip_unused_labels(&mut program); // Labels may stop CondGotos from being detected.
        self.add_conditional_gotos(&mut program);
        self.add_if_elses(&mut program);
        self.add_do_whiles(&mut program);
        self.strip_unused_labels(&mut program);
        self.remove_noops(&mut program);
        program
    }

    fn instruction_to_labelled_statement(&self, idx: usize, instr: &Instruction) -> LabelledStatement {
        let val = |val| Operand::Value(val);
        let var = |val| Operand::Variable(self.var(val));
        let ass = |lval, expr| LabelledStatement { idx: idx, label: Some(Label(idx)), stat: Statement::Assignment(lval, expr) };
        let a = instr.a().raw();
        let b = instr.b().raw();
        let out = self.var(instr.c().raw());
        match instr.opcode() {
            Opcode::Addr => ass(out, Expression::BinaryOp(var(a), Operator::Add, var(b))),
            Opcode::Addi => ass(out, Expression::BinaryOp(var(a), Operator::Add, val(b))),
            Opcode::Mulr => ass(out, Expression::BinaryOp(var(a), Operator::Mul, var(b))),
            Opcode::Muli => ass(out, Expression::BinaryOp(var(a), Operator::Mul, val(b))),
            Opcode::Banr => ass(out, Expression::BinaryOp(var(a), Operator::Band, var(b))),
            Opcode::Bani => ass(out, Expression::BinaryOp(var(a), Operator::Band, val(b))),
            Opcode::Borr => ass(out, Expression::BinaryOp(var(a), Operator::Bor, var(b))),
            Opcode::Bori => ass(out, Expression::BinaryOp(var(a), Operator::Bor, val(b))),
            Opcode::Setr => ass(out, Expression::Variable(self.var(a))),
            Opcode::Seti => ass(out, Expression::Value(self.val(a))),
            Opcode::Gtir => ass(out, Expression::BinaryOp(val(a), Operator::Gt, var(b))),
            Opcode::Gtri => ass(out, Expression::BinaryOp(var(a), Operator::Gt, val(b))),
            Opcode::Gtrr => ass(out, Expression::BinaryOp(var(a), Operator::Gt, var(b))),
            Opcode::Eqir => ass(out, Expression::BinaryOp(val(a), Operator::Eq, var(b))),
            Opcode::Eqri => ass(out, Expression::BinaryOp(var(a), Operator::Eq, val(b))),
            Opcode::Eqrr => ass(out, Expression::BinaryOp(var(a), Operator::Eq, var(b))),
        }
    }

    fn add_op_assignments(&self, program: &mut Block) {
        for labelled_statement in &mut program.statements {
            if let Statement::Assignment(var, Expression::BinaryOp(lhs, op, rhs)) = &labelled_statement.stat {
                if !op.is_conditional() {
                    if Operand::Variable(*var) == *lhs {
                        labelled_statement.stat = Statement::OpAssignment(*var, *op, rhs.clone());
                    } else if Operand::Variable(*var) == *rhs && op.is_commutative() {
                        labelled_statement.stat = Statement::OpAssignment(*var, *op, lhs.clone());
                    }
                }
            }
        }
    }

    fn add_gotos(&self, program: &mut Block) {
        for labelled_statement in &mut program.statements {
            let mut goto_statement = None;
            match &labelled_statement.stat {
                Statement::Assignment(Variable::InstructionPointer(), Expression::Value(val)) => {
                    goto_statement = Some(self.goto(*val + 1));
                }
                Statement::OpAssignment(Variable::InstructionPointer(), Operator::Add, Operand::Value(val)) => {
                    goto_statement = Some(self.goto(labelled_statement.idx as i32 + val + 1));
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
                Statement::ConditionalGoto(_, label) => {
                    used_labels.insert(label.clone());
                }
                Statement::IfElse(_, tbody, fbody) => {
                    self.find_used_labels(tbody, used_labels);
                    self.find_used_labels(fbody, used_labels);
                }
                Statement::DoWhile(body, _) => {
                    self.find_used_labels(body, used_labels);
                }
                _ => {}
            }
        }
    }

    fn strip_unused_labels(&self, program: &mut Block) {
        let mut used_labels = HashSet::new();
        self.find_used_labels(program, &mut used_labels);
        self.strip_labels_outside_set(program, &used_labels);
    }

    fn strip_labels_outside_set(&self, block: &mut Block, used_labels: &HashSet<Label>) {
        for statement in &mut block.statements {
            if let Some(label) = &statement.label {
                if !used_labels.contains(&label) {
                    statement.label = None;
                }
            }
            match &mut statement.stat {
                Statement::IfElse(_, tbody, fbody) => {
                    self.strip_labels_outside_set(tbody, used_labels);
                    self.strip_labels_outside_set(fbody, used_labels);
                }
                Statement::DoWhile(body, _) => {
                    self.strip_labels_outside_set(body, used_labels);
                }
                _ => {}
            }
        }
    }

    fn add_conditional_gotos(&self, block: &mut Block) {
        let mut i = 0;
        while i < block.statements.len() - 2 {
            let fst = &block.statements[i];
            let snd = &block.statements[i + 1];
            let thd = &block.statements[i + 2];
            if let Statement::Assignment(Variable::Named(fst_var), Expression::BinaryOp(lhs, op, rhs)) = &fst.stat {
                if op.is_conditional() {
                    if let Statement::OpAssignment(Variable::InstructionPointer(), Operator::Add, Operand::Variable(Variable::Named(snd_var))) = &snd.stat {
                        if fst_var == snd_var {
                            if let Statement::Goto(target_label) = &thd.stat {
                                if snd.label.is_none() {
                                    block.statements[i] = LabelledStatement {
                                        idx: fst.idx,
                                        label: fst.label,
                                        stat: Statement::ConditionalGoto(
                                            Expression::BinaryOp(lhs.clone(), op.invert(), rhs.clone()),
                                            *target_label),
                                    };
                                    block.statements.remove(i + 2);
                                    block.statements.remove(i + 1);
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn add_if_elses(&self, block: &mut Block) {
        let mut i = 0;
        while i < block.statements.len() {
            match &block.statements[i].stat {
                Statement::ConditionalGoto(cond, label) => {
                    if let Some(target_idx) = block.find_label_index(label) {
                        if target_idx > i {
                            let mut cond = cond.clone();
                            let mut fbody = block.extract_range(i + 1 .. target_idx);
                            let mut tbody = Box::new(Block { depth: block.depth + 1, statements: vec![] });
                            let mut end_extract = target_idx;
                            if let Some(last_statement) = &fbody.statements.last() {
                                if let Statement::Goto(label) = last_statement.stat {
                                    if let Some(if_target_idx) = block.find_label_index(&label) {
                                        if if_target_idx > target_idx {
                                            fbody.statements.pop();
                                            tbody = block.extract_range(target_idx .. if_target_idx);
                                            end_extract = if_target_idx;
                                        }
                                    }
                                }
                            }
                            if tbody.is_empty() {
                                if let Expression::BinaryOp(_, op, _) = &mut cond {
                                    std::mem::swap(&mut fbody, &mut tbody);
                                    *op = op.invert();
                                }
                            }

                            block.statements[i] = LabelledStatement {
                                idx: block.statements[i].idx,
                                label: block.statements[i].label,
                                stat: Statement::IfElse(cond, tbody, fbody),
                            };
                            block.statements.splice(i + 1 .. end_extract, std::iter::empty());
                        }
                    }
                }
                _ => {}
            }
            match &mut block.statements[i].stat {
                Statement::IfElse(_, tbody, fbody) => {
                    self.add_if_elses(tbody.as_mut());
                    self.add_if_elses(fbody.as_mut());
                }
                Statement::DoWhile(body, _) => {
                    self.add_if_elses(body.as_mut());
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn add_do_whiles(&self, block: &mut Block) {
        let mut i = 0;
        while i < block.statements.len() {
            match &block.statements[i].stat {
                Statement::ConditionalGoto(cond, label) => {
                    if let Some(target_idx) = block.find_label_index(label) {
                        if target_idx <= i {
                            let mut do_while_body = block.extract_range(target_idx..i);
                            if block.statements[i].label.is_some() {
                                do_while_body.statements.push(LabelledStatement {
                                    idx: block.statements[i].idx,
                                    label: block.statements[i].label,
                                    stat: Statement::NoOp(),
                                });
                            }
                            let do_while_statement = LabelledStatement {
                                idx: block.statements[target_idx].idx,
                                label: block.statements[target_idx].label,
                                stat: Statement::DoWhile(do_while_body, cond.clone()),
                            };
                            block.statements.splice(target_idx..=i, std::iter::once(do_while_statement));
                            i = target_idx;
                        }
                    }
                }
                _ => {}
            }
            match &mut block.statements[i].stat {
                Statement::IfElse(_, tbody, fbody) => {
                    self.add_do_whiles(tbody.as_mut());
                    self.add_do_whiles(fbody.as_mut());
                }
                Statement::DoWhile(body, _) => {
                    self.add_do_whiles(body.as_mut());
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn remove_noops(&self, block: &mut Block) {
        let mut i = 0;
        while i < block.statements.len() {
            match block.statements[i].stat {
                Statement::NoOp() => {
                    if block.statements[i].label.is_none() {
                        block.statements.remove(i);
                        continue;
                    }
                }
                _ => {}
            }
            match &mut block.statements[i].stat {
                Statement::IfElse(_, tbody, fbody) => {
                    self.remove_noops(tbody.as_mut());
                    self.remove_noops(fbody.as_mut());
                }
                Statement::DoWhile(body, _) => {
                    self.remove_noops(body.as_mut());
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn var(&self, val: Value) -> Variable {
        if Some(val as usize) == self.program.ip_register() {
            Variable::InstructionPointer()
        } else {
            Variable::Named(('a' as u8 + val as u8) as char)
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
     a += 1;
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
seti 5 0 3
seti 0 0 1
seti 42 0 1").decompile().to_string(), "     b = 123;
     b &= 456;
     if b == 72 {
         b = 0;
     }
     b = 42;
");
}

#[test]
fn test_do_while() {
    assert_eq!(Program::parse("#ip 3
seti 123 0 1
bani 1 456 1
eqri 1 72 1
addr 1 3 3
seti 0 0 3
seti 0 0 1").decompile().to_string(), "     b = 123;
     do {
         b &= 456;
     } while b != 72;
     b = 0;
");
}
