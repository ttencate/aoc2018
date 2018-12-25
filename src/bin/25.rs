use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
struct Point4([i32; 4]);

impl Point4 {
    fn distance_to(&self, other: &Point4) -> u32 {
        (self.0[0] - other.0[0]).abs() as u32 +
            (self.0[1] - other.0[1]).abs() as u32 +
            (self.0[2] - other.0[2]).abs() as u32 +
            (self.0[3] - other.0[3]).abs() as u32
    }
}

#[derive(Debug)]
struct ParsePoint4Error();

impl FromStr for Point4 {
    type Err = ParsePoint4Error;
    fn from_str(s: &str) -> Result<Point4, ParsePoint4Error> {
        let mut parts = s.split(",");
        let mut p = Point4::default();
        for i in 0..4 {
            if let Some(c) = parts.next() {
                p.0[i] = c.parse::<i32>().map_err(|_| ParsePoint4Error())?;
            } else {
                return Err(ParsePoint4Error());
            }
        }
        if parts.next().is_some() {
            return Err(ParsePoint4Error());
        }
        Ok(p)
    }
}

fn parse_input(input: &str) -> Vec<Point4> {
    input.lines()
        .map(|line| {
            line.trim().parse::<Point4>().unwrap()
        })
        .collect()
}

fn connections(points: &Vec<Point4>) -> Vec<Vec<usize>> {
    let n = points.len();
    (0..n)
        .map(|i| {
            (0..n)
                .filter_map(|j| {
                    if i != j && points[i].distance_to(&points[j]) <= 3 {
                        Some(j)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn flood_fill(connections: &Vec<Vec<usize>>, start: usize, remaining: &mut HashSet<usize>) {
    if !remaining.contains(&start) {
        return;
    }
    remaining.remove(&start);
    for &n in &connections[start] {
        flood_fill(connections, n, remaining);
    }
}

fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let connections = connections(&points);
    let n = points.len();
    let mut remaining: HashSet<usize> = (0..n).collect();
    let mut num_constellations = 0;
    while let Some(start) = remaining.iter().next() {
        flood_fill(&connections, *start, &mut remaining);
        num_constellations += 1;
    }
    num_constellations
}

#[test]
fn part1examples() {
    assert_eq!(part1(" 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"), 2);
    assert_eq!(part1("-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"), 4);
    assert_eq!(part1("1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"), 3);
    assert_eq!(part1("1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"), 8);
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
