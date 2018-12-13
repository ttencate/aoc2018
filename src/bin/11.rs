use aoc::geom::{Matrix, Point, Rect};
use std::iter;

fn power_level(cell: Point, serial_number: i32) -> i32 {
    let rack_id = cell.x + 10;
    ((((rack_id * cell.y) + serial_number) * rack_id) / 100) % 10 - 5
}

#[test]
fn test_power_level() {
    assert_eq!(power_level(Point::new(3, 5), 8), 4);
    assert_eq!(power_level(Point::new(122, 79), 57), -5);
    assert_eq!(power_level(Point::new(217, 196), 39), 0);
    assert_eq!(power_level(Point::new(101, 153), 71), 4);
}

fn part1(input: &str) -> Point {
    let serial_number = input.trim().parse::<i32>().unwrap();
    Rect::from_inclusive_ranges(1 ..= 300 - 2, 1 ..= 300 - 2)
        .iter()
        .max_by_key(|cell| {
            Rect::from_inclusive_ranges(cell.x ..= cell.x + 2, cell.y ..= cell.y + 2)
                .iter()
                .map(|cell| power_level(cell, serial_number))
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
    let serial_number = input.trim().parse::<i32>().unwrap();

    // Compute cumulative sum of all cells in the rectangle from 1,1 to x,y exclusive. O(n²).
    let mut cum_sum = Matrix::new(&Rect::from_inclusive_ranges(1 ..= 301, 1 ..= 301), 0);
    for cell in Rect::from_inclusive_ranges(1 ..= 300, 1 ..= 300) {
        cum_sum[cell + Point::new(1, 1)] =
            power_level(cell, serial_number)
            + cum_sum[cell + Point::new(0, 1)]
            + cum_sum[cell + Point::new(1, 0)]
            - cum_sum[cell];
    }

    // Use cumulative sums to quickly compute the total for each possible square. O(n³). I'm not
    // sure that a more efficient approach is possible.
    let all_cells = Rect::from_inclusive_ranges(1 ..= 300, 1 ..= 300);
    let (cell, size) =
        all_cells
        .iter()
        .flat_map(|cell| { iter::repeat(cell).zip(1 ..= 300) })
        .filter(|(cell, size)| {
            all_cells.contains(*cell + Point::new(*size, *size))
        })
        .max_by_key(|(cell, size)| {
            cum_sum[*cell + Point::new(*size, *size)]
            - cum_sum[*cell + Point::new(0, *size)]
            - cum_sum[*cell + Point::new(*size, 0)]
            + cum_sum[*cell]
        })
        .unwrap();
    format!("{},{}", cell, size)
}

#[test]
fn part2example() {
    assert_eq!(part2("18\n"), "90,269,16");
    assert_eq!(part2("42\n"), "232,251,12");
}

fn main() {
    aoc::main(part1, part2);
}
