use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    fn new(x: i32, y: i32, z: i32) -> Point3 {
        Point3 { x: x, y: y, z: z }
    }

    fn distance_to(&self, other: &Point3) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32 + (self.z - other.z).abs() as u32
    }
}

struct Nanobot {
    pos: Point3,
    r: u32,
}

impl Nanobot {
    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.pos.distance_to(&other.pos) <= self.r
    }
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Nanobot {
                pos: Point3::new(
                    captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                r: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let nanobots = parse_input(input);
    let strongest = nanobots.iter()
        .max_by_key(|nanobot| nanobot.r)
        .unwrap();
    nanobots.iter()
        .filter(|nanobot| strongest.is_in_range(nanobot))
        .count()
}

#[test]
fn part1example() {
    assert_eq!(part1("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
"), 7);
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
