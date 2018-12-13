use aoc::geom::{Direction, Matrix, Point, Turn};
use std::collections::HashMap;
use std::cmp::Ordering;

static TURNS: &[Turn] = &[
    Turn::Left,
    Turn::Straight,
    Turn::Right,
];

type Map = Matrix<u8>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cart {
    pos: Point,
    direction: Direction,
    next_turn: usize,
}

impl Cart {
    fn next(&mut self, map: &Map) {
        self.pos += self.direction.as_point();
        self.direction = match map[self.pos] as char {
            '/' => match self.direction {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            }
            '\\' => match self.direction {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            }
            '+' => {
                let direction = self.direction + TURNS[self.next_turn];
                self.next_turn = (self.next_turn + 1) % TURNS.len();
                direction
            },
            _ => self.direction,
        }
    }

    fn collides_with(&self, other: &Cart) -> bool {
        self.pos == other.pos && self.direction != other.direction
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Cart {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.pos.y.cmp(&rhs.pos.y)
            .then(self.pos.x.cmp(&rhs.pos.x))
    }
}

fn parse_input(input: &str) -> (Map, Vec<Cart>) {
    let mut map: Map = input.lines().collect();
    let mut carts = vec![];
    for pos in map.coords() {
        let (direction, track) = match map[pos] as char {
            '<' => (Direction::Left, '-'),
            '>' => (Direction::Right, '-'),
            '^' => (Direction::Up, '|'),
            'v' => (Direction::Down, '|'),
            _ => continue
        };
        map[pos] = track as u8;
        carts.push(Cart { pos: pos, direction: direction, next_turn: 0 });
    }
    (map, carts)
}

fn part1(input: &str) -> Point {
    let (map, mut carts) = parse_input(input);
    // We want to be able to borrow a single cart mutably, but also iterate immutably over the
    // vector at the same time.
    let mut carts: Vec<&mut Cart> = carts.iter_mut().collect();
    for _ in 0.. {
        carts.sort();
        for i in 0..carts.len() {
            carts[i].next(&map);
            let cart = &carts[i];
            // Need to check after each individual movement to prevent carts phasing through each
            // other.
            if carts.iter().any(|c| c.collides_with(cart)) {
                return cart.pos;
            }
        }
    }
    panic!();
}

#[test]
fn part1examples() {
    assert_eq!(part1("|\nv\n|\n|\n|\n^\n|"), Point::new(0, 3));
    assert_eq!(part1(r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
"), Point::new(7, 3));
}

fn part2(input: &str) -> Point {
    let (map, mut carts) = parse_input(input);
    // We want to be able to borrow a single cart mutably, but also iterate immutably over the
    // vector at the same time.
    let mut carts: Vec<&mut Cart> = carts.iter_mut().collect();
    for _ in 0.. {
        carts.sort();
        let mut i = 0;
        while i < carts.len() {
            carts[i].next(&map);
            let mut indices = HashMap::new();
            for (index, cart) in carts.iter().enumerate() {
                indices.entry(cart.pos).or_insert(vec![]).push(index);
            }
            let mut removed_indices: Vec<usize> = indices
                .values()
                .filter(|is| is.len() > 1)
                .flat_map(Clone::clone)
                .collect();
            removed_indices.sort_unstable();
            for &index in removed_indices.iter().rev() {
                carts.remove(index);
                if i >= index {
                    i = i.wrapping_sub(1);
                }
            }
            i = i.wrapping_add(1);
        }
        if carts.len() == 1 {
            return carts[0].pos;
        }
    }
    panic!();
}

#[test]
fn part2example() {
    assert_eq!(part2(r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"), Point::new(6, 4));
}

fn main() {
    aoc::main(part1, part2);
}
