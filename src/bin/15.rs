use aoc::geom::*;
use itertools::Itertools;
use std::collections::VecDeque;

type Map = Matrix<u8>;

type UnitId = usize;

struct Unit {
    id: usize,
    army: u8,
    pos: Point,
    hit_points: u32,
    attack_power: u32,
    alive: bool,
}

fn is_unit(c: u8) -> bool {
    c == 'E' as u8 || c == 'G' as u8
}

fn is_enemy(a: u8, b: u8) -> bool {
    return is_unit(a) && is_unit(b) && a != b;
}

impl Unit {
    fn move_pos(&self, map: &Map) -> Option<Point> {
        if self.find_enemy_in_range(self.pos, map).is_some() {
            None
        } else if let Some(destination) = self.find_destination(map) {
            Some(self.find_first_step_towards(destination, map))
        } else {
            None
        }
    }

    fn attack_unit(&self, units: &Vec<Unit>) -> Option<UnitId> {
        let mut candidates: Vec<&Unit> = units
            .iter()
            .filter(|unit| unit.alive && is_enemy(self.army, unit.army) && self.pos.distance_to(unit.pos) == 1)
            .collect();
        candidates.sort_by_key(|unit| (unit.hit_points, unit.pos));
        candidates.first().map(|unit| unit.id)
    }

    fn find_enemy_in_range(&self, pos: Point, map: &Map) -> Option<Point> {
        pos
            .neighbors()
            .iter()
            .filter_map(|&neighbor| {
                if is_enemy(self.army, map[neighbor]) {
                    Some(neighbor)
                } else {
                    None
                }
            })
            .next()
    }

    fn find_destination(&self, map: &Map) -> Option<Point> {
        let mut visited = Matrix::new(map.rect(), false);
        let mut queue = VecDeque::new();

        visited[self.pos] = true;
        for &neighbor in self.pos.neighbors().iter() {
            queue.push_back((neighbor, 0));
        }

        while let Some((pos, dist)) = queue.pop_front() {
            if visited[pos] {
                continue;
            }
            if map[pos] != '.' as u8 {
                continue;
            }
            visited[pos] = true;
            for &neighbor in pos.neighbors().iter() {
                if is_enemy(self.army, map[neighbor]) {
                    return Some(pos);
                }
                queue.push_back((neighbor, dist + 1));
            }
        }
        None
    }

    fn find_first_step_towards(&self, destination: Point, map: &Map) -> Point {
        let mut visited_from = Matrix::new(map.rect(), None);
        let mut queue = VecDeque::new();

        for &neighbor in self.pos.neighbors().iter() {
            queue.push_back((neighbor, self.pos, 0));
        }

        while let Some((pos, from, dist)) = queue.pop_front() {
            if visited_from[pos].is_some() {
                continue;
            }
            if map[pos] != '.' as u8 {
                continue;
            }
            visited_from[pos] = Some(from);
            if pos == destination {
                let mut p = pos;
                return loop {
                    match visited_from[p] {
                        Some(f) => {
                            if f == self.pos {
                                break p;
                            } else {
                                p = f;
                            }
                        }
                        None => panic!()
                    }
                };
            }
            for &neighbor in pos.neighbors().iter() {
                queue.push_back((neighbor, pos, dist + 1));
            }
        }
        panic!();
    }
}

