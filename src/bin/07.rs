fn part1(input: &str) -> String {
    "".to_string()
}

#[test]
fn part1example() {
    assert_eq!(part1("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."), "CABDFE");
}

fn main() {
    aoc::main!(7, part1);
}
