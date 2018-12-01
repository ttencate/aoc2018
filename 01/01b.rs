use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

fn main() {
    let input = io::stdin();
    let freqs = Vec::from_iter(
        input.lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap()));
    let mut sum: i32 = 0;
    let mut seen = HashSet::new();
    loop {
        for freq in &freqs {
            seen.insert(sum);
            sum += freq;
            if seen.contains(&sum) {
                println!("{}", sum);
                return;
            }
        }
    }
}
