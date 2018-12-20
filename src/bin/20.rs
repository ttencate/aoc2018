use aoc::geom::{Matrix, Point, Rect};
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
struct Room {
    neighbors: Vec<Point>,
}

#[derive(Debug, Default)]
struct Map {
    rooms: HashMap<Point, Room>,
}

impl Map {
    pub fn from_regex(regex: &str) -> Map {
        let mut map = Map::default();

        let mut pos = Point::default();
        map.get_or_create_room_mut(pos);

        let mut stack = Vec::new();

        for c in regex.chars() {
            match c {
                '(' => {
                    stack.push(pos);
                }
                '|' => {
                    // Technically, | can also occur outside parentheses.
                    pos = *stack.last().unwrap();
                }
                ')' => {
                    pos = stack.pop().unwrap();
                }
                'N' => {
                    let next = pos + Point::up();
                    map.link_rooms(pos, next);
                    pos = next;
                }
                'S' => {
                    let next = pos + Point::down();
                    map.link_rooms(pos, next);
                    pos = next;
                }
                'W' => {
                    let next = pos + Point::left();
                    map.link_rooms(pos, next);
                    pos = next;
                }
                'E' => {
                    let next = pos + Point::right();
                    map.link_rooms(pos, next);
                    pos = next;
                }
                '^' | '$' => continue,
                _ => panic!("unexpected character {}", c)
            }
        }

        assert!(stack.is_empty());

        map
    }

    pub fn get_or_create_room_mut(&mut self, pos: Point) -> &mut Room {
        self.rooms.entry(pos).or_insert(Room::default())
    }

    pub fn link_rooms(&mut self, a: Point, b: Point) {
        assert!(a.distance_to(b) == 1);
        fn link_one_way(map: &mut Map, a: Point, b: Point) {
            let room = map.get_or_create_room_mut(a);
            if room.neighbors.iter().find(|n| **n == b).is_none() {
                room.neighbors.push(b);
            }
        };
        link_one_way(self, a, b);
        link_one_way(self, b, a);
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let (x_min, x_max, y_min, y_max) =
            self.rooms.keys()
            .fold(
                (i32::max_value(), i32::min_value(), i32::max_value(), i32::min_value()),
                |(x_min, x_max, y_min, y_max), p| {
                    (min(x_min, p.x), max(x_max, p.x), min(y_min, p.y), max(y_max, p.y))
                }
            );
        let mut mat = Matrix::new(
            &Rect::from_inclusive_ranges(2 * x_min - 1 ..= 2 * x_max + 1, 2 * y_min - 1 ..= 2 * y_max + 1),
            '#' as u8);
        for (&pos, room) in self.rooms.iter() {
            let mp = 2 * pos;
            mat[mp] = if mp == Point::default() { 'X' as u8 } else { '.' as u8 };
            if room.neighbors.iter().find(|n| **n == pos + Point::right()).is_some() {
                mat[mp + Point::right()] = '|' as u8;
            }
            if room.neighbors.iter().find(|n| **n == pos + Point::down()).is_some() {
                mat[mp + Point::down()] = '-' as u8;
            }
        }
        write!(f, "{}", mat)
    }
}

#[test]
fn test_map_from_regex() {
    assert_eq!(Map::from_regex("^WNE$").to_string(), "#####
#.|.#
#-###
#.|X#
#####");
    assert_eq!(Map::from_regex("^ENWWW(NEEE|SSE(EE|N))$").to_string(), "#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########");
    assert_eq!(Map::from_regex("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").to_string(), "###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########");
    assert_eq!(Map::from_regex("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").to_string(), "#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############");
    assert_eq!(Map::from_regex("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").to_string(), "###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############");
}

fn part1(input: &str) -> usize {
    let map = Map::from_regex(input.trim());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((Point::default(), 0));
    let mut max_dist = 0;
    while let Some((pos, dist)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        max_dist = max(max_dist, dist);
        for &neighbor in &map.rooms[&pos].neighbors {
            queue.push_back((neighbor, dist + 1));
        }
    }
    max_dist
}

#[test]
fn part1example() {
    assert_eq!(part1("^WNE$"), 3);
    assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
    assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
    assert_eq!(part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), 23);
    assert_eq!(part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), 31);
}

fn part2(input: &str) -> usize {
    let map = Map::from_regex(input.trim());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((Point::default(), 0));
    let mut count = 0;
    while let Some((pos, dist)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        if dist >= 1000 {
            count += 1;
        }
        for &neighbor in &map.rooms[&pos].neighbors {
            queue.push_back((neighbor, dist + 1));
        }
    }
    count
}

fn main() {
    aoc::main(part1, part2);
}
