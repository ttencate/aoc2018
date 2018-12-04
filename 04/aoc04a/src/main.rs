extern crate regex;

use std::collections::HashMap;
use regex::Regex;
use std::io;
use std::io::BufRead;
use std::iter;
use std::iter::FromIterator;

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
            sleep_by_minute: Vec::from_iter(iter::repeat(0).take(60)),
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

fn main() {
    let input = io::stdin();
    let mut lines = Vec::from_iter(input.lock().lines().filter_map(Result::ok));
    lines.sort_unstable();
    let re = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard #(\d+) begins shift|falls asleep|wakes up)$").unwrap();

    let mut current_id = None;
    let mut sleep_start_minute = None;
    let mut guards = HashMap::new();

    for line in lines {
        let captures = re.captures(&line).unwrap();
        let (_year, _month, _day, _hour, minute, action, id) = (
            captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
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

    let guard = guards.values().max_by_key(|guard| guard.total_sleep_minutes).unwrap();
    println!("{}", guard.id * guard.max_minute());
}
