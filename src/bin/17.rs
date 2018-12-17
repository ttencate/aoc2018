use aoc::geom::{Matrix, Point, Rect};
use regex::Regex;

const SAND: u8 = '.' as u8;
const CLAY: u8 = '#' as u8;
const WATER_AT_REST: u8 = '~' as u8;
const WATER_FLOWING: u8 = '|' as u8;

#[allow(dead_code)]
static EXAMPLE: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

fn parse_input(input: &str) -> Matrix<u8> {
    let re = Regex::new(r"^(.)=(\d+), (.)=(\d+)\.\.(\d+)$").unwrap();
    let rects = input.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let (a_var, a_val, _b_var, b_min, b_max) = (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(5).unwrap().as_str().parse::<i32>().unwrap());
            match a_var {
                "x" => Rect::from_inclusive_ranges(a_val..=a_val, b_min..=b_max),
                "y" => Rect::from_inclusive_ranges(b_min..=b_max, a_val..=a_val),
                _ => panic!(),
            }
        })
        .collect::<Vec<Rect>>();
    let bounding_rect = rects
        .iter()
        .fold(Rect::empty(), |acc, rect| Rect::bounding_rects(&acc, rect))
        // Padding left and right with 1 column is always enough.
        .padded(1, 1, 0, 0);
    let mut mat = Matrix::new(&bounding_rect, SAND);
    for rect in rects {
        mat.fill_rect(&rect, CLAY);
    }
    mat
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input(EXAMPLE).to_string(), "............#.
.#..#.......#.
.#..#..#......
.#..#..#......
.#.....#......
.#.....#......
.#######......
..............
..............
....#.....#...
....#.....#...
....#.....#...
....#######...");
}

fn is_passable(cell: u8) -> bool {
    cell == SAND || cell == WATER_FLOWING
}

fn flow_horizontally(start: Point, step: Point, mat: &mut Matrix<u8>) -> (Point, bool) {
    let mut p = start;
    loop {
        if *mat.get(p + step).unwrap_or(&SAND) == CLAY {
            break (p, true)
        }
        if p != start && is_passable(*mat.get(p + Point::down()).unwrap_or(&SAND)) {
            flow(p + Point::down(), mat);
        }
        if is_passable(*mat.get(p + Point::down()).unwrap_or(&SAND)) {
            break (p, false)
        }
        p += step;
    }
}

fn flow(p: Point, mat: &mut Matrix<u8>) {
    if p.y < mat.rect().y_min() {
        flow(Point::new(p.x, mat.rect().y_min()), mat);
        return;
    }
    if p.y > mat.rect().y_max() {
        return;
    }
    if *mat.get(p).unwrap_or(&SAND) == WATER_FLOWING {
        // We've been here before, no need to redo it.
        return;
    }

    // First let theh recursive call do its work to figure out the situation below us.
    let below = *mat.get(p + Point::down()).unwrap_or(&SAND);
    if below == SAND {
        flow(p + Point::down(), mat);
    }

    // We either become | or ~. Scan left and right to find out which.
    let (left, contained_left) = flow_horizontally(p, Point::left(), mat);
    let (right, contained_right) = flow_horizontally(p, Point::right(), mat);
    let contained = contained_left && contained_right;

    let fill_value = if contained { WATER_AT_REST } else { WATER_FLOWING };
    for x in left.x ..= right.x {
        mat[Point::new(x, p.y)] = fill_value;
    }
}

#[test]
fn test_flow() {
    let mut mat = parse_input(EXAMPLE);
    flow(Point::new(500, 0), &mut mat);
    let expected = "......|.....#.
.#..#||||...#.
.#..#~~#|.....
.#..#~~#|.....
.#~~~~~#|.....
.#~~~~~#|.....
.#######|.....
........|.....
...|||||||||..
...|#~~~~~#|..
...|#~~~~~#|..
...|#~~~~~#|..
...|#######|..";
    assert_eq!(mat.to_string(), expected, "\nLeft:\n{}\n\nRight:\n{}", mat.to_string(), expected);
}

fn part1(input: &str) -> usize {
    let mut mat = parse_input(input);
    flow(Point::new(500, 0), &mut mat);
    mat.coords().filter(|&point| mat[point] == WATER_AT_REST || mat[point] == WATER_FLOWING).count()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 57);
}

fn part2(input: &str) -> usize {
    let mut mat = parse_input(input);
    flow(Point::new(500, 0), &mut mat);
    mat.coords().filter(|&point| mat[point] == WATER_AT_REST).count()
}

#[test]
fn part2example() {
    assert_eq!(part2(EXAMPLE), 29);
}

fn main() {
    aoc::main(part1, part2);
}
