use std::collections::HashMap;

#[allow(dead_code)]
static EXAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

type State = (i32, Vec<u8>);
type Rules = HashMap<Vec<u8>, u8>;

fn parse_input(input: &str) -> (State, Rules) {
    let mut lines = input.lines();
    let initial_state = lines.next().unwrap().split(":").skip(1).next().unwrap().trim().bytes().collect();
    lines.next().unwrap();
    let rules =
        lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap().bytes().collect();
            parts.next().unwrap();
            let result = parts.next().unwrap().bytes().next().unwrap();
            (pattern, result)
        })
        .collect();
    ((0, initial_state), rules)
}

fn next_state(state: &State, rules: &Rules) -> State {
    let (start, pots) = state;
    let next_start = start - 2;
    let next_pots =
        (-2 .. state.1.len() as i32 + 2)
        .map(|idx| {
            let segment: Vec<u8> =
                (-2 ..= 2)
                .map(|offset| {
                    *pots.get((idx + offset) as usize).unwrap_or(&('.' as u8))
                })
                .collect();
            *rules.get(&segment).unwrap_or(&('.' as u8))
        })
        .collect();
    (next_start, next_pots)
}

fn part1(input: &str) -> i32 {
    let (initial_state, rules) = parse_input(input);
    let mut state = initial_state;
    for _ in 0..20 {
        state = next_state(&state, &rules);
    }

    state.1
        .iter()
        .enumerate()
        .map(|(idx, &pot)| if pot == '#' as u8 { state.0 + idx as i32 } else { 0 })
        .sum()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 325);
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

#[test]
fn part2example() {
    assert_eq!(part2(""), "TODO");
}

fn main() {
    aoc::main(part1, part2);
}
