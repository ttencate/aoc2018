use aoc::geom::*;
use itertools::Itertools;
use std::collections::VecDeque;

type Map = Matrix<u8>;

type UnitId = usize;

type Army = u8;
const ELVES: Army = 'E' as Army;
const GOBLINS: Army = 'G' as Army;

#[derive(Clone, Debug)]
struct Unit {
    id: usize,
    army: u8,
    pos: Point,
    hit_points: u32,
    attack_power: u32,
    alive: bool,
}

fn is_unit(c: u8) -> bool {
    c == ELVES || c == GOBLINS
}

fn is_enemy(a: u8, b: u8) -> bool {
    return is_unit(a) && is_unit(b) && a != b;
}

impl Unit {
    fn move_pos(&self, map: &Map) -> Option<Point> {
        if self.find_enemy_in_range(self.pos, map).is_some() {
            None
        } else {
            self.find_first_step(map)
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
        pos.neighbors()
            .iter()
            .find(|&&neighbor| is_enemy(self.army, map[neighbor]))
            .map(|neighbor| *neighbor)
    }

    fn find_first_step(&self, map: &Map) -> Option<Point> {
        let mut visited = Matrix::new(map.rect(), false);
        let mut queue = VecDeque::new();

        visited[self.pos] = true;
        for &neighbor in self.pos.neighbors().iter() {
            queue.push_back((neighbor, neighbor, 0));
        }

        while let Some((pos, first_step, dist)) = queue.pop_front() {
            if visited[pos] {
                continue;
            }
            if map[pos] != '.' as u8 {
                continue;
            }
            visited[pos] = true;
            if self.find_enemy_in_range(pos, map).is_some() {
                return Some(first_step);
            }
            for &neighbor in pos.neighbors().iter() {
                queue.push_back((neighbor, first_step, dist + 1));
            }
        }
        None
    }
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

#[derive(Clone)]
struct State {
    map: Map,
    units: Vec<Unit>,
    rounds_completed: u32,
}

impl State {
    // Returns true if the round was fought to completion.
    fn round(&mut self) -> bool {
        for id in self.turn_order() {
            if !self.take_turn(id) {
                return false;
            }
        }
        self.rounds_completed += 1;
        true
    }

    fn turn_order(&self) -> Vec<UnitId> {
        (0..self.units.len()).sorted_by_key(|&id| self.units[id].pos).collect()
    }

    fn take_turn(&mut self, id: UnitId) -> bool {
        if !self.units[id].alive {
            return true;
        }
        if self.loser().is_some() {
            return false;
        }
        self.perform_move(id);
        self.perform_attack(id);
        return true;
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

    fn loser(&self) -> Option<u8> {
        [ELVES, GOBLINS]
            .iter()
            .find(|&&army| {
                self.units.iter().filter(|unit| unit.army == army && unit.alive).count() == 0
            })
            .map(|army| *army)
    }

    fn run_until_done(&mut self) -> (u32, u32) {
        loop {
            if !self.round() {
                break (self.rounds_completed, self.units.iter().map(|unit| unit.hit_points).sum::<u32>());
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
fn test_state_round_example_1() {
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
    State { map: map, units: units, rounds_completed: 0 }
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

fn upgrade_army(state: &State, army: u8, attack_power: u32) -> State {
    let mut state = state.clone();
    for unit in state.units.iter_mut() {
        if unit.army == army {
            unit.attack_power = attack_power;
        }
    }
    state
}

fn part2(input: &str) -> u32 {
    let start_state = parse_input(input);
    let mut elf_attack_power = 3;
    loop {
        let mut state = upgrade_army(&start_state, ELVES, elf_attack_power);
        let (rounds, remaining_hit_points) = state.run_until_done();
        if state.units.iter().filter(|unit| unit.army == ELVES).all(|elf| elf.alive) {
            break rounds * remaining_hit_points
        }
        elf_attack_power += 1;
    }
}

#[test]
fn test_state_round_example_3_upgraded() {
    let mut state = upgrade_army(&parse_input("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"), ELVES, 15);
    state.round();
    assert_eq!(state.to_string(), "#######   
#.EG#.#   E(197), G(185)
#G#G..#   G(200), G(200)
#..#..#   
#.G.#G#   G(200), G(185)
#....E#   E(200)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(200), E(191), G(170)
#.#G..#   G(200)
#..#..#   
#..G#G#   G(200), G(170)
#....E#   E(197)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(200), E(185), G(155)
#.#G..#   G(200)
#..#..#   
#...#G#   G(155)
#..G.E#   G(200), E(194)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(200), E(179), G(140)
#.#G..#   G(200)
#..#..#   
#...#G#   G(140)
#...GE#   G(200), E(188)
#######   ");
    for _ in 4..13 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(200), E(125), G(5)
#.#G..#   G(200)
#..#..#   
#...#G#   G(5)
#...GE#   G(200), E(134)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(200), E(119), G(200)
#.#...#   
#..#..#   
#...#.#   
#...GE#   G(200), E(128)
#######   ");
    for _ in 14..27 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#GEG#.#   G(5), E(41), G(200)
#.#...#   
#..#..#   
#...#.#   
#...GE#   G(5), E(89)
#######   ");
    state.round();
    assert_eq!(state.to_string(), "#######   
#.EG#.#   E(35), G(200)
#.#...#   
#..#..#   
#...#.#   
#....E#   E(86)
#######   ");
    for _ in 28..33 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#.EG#.#   E(20), G(110)
#.#E..#   E(86)
#..#..#   
#...#.#   
#.....#   
#######   ");
    for _ in 33..36 {
        state.round();
    }
    assert_eq!(state.to_string(), "#######   
#.EG#.#   E(11), G(20)
#.#E..#   E(86)
#..#..#   
#...#.#   
#.....#   
#######   ");
    assert_eq!(state.rounds_completed, 36);
    assert_eq!(state.round(), true);
    assert_eq!(state.to_string(), "#######   
#.E.#.#   E(8)
#.#E..#   E(86)
#..#..#   
#...#.#   
#.....#   
#######   ");
    assert_eq!(state.rounds_completed, 37);
    assert_eq!(state.round(), false);
}

#[test]
fn part2example() {
    assert_eq!(part2("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"), 4988);
    assert_eq!(part2("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"), 31284);
    assert_eq!(part2("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"), 3478);
    assert_eq!(part2("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"), 6474);
    assert_eq!(part2("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"), 1140);
}

fn main() {
    aoc::main(part1, part2);
}
