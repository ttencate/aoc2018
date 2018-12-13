use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
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

    fn duration(&self, step_duration: u32) -> u32 {
        return step_duration + (self.id as u32 - 'A' as u32) + 1
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
        .filter_map(|(&id, node)| if node.borrow().can_start() { Some(id) } else { None })
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

fn part2_with_params(input: &str, num_workers: usize, step_duration: u32) -> u32 {
    let nodes = parse_input(input);
    let mut ready: BTreeSet<NodeId> = nodes
        .iter()
        .filter_map(|(&id, node)| if node.borrow().can_start() { Some(id) } else { None })
        .collect();
    let mut end_times: BTreeMap<NodeId, u32> = BTreeMap::new();
    let mut now = 0;
    while !ready.is_empty() || !end_times.is_empty() {
        while end_times.len() < num_workers && !ready.is_empty() {
            let curr_id = *ready.iter().next().unwrap();
            let curr = nodes[&curr_id].borrow_mut();
            ready.remove(&curr_id);
            end_times.insert(curr_id, now + curr.duration(step_duration));
        }
        if !end_times.is_empty() {
            let (&ended_id, &next_end_time) = end_times
                .iter()
                .min_by_key(|(_, &end_time)| end_time)
                .unwrap();
            assert!(now <= next_end_time);
            now = next_end_time;
            let mut ended = nodes[&ended_id].borrow_mut();
            end_times.remove(&ended_id);
            for out_edge in ended.out_edges.drain() {
                let mut next = nodes[&out_edge].borrow_mut();
                next.in_edges.remove(&ended_id);
                if next.can_start() {
                    ready.insert(next.id);
                }
            }
        }
    }
    now
}

fn part2(input: &str) -> u32 {
    part2_with_params(input, 5, 60)
}

#[test]
fn part2example() {
    assert_eq!(part2_with_params(EXAMPLE, 2, 0), 15);
}

fn main() {
    aoc::main(part1, part2);
}
