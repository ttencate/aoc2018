use aoc::geom::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use regex::Regex;

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Cell {
    Blocked = '#' as u8,
    Rocky = '.' as u8,
    Wet = '=' as u8,
    Narrow = '|' as u8,
}

impl Cell {
    fn from_erosion_level(erosion_level: u32) -> Cell {
        match erosion_level % 3 {
            0 => Cell::Rocky,
            1 => Cell::Wet,
            2 => Cell::Narrow,
            _ => panic!()
        }
    }

    fn risk_level(&self) -> u32 {
        match self {
            Cell::Blocked => panic!(),
            Cell::Rocky => 0,
            Cell::Wet => 1,
            Cell::Narrow => 2,
        }
    }

    fn can_use(&self, tool: Tool) -> bool {
        match self {
            Cell::Rocky => match tool {
                Tool::ClimbingGear | Tool::Torch => true,
                _ => false,
            },
            Cell::Wet => match tool {
                Tool::ClimbingGear | Tool::Neither => true,
                _ => false,
            },
            Cell::Narrow => match tool {
                Tool::Torch | Tool::Neither => true,
                _ => false,
            },
            Cell::Blocked => panic!()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn all() -> &'static [Tool] {
        &[Tool::Torch, Tool::ClimbingGear, Tool::Neither]
    }
}

struct Cave {
    depth: u32,
    target: Point,
    geologic_indices: HashMap<Point, u32>,
}

impl Cave {
    pub fn new(depth: u32, target: Point) -> Cave {
        Cave { depth: depth, target: target, geologic_indices: HashMap::new() }
    }

    fn get_geologic_index(&mut self, p: Point) -> u32 {
        if let Some(geologic_index) = self.geologic_indices.get(&p) {
            *geologic_index
        } else {
            let geologic_index =
                if p == Point::origin() || p == self.target {
                    0
                } else if p.y == 0 {
                    p.x as u32 * 16807
                } else if p.x == 0 {
                    p.y as u32 * 48271
                } else {
                    self.get_erosion_level(p + Point::left()) * self.get_erosion_level(p + Point::up())
                };
            self.geologic_indices.insert(p, geologic_index);
            geologic_index
        }
    }

    fn get_erosion_level(&mut self, p: Point) -> u32 {
        (self.get_geologic_index(p) + self.depth) % 20183
    }

    pub fn get_cell(&mut self, p: Point) -> Cell {
        if p.x < 0 || p.y < 0 {
            Cell::Blocked
        } else {
            Cell::from_erosion_level(self.get_erosion_level(p))
        }
    }
}

#[derive(Eq)]
struct State {
    pos: Point,
    tool: Tool,
    time: u32,
}

impl PartialEq for State {
    fn eq(&self, rhs: &State) -> bool {
        self.time == rhs.time
    }
}

impl Ord for State {
    fn cmp(&self, rhs: &State) -> Ordering {
        // Greater states are those with lower times.
        self.time.cmp(&rhs.time).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &State) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

fn parse_input(input: &str) -> (u32, Point) {
    let captures = Regex::new(r"(?s:^depth:\s*(\d+)\ntarget:\s*(\d+),(\d+)\s*$)").unwrap().captures(input).unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        Point::new(
            captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<i32>().unwrap()
        )
    )
}

fn part1(input: &str) -> u32 {
    let (depth, target) = parse_input(input);
    let mut cave = Cave::new(depth, target);
    Rect::from_inclusive_ranges(0..=target.x, 0..=target.y)
        .iter()
        .map(|p| cave.get_cell(p).risk_level())
        .sum()
}

#[test]
fn part1example() {
    assert_eq!(part1("depth: 510\ntarget: 10,10\n"), 114);
}

fn part2(input: &str) -> u32 {
    let (depth, target) = parse_input(input);
    let mut cave = Cave::new(depth, target);

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State { pos: Point::origin(), tool: Tool::Torch, time: 0 });
    while let Some(state) = queue.pop() {
        let cell = cave.get_cell(state.pos);
        if cell == Cell::Blocked {
            continue;
        }
        if !cell.can_use(state.tool) {
            continue;
        }
        if state.pos == target && state.tool == Tool::Torch {
            return state.time;
        }

        if visited.contains(&(state.pos, state.tool)) {
            continue;
        }
        visited.insert((state.pos, state.tool));

        for neighbor in &state.pos.neighbors() {
            queue.push(State { pos: *neighbor, tool: state.tool, time: state.time + 1 });
        }
        for &next_tool in Tool::all() {
            if next_tool != state.tool {
                queue.push(State { pos: state.pos, tool: next_tool, time: state.time + 7 });
            }
        }
    }
    panic!();
}

#[test]
fn part2example() {
    assert_eq!(part2("depth: 510\ntarget: 10,10\n"), 45);
}

fn main() {
    aoc::main(part1, part2);
}
