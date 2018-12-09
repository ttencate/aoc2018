extern crate regex;

use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ops::DerefMut;

struct Node<T> {
    prev_idx: usize,
    next_idx: usize,
    value: T,
}

// A circular linked list data structure where all the nodes are owned by a single Vec. This is a
// nice way to keep ownership clear and obvious, and also has a more efficient memory layout than a
// pointer-based implementation.
struct CircularLinkedList<T> {
    nodes: Vec<Node<T>>,
}

impl<T> CircularLinkedList<T> {
    fn with_capacity(capacity: usize) -> CircularLinkedList<T> {
        CircularLinkedList { nodes: Vec::with_capacity(capacity) }
    }

    // Just because I don't want to think about iterators in the empty-list case...
    fn add<'a>(&'a mut self, element: T) -> IterMut<T> {
        assert!(self.nodes.is_empty());
        self.nodes.push(Node { prev_idx: 0, next_idx: 0, value: element });
        IterMut { list: self, current_idx: 0 }
    }
}

// A bidirectional iterator-like thing that points to a single value in the CircularLinkedList.
struct IterMut<'a, T: 'a> {
    list: &'a mut CircularLinkedList<T>,
    current_idx: usize,
}

impl<'a, T> IterMut<'a, T> {
    fn nodes(&self) -> &Vec<Node<T>> {
        &self.list.nodes
    }

    fn nodes_mut(&mut self) -> &mut Vec<Node<T>> {
        &mut self.list.nodes
    }

    fn node(&self, idx: usize) -> &Node<T> {
        &self.nodes()[idx]
    }

    fn node_mut(&mut self, idx: usize) -> &mut Node<T> {
        &mut self.nodes_mut()[idx]
    }

    fn prev(&mut self) {
        let current_idx = self.current_idx;
        self.current_idx = self.node(current_idx).prev_idx;
    }

    fn next(&mut self) {
        let current_idx = self.current_idx;
        self.current_idx = self.node(current_idx).next_idx;
    }

    fn insert_after(&mut self, element: T) {
        let current_idx = self.current_idx;
        let next_idx = self.node(current_idx).next_idx;
        let new_idx = self.nodes().len();
        self.list.nodes.push(Node {
            prev_idx: self.current_idx,
            next_idx: next_idx,
            value: element,
        });
        self.node_mut(current_idx).next_idx = new_idx;
        self.node_mut(next_idx).prev_idx = new_idx;
    }

    // To prevent "leaking" the node, we should actually copy the last node into the position of
    // the deleted node and update indices accordingly. But in this puzzle, there's no need.
    fn remove_and_next(&mut self) {
        let current_idx = self.current_idx;
        let prev_idx = self.node(current_idx).prev_idx;
        let next_idx = self.node(current_idx).next_idx;
        self.node_mut(prev_idx).next_idx = next_idx;
        self.node_mut(next_idx).prev_idx = prev_idx;
        self.current_idx = next_idx;
    }
}

impl<'a, T> Deref for IterMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        let current_idx = self.current_idx;
        &self.node(current_idx).value
    }
}

impl<'a, T> DerefMut for IterMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        let current_idx = self.current_idx;
        &mut self.node_mut(current_idx).value
    }
}

impl<'a, T> Display for IterMut<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({:2})", **self);
        let mut idx = self.list.nodes[self.current_idx].next_idx;
        while idx != self.current_idx {
            write!(f, " {:2} ", self.list.nodes[idx].value);
            idx = self.list.nodes[idx].next_idx;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let captures = Regex::new(r"(\d+) players; last marble is worth (\d+) points")
        .unwrap()
        .captures(input)
        .unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
    )
}

fn part1(input: &str) -> usize {
    let (num_players, last_marble_value) = parse_input(input);
    winning_score(num_players, last_marble_value)
}

fn winning_score(num_players: usize, last_marble_value: usize) -> usize {
    let mut marbles = CircularLinkedList::with_capacity(last_marble_value + 1);
    let mut current = marbles.add(0);

    let mut scores = vec![0; num_players];
    let mut current_player = 0;

    // println!("[-] {}", current);
    for value in 1 ..= last_marble_value {
        if value % 23 != 0 {
            current.next();
            current.insert_after(value);
            current.next();
        } else {
            scores[current_player] += value;
            for _ in 0..7 {
                current.prev();
            }
            scores[current_player] += *current;
            current.remove_and_next();
        }
        // println!("[{}] {}", current_player + 1, current);
        current_player = (current_player + 1) % num_players;
    }

    *scores.iter().max().unwrap()
}

#[test]
fn part1examples() {
    assert_eq!(part1("9 players; last marble is worth 25 points"), 32);
    assert_eq!(part1("10 players; last marble is worth 1618 points"), 8317);
    assert_eq!(part1("13 players; last marble is worth 7999 points"), 146373);
    assert_eq!(part1("17 players; last marble is worth 1104 points"), 2764);
    assert_eq!(part1("21 players; last marble is worth 6111 points"), 54718);
    assert_eq!(part1("30 players; last marble is worth 5807 points"), 37305);
}

fn part2(input: &str) -> usize {
    let (num_players, last_marble_value) = parse_input(input);
    winning_score(num_players, 100 * last_marble_value)
}

#[test]
fn part2example() {
}

fn main() {
    aoc::main(part1, part2);
}
