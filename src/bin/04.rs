extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[allow(dead_code)]
static EXAMPLE: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

#[derive(Debug)]
#[derive(Default)]
struct Guard {
    id: u32,
    total_sleep_minutes: u32,
    sleep_by_minute: Vec<u32>,
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard {
            id: id,
            total_sleep_minutes: 0,
            sleep_by_minute: vec![0; 60],
        }
    }

    fn add_sleep(&mut self, start_minute: u32, end_minute: u32) {
        self.total_sleep_minutes += end_minute - start_minute;
        for minute in start_minute..end_minute {
            self.sleep_by_minute[minute as usize] += 1;
        }
    }

    fn max_minute(&self) -> u32 {
        self.sleep_by_minute.iter().enumerate().max_by_key(|(_, sleep)| *sleep).unwrap().0 as u32
    }
}

fn parse_input(input: &str) -> HashMap<u32, Guard> {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();
    let re = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard #(\d+) begins shift|falls asleep|wakes up)$").unwrap();

    let mut current_id = None;
    let mut sleep_start_minute = None;
    let mut guards = HashMap::new();

    for line in lines {
        let captures = re.captures(&line).unwrap();
        let (minute, action, id) = (
            captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(6).unwrap().as_str(),
            captures.get(7).map(|c| c.as_str().parse::<u32>().unwrap()));
        if id.is_some() {
            current_id = id;
            continue;
        }
        let current_id = current_id.unwrap();
        let guard = &mut guards.entry(current_id).or_insert(Guard::new(current_id));
        match action {
            "falls asleep" => {
                sleep_start_minute = Some(minute);
            },
            "wakes up" => {
                guard.add_sleep(sleep_start_minute.unwrap(), minute);
                sleep_start_minute = None;
            },
            _ => panic!()
        }
    }

    guards
}

fn part1(input: &str) -> u32 {
    let guards = parse_input(input);
    let guard = guards
        .values()
        .max_by_key(|guard| guard.total_sleep_minutes)
        .unwrap();
    guard.id * guard.max_minute()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 240);
}

fn part2(input: &str) -> u32 {
    let guards = parse_input(input);
    let guard = guards
        .values()
        .max_by_key(|guard| guard.sleep_by_minute[guard.max_minute() as usize])
        .unwrap();
    guard.id * guard.max_minute()
}

#[test]
fn part2example() {
    assert_eq!(part2(EXAMPLE), 4455);
}

fn main() {
    aoc::main(part1, part2);
}
