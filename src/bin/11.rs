extern crate aoc;

use aoc::geom::{Point, Rect};

fn power_level(cell: &Point, serial_number: i32) -> i32 {
    let rack_id = cell.x + 10;
    ((((rack_id * cell.y) + serial_number) * rack_id) / 100) % 10 - 5
}

#[test]
fn test_power_level() {
    assert_eq!(power_level(&Point::new(3, 5), 8), 4);
    assert_eq!(power_level(&Point::new(122, 79), 57), -5);
    assert_eq!(power_level(&Point::new(217, 196), 39), 0);
    assert_eq!(power_level(&Point::new(101, 153), 71), 4);
}

fn part1(input: &str) -> Point {
    let serial_number = input.trim().parse::<i32>().unwrap();
    Rect::new(1, 1, 300 - 2, 300 - 2)
        .iter()
        .max_by_key(|cell| {
            Rect::new(cell.x, cell.y, 3, 3)
                .iter()
                .map(|cell| power_level(&cell, serial_number))
                .sum::<i32>()
        })
        .unwrap()
}

#[test]
fn part1example() {
    assert_eq!(part1("18\n"), Point::new(33, 45));
    assert_eq!(part1("42\n"), Point::new(21, 61));
}

fn part2(input: &str) -> String {
    // let serial_number = input.trim().parse::<i32>().unwrap();
    "TODO".to_string()
}

#[test]
fn part2example() {
    assert_eq!(part2(""), "TODO");
}

fn main() {
    aoc::main(part1, part2);
}
