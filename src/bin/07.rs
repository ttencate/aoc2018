extern crate regex;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use regex::Regex;

#[allow(dead_code)]
static EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

type NodeId = char;

struct Node {
    id: NodeId,
    out_edges: HashSet<NodeId>,
    in_edges: HashSet<NodeId>,
}

impl Node {
    fn new(id: NodeId) -> Node {
        Node { id: id, out_edges: HashSet::new(), in_edges: HashSet::new() }
    }

    fn can_start(&self) -> bool {
        self.in_edges.is_empty()
    }
}

fn parse_input(input: &str) -> HashMap<char, RefCell<Node>> {
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let captures = re.captures(&line).unwrap();
        let fst = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let snd = captures.get(2).unwrap().as_str().chars().next().unwrap();
        nodes.entry(fst).or_insert(RefCell::new(Node::new(fst))).borrow_mut().out_edges.insert(snd);
        nodes.entry(snd).or_insert(RefCell::new(Node::new(snd))).borrow_mut().in_edges.insert(fst);
    }
    nodes
}

fn part1(input: &str) -> String {
    let nodes = parse_input(input);
    let mut ready: BTreeSet<NodeId> = nodes
        .iter()
        .filter_map(|(id, node)| if node.borrow().can_start() { Some(id) } else { None })
        .map(|x| *x) // Is there a better way?
        .collect();
    let mut out = Vec::with_capacity(nodes.len());
    while !ready.is_empty() {
        let curr_id = *ready.iter().next().unwrap();
        let mut curr = nodes[&curr_id].borrow_mut();
        ready.remove(&curr_id);
        for out_edge in curr.out_edges.drain() {
            let mut next = nodes[&out_edge].borrow_mut();
            next.in_edges.remove(&curr_id);
            if next.can_start() {
                ready.insert(next.id);
            }
        }
        out.push(curr_id);
    }
    out.into_iter().collect()
}

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), "CABDFE");
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(7, part1, part2);
}
