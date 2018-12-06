extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use regex::Regex;

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

fn main() {
    let input = io::stdin();
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let mut points = Vec::new();
    for line in input.lock().lines().filter_map(Result::ok) {
        let captures = re.captures(&line).unwrap();
        let point = Point::new(
            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<i32>().unwrap());
        points.push(point);
    }

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
                }
                *region_sizes.entry(closest_index).or_insert(0) += 1;
            }
        }
    }
    println!("{}", region_sizes.values().max().unwrap());

    let mut safe_region_size = 0;
    for y in y_min ..= y_max {
        for x in x_min ..= x_max {
            let p = Point::new(x, y);
            let total_distance: i32 = points.iter().map(|point| p.distance_to(point)).sum();
            if total_distance < 10000 {
                // If it touches the edge, we're in trouble and need to scan a bigger region.
                assert!(x != x_min && x != x_max && y != y_min && y != y_max);
                safe_region_size += 1;
            }
        }
    }
    println!("{}", safe_region_size);
}
