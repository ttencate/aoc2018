use aoc::vm::Program;
use aoc::vm::decompiler::Decompile;
use std::io::Read;

fn main() {
    let input = std::io::stdin();
    let mut code = String::new();
    input.lock().read_to_string(&mut code).unwrap();
    print!("{}", Program::parse(&code).decompile().to_string());
}
