use std::io;
use std::io::Read;

fn annihilate(a: &u8, b: &u8) -> bool {
    a.eq_ignore_ascii_case(b) && a != b
}

fn main() {
    let input = io::stdin();
    let mut stack = Vec::new();
    for byte in input.bytes().filter_map(Result::ok) {
        if !byte.is_ascii_alphabetic() {
            continue;
        }
        if !stack.is_empty() && annihilate(stack.last().unwrap(), &byte) {
            stack.pop();
        } else {
            stack.push(byte);
        }
    }
    println!("{}", stack.len());
}
