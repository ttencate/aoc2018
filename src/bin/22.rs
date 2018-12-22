use aoc::geom::*;
use regex::Regex;

#[repr(u8)]
enum Cell {
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
            Cell::Rocky => 0,
            Cell::Wet => 1,
            Cell::Narrow => 2,
        }
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
    let rect = Rect::from_inclusive_ranges(0..=target.x, 0..=target.y);
    let mut geologic_indices = Matrix::new(&rect, 0);
    let mut erosion_levels = Matrix::new(&rect, 0);
    let mut risk_level = 0;
    for p in rect {
        let geologic_index =
            if p == Point::origin() || p == target {
                0
            } else if p.y == 0 {
                p.x as u32 * 16807
            } else if p.x == 0 {
                p.y as u32 * 48271
            } else {
                erosion_levels[p + Point::left()] * erosion_levels[p + Point::up()]
            };
        let erosion_level = (geologic_index + depth) % 20183;
        geologic_indices[p] = geologic_index;
        erosion_levels[p] = erosion_level;
        risk_level += Cell::from_erosion_level(erosion_level).risk_level();
    }
    risk_level
}

#[test]
fn part1example() {
    assert_eq!(part1("depth: 510\ntarget: 10,10\n"), 114);
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
