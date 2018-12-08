struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn parse(input: &mut Iterator<Item=u32>) -> Node {
        let num_children = input.next().unwrap() as usize;
        let num_metadata = input.next().unwrap() as usize;
        let children = (0..num_children).map(|_| Node::parse(input)).collect();
        let metadata = input.take(num_metadata).collect();
        Node {
            children: children,
            metadata: metadata,
        }
    }

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>() + self.children.iter().map(Node::metadata_sum).sum::<u32>()
    }
}

fn parse_tree(input: &str) -> Node {
    let mut iter = input.split_whitespace().map(str::parse::<u32>).map(Result::unwrap);
    let node = Node::parse(&mut iter);
    assert!(iter.next().is_none());
    node
}

fn part1(input: &str) -> u32 {
    let tree = parse_tree(input);
    tree.metadata_sum()
}

#[test]
fn part1example() {
    assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(8, part1, part2);
}
