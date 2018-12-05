use std::io;
use std::io::BufRead;

fn annihilate(a: &u8, b: &u8) -> bool {
    a.eq_ignore_ascii_case(b) && a != b
}

fn reacted_length<T>(input: T) -> usize
    where T: Iterator<Item=u8>
{
    let mut stack = Vec::new();
    for byte in input.filter(u8::is_ascii_alphabetic) {
        if !stack.is_empty() && annihilate(stack.last().unwrap(), &byte) {
            stack.pop();
        } else {
            stack.push(byte);
        }
    }
    stack.len()
}

fn remove_unit<T>(input: T, removal: u8) -> impl Iterator<Item=u8>
    where T: Iterator<Item=u8>
{
    input.filter(move |c| !c.eq_ignore_ascii_case(&removal))
}

fn main() {
    let input = io::stdin();
    let polymer = input.lock().lines().next().unwrap().unwrap();

    println!("{}", reacted_length(polymer.bytes()));

    let min_length = (b'A'..(b'Z' + 1))
        .map(|removal| remove_unit(polymer.bytes(), removal))
        .map(reacted_length)
        .min()
        .unwrap();
    println!("{}", min_length);
}
