use std::env;
use std::fmt::Display;

pub mod geom;

mod input;

pub fn main<P1, P2, R1, R2>(part1: P1, part2: P2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Display, R2: Display
{
    let day = env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().parse::<u32>().unwrap();
    let input = input::get_input(2018, day);
    println!("Answer to day {} part 1: {}", day, part1(&input));
    println!("Answer to day {} part 2: {}", day, part2(&input));
}