#[test]
fn test_unit_find_destination() {
    let state = parse_input("#######
#E..G.#
#...#.#
#.G.#G#
#######");
    assert_eq!(state.units[0].find_destination(&state.map), Some(Point::new(3, 1)));
}

#[test]
fn test_unit_move_pos() {
    let state = parse_input("#######
#E..G.#
#...#.#
#.G.#G#
#######");
    assert_eq!(state.units[0].move_pos(&state.map), Some(Point::new(2, 1)));
    let state = parse_input("#######
#.E...#
#.....#
#...G.#
#######");
    assert_eq!(state.units[0].move_pos(&state.map), Some(Point::new(3, 1)));
}

#[test]
fn test_unit_attack_unit() {
    let mut state = parse_input("G....
..G..
..EG.
..G..
...G.");
    state.units[0].hit_points = 9;
    state.units[1].hit_points = 4;
    state.units[3].hit_points = 2;
    state.units[4].hit_points = 2;
    state.units[5].hit_points = 1;
    assert_eq!(state.units[2].attack_unit(&state.units), Some(3));
    let state = parse_input("G....
..G..
..E..
..G..
...G.");
    assert_eq!(state.units[2].attack_unit(&state.units), Some(1));
}

struct State {
    map: Map,
    units: Vec<Unit>,
}

impl State {
    // Returns true if the round was fought to completion.
    fn round(&mut self) -> bool {
        for id in self.turn_order() {
            if self.is_done() {
                return false;
            }
            self.take_turn(id);
        }
        true
    }

    fn turn_order(&self) -> Vec<UnitId> {
        let mut turn_order: Vec<UnitId> = (0..self.units.len()).collect();
        turn_order.sort_by_key(|&id| self.units[id].pos);
        turn_order
    }

    fn take_turn(&mut self, id: UnitId) {
        if !self.units[id].alive {
            return;
        }
        self.perform_move(id);
        self.perform_attack(id);
    }

    fn perform_move(&mut self, id: UnitId) {
        if let Some(new_pos) = self.units[id].move_pos(&self.map) {
            let unit = &mut self.units[id];
            self.map[unit.pos] = '.' as u8;
            unit.pos = new_pos;
            self.map[unit.pos] = unit.army;
        }
    }

    fn perform_attack(&mut self, id: UnitId) {
        if let Some(attacked_id) = self.units[id].attack_unit(&self.units) {
            let attack_power = self.units[id].attack_power;
            let attacked_unit = &mut self.units[attacked_id];
            attacked_unit.hit_points = attacked_unit.hit_points.saturating_sub(attack_power);
            if attacked_unit.hit_points == 0 {
                attacked_unit.alive = false;
                self.map[attacked_unit.pos] = '.' as u8;
            }
        }
    }

    fn is_done(&self) -> bool {
        ['E' as u8, 'G' as u8]
            .iter()
            .any(|&army| {
                 self.units.iter().filter(|unit| unit.army == army && unit.alive).count() == 0
             })
    }

    fn run_until_done(&mut self) -> (u32, u32) {
        let mut completed_rounds = 0;
        loop {
            if self.round() {
                completed_rounds += 1;
            } else {
                break (completed_rounds, self.units.iter().map(|unit| unit.hit_points).sum::<u32>());
            }
        }
    }

    fn unit_at(&self, pos: Point) -> Option<&Unit> {
        if is_unit(self.map[pos]) {
            Some(self.units.iter().find(|unit| unit.alive && unit.pos == pos).unwrap())
        } else {
            None
        }
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        self.map.rect().y_range()
            .map(|y| {
                format!("{}   {}",
                    String::from_utf8_lossy(self.map.row(y)),
                    self.map.rect().x_range()
                        .filter_map(|x| {
                            self.unit_at(Point::new(x, y))
                                .map(|unit| format!("{}({})", unit.army as char, unit.hit_points))
                        })
                        .join(", "))
            })
            .join("\n")
    }
}

#[test]
fn test_state_perform_move() {
    let move_all = |state: &mut State| {
        for id in state.turn_order() {
            state.perform_move(id);
        }
    };
    let mut state = parse_input("#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########");
    move_all(&mut state);
    assert_eq!(state.map.to_string(), "#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########");
    move_all(&mut state);
    assert_eq!(state.map.to_string(), "#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########");
    move_all(&mut state);
    assert_eq!(state.map.to_string(), "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########");
    move_all(&mut state);
    assert_eq!(state.map.to_string(), "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########");
}

#[test]
fn test_state_round() {
    let mut state = parse_input("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######");
    state.round();
    assert_eq!(state.to_string(), "#######   
#..G..#   G(200)
#...EG#   E(197), G(197)
#.#G#G#   G(200), G(197)
#...#E#   E(197)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#...G.#   G(200)
#..GEG#   G(200), E(188), G(194)
#.#.#G#   G(194)
#...#E#   E(194)
#.....#   
#######   ");
    for _ in 2..23 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#...G.#   G(200)
#..G.G#   G(200), G(131)
#.#.#G#   G(131)
#...#E#   E(131)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#..G..#   G(200)
#...G.#   G(131)
#.#G#G#   G(200), G(128)
#...#E#   E(128)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#.G...#   G(200)
#..G..#   G(131)
#.#.#G#   G(125)
#..G#E#   G(200), E(125)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(122)
#...#E#   E(122)
#..G..#   G(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(119)
#...#E#   E(119)
#...G.#   G(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(116)
#...#E#   E(113)
#....G#   G(200)
#######   ");
    for _ in 28..47 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#   
#....G#   G(200)
#######   ");
}

#[test]
fn test_state_round_summarized_battle_1() {
    let mut state = parse_input("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######");
    state.round();
    assert_eq!(state.to_string(), "#######   
#G.E#E#   G(197), E(200), E(200)
#E#..E#   E(194), E(200)
#G.##.#   G(200)
#...#E#   E(200)
#..E..#   E(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(191), E(200), E(200)
#E#..E#   E(188), E(200)
#G.##.#   G(200)
#..E#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(185), E(200), E(200)
#E#..E#   E(182), E(200)
#G.##.#   G(200)
#.E.#.#   E(200)
#....E#   E(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(179), E(200), E(200)
#E#..E#   E(176), E(200)
#GE##.#   G(197), E(200)
#...#.#   
#...E.#   E(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(173), E(200), E(200)
#E#..E#   E(170), E(200)
#GE##.#   G(194), E(200)
#...#.#   
#..E..#   E(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(167), E(200), E(200)
#E#..E#   E(164), E(200)
#GE##.#   G(191), E(200)
#..E#.#   E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(161), E(200), E(200)
#E#...#   E(158)
#GE##E#   G(188), E(200), E(200)
#.E.#.#   E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(155), E(200), E(200)
#E#...#   E(152)
#GE##.#   G(182), E(200)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(149), E(200), E(200)
#E#...#   E(146)
#GE##.#   G(176), E(200)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    for _ in 9..33 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(5), E(200), E(200)
#E#...#   E(2)
#GE##.#   G(32), E(200)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GE.#E#   G(2), E(200), E(200)
#.#...#   
#GE##.#   G(26), E(197)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#.E.#E#   E(197), E(200)
#.#...#   
#GE##.#   G(20), E(194)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#E..#E#   E(197), E(200)
#.#...#   
#GE##.#   G(14), E(191)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#...#E#   E(200)
#E#...#   E(197)
#GE##.#   G(5), E(188)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    assert_eq!(state.round(), false);
    assert_eq!(state.to_string(), "#######   
#...#E#   E(200)
#E#...#   E(197)
#.E##.#   E(185)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
}

fn parse_input(input: &str) -> State {
    let map: Map = input.lines().collect();
    let mut next_id = 0;
    let units = map
        .coords()
        .filter_map(|pos| {
            let cell = map[pos];
            if is_unit(cell) {
                let unit = Unit { id: next_id, army: cell, pos: pos, hit_points: 200, attack_power: 3, alive: true };
                next_id += 1;
                Some(unit)
            } else {
                None
            }
        })
        .collect();
    State { map: map, units: units }
}

fn part1(input: &str) -> u32 {
    let mut state = parse_input(input);
    let (rounds, remaining_hit_points) = state.run_until_done();
    rounds * remaining_hit_points
}

#[test]
fn part1examples() {
    assert_eq!(part1("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"), 27730);

    let mut state = parse_input("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######");
    let (rounds, remaining_hit_points) = state.run_until_done();
    assert_eq!(state.to_string(), "#######   
#...#E#   E(200)
#E#...#   E(197)
#.E##.#   E(185)
#E..#E#   E(200), E(200)
#.....#   
#######   ");
    assert_eq!((rounds, remaining_hit_points), (37, 982));

    let mut state = parse_input("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######");
    let (rounds, remaining_hit_points) = state.run_until_done();
    assert_eq!(state.to_string(), "#######   
#.E.E.#   E(164), E(197)
#.#E..#   E(200)
#E.##.#   E(98)
#.E.#.#   E(200)
#...#.#   
#######   ");
    assert_eq!((rounds, remaining_hit_points), (46, 859));

    let mut state = parse_input("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######");
    let (rounds, remaining_hit_points) = state.run_until_done();
    assert_eq!(state.to_string(), "#######   
#G.G#.#   G(200), G(98)
#.#G..#   G(200)
#..#..#   
#...#G#   G(95)
#...G.#   G(200)
#######   ");
    assert_eq!((rounds, remaining_hit_points), (35, 793));

    let mut state = parse_input("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######");
    let (rounds, remaining_hit_points) = state.run_until_done();
    assert_eq!(state.to_string(), "#######   
#.....#   
#.#G..#   G(200)
#.###.#   
#.#.#.#   
#G.G#G#   G(98), G(38), G(200)
#######   ");
    assert_eq!((rounds, remaining_hit_points), (54, 536));

    let mut state = parse_input("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########");
    let (rounds, remaining_hit_points) = state.run_until_done();
    assert_eq!(state.to_string(), "#########   
#.G.....#   G(137)
#G.G#...#   G(200), G(200)
#.G##...#   G(200)
#...##..#   
#.G.#...#   G(200)
#.......#   
#.......#   
#########   ");
    assert_eq!((rounds, remaining_hit_points), (20, 937));
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
