extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[allow(dead_code)]
static EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance_to(&self, other: &Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            Point::new(
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    let points = parse_input(input);
    let x_min = points.iter().map(|p| p.x).min().unwrap();
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_min = points.iter().map(|p| p.y).min().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut region_sizes = HashMap::new();
    for y in y_min ..= y_max {
        for x in x_min ..= x_max {
            let p = Point::new(x, y);
            let mut dist = i32::max_value();
            let mut closest_index = None;
            for (index, point) in points.iter().enumerate() {
                let d = p.distance_to(point);
                if d < dist {
                    dist = d;
                    closest_index = Some(index);
                } else if d == dist {
                    closest_index = None;
                }
            }
            if let Some(closest_index) = closest_index {
                if region_sizes.get(&closest_index).map(|i| *i < 0).unwrap_or(false) {
                    continue;
                }
                // If it touches the edge, it bleeds out to infinity. Mark this as -1.
                if x == x_min || x == x_max || y == y_min || y == y_max {
                    region_sizes.insert(closest_index, -1);
                    continue;
                }
                *region_sizes.entry(closest_index).or_insert(0) += 1;
            }
        }
    }
    *region_sizes.values().max().unwrap()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 17);
}

fn part2_with_threshold(input: &str, threshold: i32) -> u32 {
    let points = parse_input(input);
    let x_min = points.iter().map(|p| p.x).min().unwrap();
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_min = points.iter().map(|p| p.y).min().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut safe_region_size = 0;
    for y in y_min ..= y_max {
        for x in x_min ..= x_max {
            let p = Point::new(x, y);
            let total_distance: i32 = points.iter().map(|point| p.distance_to(point)).sum();
            if total_distance < threshold {
                // If it touches the edge, we're in trouble and need to scan a bigger region.
                assert!(x != x_min && x != x_max && y != y_min && y != y_max);
                safe_region_size += 1;
            }
        }
    }
    safe_region_size
}

fn part2(input: &str) -> u32 {
    part2_with_threshold(input, 10000)
}

#[test]
fn part2example() {
    assert_eq!(part2_with_threshold(EXAMPLE, 32), 16);
}

fn main() {
    aoc::main(6, part1, part2);
}
