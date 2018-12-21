use super::*;
use std::fmt::{Display, Formatter};

struct Variable(String);

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Operator(String);

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Statement::Assignment(var, expr) => {
                write!(f, "{} = {};", var, expr)?;
            }
        }
        Ok(())
    }
}

struct Block {
    statements: Vec<Statement>,
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
            .map(|instr| self.instruction_to_statement(instr))
            .collect();
        Block { statements: statements }
    }

    fn instruction_to_statement(&self, instr: &Instruction) -> Statement {
        fn val(val: Value) -> Box<Expression> { Box::new(Expression::Value(val)) }
        let var = |val| { Box::new(Expression::Variable(self.var(val))) };
        fn op(sym: &str) -> Operator { Operator(sym.to_string()) }
        fn ass(var: Variable, expr: Box<Expression>) -> Statement { Statement::Assignment(var, expr) }
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

    fn var(&self, val: Value) -> Variable {
        Variable(if Some(val as usize) == self.program.ip_register() {
            "ip".to_string()
        } else {
            (('a' as u8 + val as u8) as char).to_string()
        })
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
ip = ip + 1;
d = b + c;
ip = b;
e = 8;
f = 9;
");
}
