use std::collections::HashSet;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
}

#[test]
fn part1examples() {
    assert_eq!(part1("+1\n-2\n+3\n+1"), 3);
    assert_eq!(part1("+1\n+1\n+1"), 3);
    assert_eq!(part1("+1\n+1\n-2"), 0);
    assert_eq!(part1("-1\n-2\n-3"), -6);
}

fn part2(input: &str) -> i32 {
    let freqs: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut sum: i32 = 0;
    let mut seen = HashSet::new();
    loop {
        for freq in &freqs {
            seen.insert(sum);
            sum += freq;
            if seen.contains(&sum) {
                return sum;
            }
        }
    }
}

#[test]
fn part2example() {
    assert_eq!(part2("+1\n-2\n+3\n+1"), 2);
    assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
    assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
    assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
}

fn main() {
    aoc::main(1, part1, part2);
}
