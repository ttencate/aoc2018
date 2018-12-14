use itertools::Itertools;
use std::collections::VecDeque;

struct State {
    scoreboard: Vec<u8>,
    elves: Vec<usize>,
}

impl State {
    fn new() -> State {
        State {
            scoreboard: vec![3, 7],
            elves: vec![0, 1],
        }
    }

    fn step(&mut self) {
        let score_sum = self.elves.iter().map(|&idx| self.scoreboard[idx]).sum();
        if score_sum <= 9 {
            self.scoreboard.push(score_sum);
        } else {
            self.scoreboard.push(score_sum / 10);
            self.scoreboard.push(score_sum % 10);
        }

        for i in 0..self.elves.len() {
            self.elves[i] = (self.elves[i] + 1 + self.scoreboard[self.elves[i]] as usize) % self.scoreboard.len();
        }
    }

    fn substr(&self, start: usize, length: usize) -> String {
        self.scoreboard.iter().skip(start).take(length).map(u8::to_string).join("")
    }
}

fn part1(input: &str) -> String {
    let num_recipes_to_make = input.trim().parse::<usize>().unwrap();
    let mut state = State::new();
    while state.scoreboard.len() < num_recipes_to_make + 10 {
        state.step();
    }
    state.substr(num_recipes_to_make, 10)
}

#[test]
fn part1examples() {
    assert_eq!(part1("9"), "5158916779");
    assert_eq!(part1("5"), "0124515891");
    assert_eq!(part1("18"), "9251071085");
    assert_eq!(part1("2018"), "5941429882");
}

fn part2(input: &str) -> usize {
    let search_string: VecDeque<u8> = input.trim().chars().map(|c| c as u8 - '0' as u8).collect();

    let mut state = State::new();
    let mut start_idx = 0;
    let mut substring = VecDeque::new();
    loop {
        if state.scoreboard.len() < start_idx + search_string.len() {
            state.step();
        } else {
            while substring.len() < search_string.len() {
                substring.push_back(state.scoreboard[start_idx + substring.len()]);
            }
            if substring == search_string {
                break start_idx;
            }
            substring.pop_front();
            start_idx += 1;
        }
    }
}

#[test]
fn part2examples() {
    assert_eq!(part2("51589"), 9);
    assert_eq!(part2("01245"), 5);
    assert_eq!(part2("92510"), 18);
    assert_eq!(part2("59414"), 2018);
}

fn main() {
    aoc::main(part1, part2);
}
