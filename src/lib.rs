use std::time::Instant;
use std::env;
use std::fmt::Display;

pub mod geom;

mod input;

pub fn main<P1, P2, R1, R2>(part1: P1, part2: P2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Display, R2: Display
{
    let day = env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().parse::<u32>().unwrap();
    let input = input::get_input(2018, day);
    run(1, part1, &input);
    run(2, part2, &input);
}

fn run<P, R>(day: u32, func: P, input: &str)
    where P: Fn(&str) -> R, R: Display
{
    let start = Instant::now();
    let output = func(input);
    let duration = start.elapsed();

    println!("Answer to day {}, part 1 ({}.{:03} s): {}", day, duration.as_secs(), duration.subsec_millis(), output);
}
