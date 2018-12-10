extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::iter;
use std::ops;

#[allow(dead_code)]
static EXAMPLE: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point { x: self * rhs.x, y: self * rhs.y }
    }
}

struct Star {
    position: Point,
    velocity: Point,
}

fn parse_input(input: &str) -> Vec<Star> {
    let re = Regex::new(r"position=<\s*(-?\d+)\s*,\s*(-?\d+)\s*> velocity=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Star {
                position: Point {
                    x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                },
                velocity: Point {
                    x: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    y: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                },
            }
        })
        .collect()
}

fn simulate(stars: &Vec<Star>, time: i32) -> Vec<Point> {
    stars.iter().map(|star| star.position + time * star.velocity).collect()
}

fn render(points: &Vec<Point>) -> String {
    let x_min = points.iter().map(|p| p.x).min().unwrap();
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_min = points.iter().map(|p| p.y).min().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();
    let width = (x_max - x_min + 1) as usize;
    let height = (y_max - y_min + 1) as usize;
    let mut out: Vec<Vec<bool>> = iter::repeat(iter::repeat(false).take(width).collect()).take(height).collect();
    for point in points {
        out[(point.y - y_min) as usize][(point.x - x_min) as usize] = true;
    }
    "\n".to_string() + &out.iter().map(|row| row.iter().map(|&cell| if cell { '#' } else { '.' }).join("")).join("\n")
}

#[test]
fn test_render() {
    assert_eq!(render(&vec![Point { x: 0, y: 0 }]), "\n#");
    assert_eq!(render(&vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }]), "\n##");
    assert_eq!(render(&vec![Point { x: 0, y: 0 }, Point { x: -1, y: 0 }]), "\n##");
    assert_eq!(render(&vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }]), "\n#\n#");
    assert_eq!(render(&vec![Point { x: 3, y: 5 }, Point { x: 5, y: 7 }]), "\n#..\n...\n..#");
}

fn message_time(stars: &Vec<Star>) -> i32 {
    let mut time = 0;
    let mut prev_y_min = i32::min_value();
    loop {
        let y_min = simulate(&stars, time).iter().map(|p| p.y).min().unwrap();
        if y_min < prev_y_min {
            return time - 1;
        }
        time += 1;
        prev_y_min = y_min;
    }
}

fn part1(input: &str) -> String {
    let stars = parse_input(input);
    render(&simulate(&stars, message_time(&stars)))
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), "\n#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###");
}

fn part2(input: &str) -> i32 {
    let stars = parse_input(input);
    message_time(&stars)
}

#[test]
fn part2example() {
    assert_eq!(part2(EXAMPLE), 3);
}

fn main() {
    aoc::main(part1, part2);
}
