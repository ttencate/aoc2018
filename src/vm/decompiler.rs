use super::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Variable(String);

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Label(String);

impl Display for Label {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Operator(String);

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq)]
enum Expression {
    Value(Value),
    Variable(Variable),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Expression::Value(val) => write!(f, "{}", val),
            Expression::Variable(var) => write!(f, "{}", var),
            Expression::BinaryOp(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::Conditional(cond, tval, fval) => write!(f, "{} ? {} : {}", cond, tval, fval),
        }
    }
}

enum Statement {
    Assignment(Variable, Box<Expression>),
    Goto(Label),
    Exit(),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Statement::Assignment(var, expr) => write!(f, "{} = {};", var, expr),
            Statement::Goto(label) => write!(f, "goto {};", label),
            Statement::Exit() => write!(f, "exit();"),
        }
    }
}

struct LabelledStatement {
    idx: usize,
    label: Option<Label>,
    stat: Statement,
}

impl Display for LabelledStatement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(label) = &self.label {
            write!(f, "{}: ", label)?;
        }
        write!(f, "{}", self.stat)?;
        Ok(())
    }
}

struct Block {
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

trait Decompile {
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
        program
    }

    fn instruction_to_labelled_statement(&self, idx: usize, instr: &Instruction) -> LabelledStatement {
        fn val(val: Value) -> Box<Expression> { Box::new(Expression::Value(val)) }
        let var = |val| Box::new(Expression::Variable(self.var(val)));
        fn op(sym: &str) -> Operator { Operator(sym.to_string()) }
        let ass = |var, expr| LabelledStatement { idx: idx, label: Some(self.label(idx)), stat: Statement::Assignment(var, expr) };
        fn bin_op(lhs: Box<Expression>, op: Operator, rhs: Box<Expression>) -> Box<Expression> { Box::new(Expression::BinaryOp(lhs, op, rhs)) }
        fn cond_expr(cond: Box<Expression>, tval: Box<Expression>, fval: Box<Expression>) -> Box<Expression> { Box::new(Expression::Conditional(cond, tval, fval)) }
        let a = instr.a().raw();
        let b = instr.b().raw();
        let out = self.var(instr.c().raw());
        match instr.opcode() {
            Opcode::Addr => ass(out, bin_op(var(a), op("+"), var(b))),
            Opcode::Addi => ass(out, bin_op(var(a), op("+"), val(b))),
            Opcode::Mulr => ass(out, bin_op(var(a), op("*"), var(b))),
            Opcode::Muli => ass(out, bin_op(var(a), op("*"), val(b))),
            Opcode::Banr => ass(out, bin_op(var(a), op("&"), var(b))),
            Opcode::Bani => ass(out, bin_op(var(a), op("&"), val(b))),
            Opcode::Borr => ass(out, bin_op(var(a), op("|"), var(b))),
            Opcode::Bori => ass(out, bin_op(var(a), op("|"), val(b))),
            Opcode::Setr => ass(out, var(a)),
            Opcode::Seti => ass(out, val(a)),
            Opcode::Gtir => ass(out, cond_expr(bin_op(val(a), op(">"), var(b)), val(1), val(0))),
            Opcode::Gtri => ass(out, cond_expr(bin_op(var(a), op(">"), val(b)), val(1), val(0))),
            Opcode::Gtrr => ass(out, cond_expr(bin_op(var(a), op(">"), var(b)), val(1), val(0))),
            Opcode::Eqir => ass(out, cond_expr(bin_op(val(a), op("=="), var(b)), val(1), val(0))),
            Opcode::Eqri => ass(out, cond_expr(bin_op(var(a), op("=="), val(b)), val(1), val(0))),
            Opcode::Eqrr => ass(out, cond_expr(bin_op(var(a), op("=="), var(b)), val(1), val(0))),
        }
    }

    fn add_gotos(&self, program: &mut Block) {
        for labelled_statement in &mut program.statements {
            let mut goto_statement = None;
            match &labelled_statement.stat {
                Statement::Assignment(var, expr) => {
                    if *var == self.ip() {
                        match expr.as_ref() {
                            Expression::Value(val) => {
                                goto_statement = Some(self.goto(*val + 1));
                            }
                            Expression::BinaryOp(lhs, op, rhs) => {
                                if *op == Operator("+".to_string()) {
                                    if **lhs == Expression::Variable(self.ip()) {
                                        if let Expression::Value(val) = **rhs {
                                            goto_statement = Some(self.goto(labelled_statement.idx as i32 + val + 1));
                                        }
                                    }
                                }
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

    fn find_used_labels(&self, program: &Block) -> HashSet<Label> {
        let mut used_labels = HashSet::new();
        for labelled_statement in &program.statements {
            match &labelled_statement.stat {
                Statement::Goto(label) => {
                    used_labels.insert(label.clone());
                }
                _ => {}
            }
        }
        used_labels
    }

    fn strip_unused_labels(&self, program: &mut Block) {
        let used_labels = self.find_used_labels(program);
        for statement in &mut program.statements {
            if let Some(label) = &statement.label {
                if !used_labels.contains(&label) {
                    statement.label = None;
                }
            }
        }
    }

    fn var(&self, val: Value) -> Variable {
        if Some(val as usize) == self.program.ip_register() {
            self.ip()
        } else {
            Variable((('a' as u8 + val as u8) as char).to_string())
        }
    }

    fn ip(&self) -> Variable {
        Variable("ip".to_string())
    }

    fn label(&self, idx: usize) -> Label {
        Label(format!("_{}", idx))
    }

    fn goto(&self, idx: Value) -> Statement {
        if idx >= 0 && idx < self.program.instructions().len() as i32 {
            Statement::Goto(self.label(idx as usize))
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
seti 9 0 5").decompile().to_string(), "b = 5;
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
seti 9 0 5").decompile().to_string(), "b = 5;
c = 6;
goto _4;
d = b + c;
_4: ip = b;
e = 8;
f = 9;
");
}
