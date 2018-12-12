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

type Rules = HashMap<Vec<u8>, u8>;

struct State {
    start: i64,
    pots: Vec<u8>,
}

impl State {
    fn next(&self, rules: &Rules) -> State {
        let next_start = self.start - 2;
        let next_pots =
            (-2 .. self.pots.len() as i32 + 2)
            .map(|idx| {
                let segment: Vec<u8> =
                    (-2 ..= 2)
                    .map(|offset| {
                        *self.pots.get((idx + offset) as usize).unwrap_or(&('.' as u8))
                    })
                    .collect();
                *rules.get(&segment).unwrap_or(&('.' as u8))
            })
            .collect();
        State { start: next_start, pots: next_pots }
    }

    fn value(&self) -> i64 {
        self.pots
            .iter()
            .enumerate()
            .map(|(idx, &pot)| if pot == '#' as u8 { self.start + idx as i64 } else { 0 })
            .sum()
    }

    fn trimmed(self) -> State {
        let left_trim = self.pots
            .iter()
            .take_while(|&&pot| pot != '#' as u8)
            .count();
        let right_trim = self.pots
            .iter()
            .rev()
            .take_while(|&&pot| pot != '#' as u8)
            .count();
        State {
            start: self.start + left_trim as i64,
            pots: self.pots[left_trim .. self.pots.len() - right_trim].to_vec()
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:6} {}", self.start, String::from_utf8(self.pots.clone()).unwrap());
        Ok(())
    }
}

fn parse_input(input: &str) -> (State, Rules) {
    let mut lines = input.lines();
    let pots = lines.next().unwrap().split(":").skip(1).next().unwrap().trim().bytes().collect();
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
    (State { start: 0, pots: pots }, rules)
}

fn part1(input: &str) -> i64 {
    let (initial_state, rules) = parse_input(input);
    let mut state = initial_state;
    for _ in 0..20 {
        state = state.next(&rules);
    }
    state.value()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 325);
}

fn part2(input: &str) -> i64 {
    let end_generation = 50000000000;
    let (initial_state, rules) = parse_input(input);
    let mut state = initial_state;
    // println!("{:3}: {}", 0, state);
    let mut patterns_seen: HashMap<Vec<u8>, (i64, u64)> = HashMap::new();
    let mut generation = 0u64;
    loop {
        patterns_seen.insert(state.pots.clone(), (state.start, generation));
        state = state.next(&rules).trimmed();
        generation += 1;
        // println!("{:3}: {}", generation, state);
        if let Some((prev_start, prev_generation)) = patterns_seen.get(&state.pots) {
            // Cycle detected!
            let cycle_length = generation - prev_generation;
            let start_delta = state.start - prev_start;
            let remaining_cycles = (end_generation - generation) / cycle_length;
            generation += remaining_cycles * cycle_length;
            state.start += remaining_cycles as i64 * start_delta;
            break;
        }
    }
    while generation < end_generation {
        state = state.next(&rules).trimmed();
        generation += 1;
    }
    state.value()
}

fn main() {
    aoc::main(part1, part2);
}
