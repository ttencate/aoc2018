use aoc::geom::*;
use fixedbitset::FixedBitSet;
use regex::Regex;

struct Nanobot {
    pos: Point3,
    r: u32,
}

impl Nanobot {
    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.pos.distance_to(&other.pos) <= self.r
    }

    fn overlaps_with(&self, other: &Nanobot) -> bool {
        self.pos.distance_to(&other.pos) <= self.r + other.r
    }
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Nanobot {
                pos: Point3::new(
                    captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                r: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let nanobots = parse_input(input);
    let strongest = nanobots.iter()
        .max_by_key(|nanobot| nanobot.r)
        .unwrap();
    nanobots.iter()
        .filter(|nanobot| strongest.is_in_range(nanobot))
        .count()
}

#[test]
fn part1example() {
    assert_eq!(part1("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
"), 7);
}

/*
https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm

BronKerbosch1(Ø, all_vertices, Ø)

BronKerbosch1(R, P, X):
   if P and X are both empty:
       report R as a maximal clique
   for each vertex v in P:
       BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
       P := P \ {v}
       X := X ⋃ {v}
 */
fn find_maximal_cliques(adj: &Vec<FixedBitSet>, n: usize, r: FixedBitSet, mut p: FixedBitSet, mut x: FixedBitSet, out: &mut Vec<FixedBitSet>, largest_size: &mut usize) {
    let p_size = p.count_ones(..);
    let r_size = r.count_ones(..);
    let x_size = x.count_ones(..);

    // Optimization: if our current proto-clique plus candidate vertices is smaller than the
    // largest maximal clique found so far, then we can give up already.
    if r_size + p_size < *largest_size {
        return;
    }

    if p_size == 0 && x_size == 0 {
        if r_size > *largest_size {
            *largest_size = r_size;
            out.clear();
        }
        out.push(r);
    } else {
        let ones = p.ones().collect::<Vec<usize>>();
        let mut v = FixedBitSet::with_capacity(n);
        for i in ones {
            v.set(i, true);
            let nv = &adj[i];
            find_maximal_cliques(adj, n, &r | &v, &p & nv, &x & nv, out, largest_size);
            p.set(i, false);
            x.insert(i);
            v.set(i, false);
        }
    }
}

fn part2(input: &str) -> u32 {
    let nanobots = parse_input(input);
    let n = nanobots.len();

    // Build adjacency matrix.
    let adj = (0..n)
        .map(|i| {
            let mut neigh = FixedBitSet::with_capacity(n);
            for j in 0..n {
                neigh.set(j, i != j && nanobots[i].overlaps_with(&nanobots[j]));
            }
            neigh
        })
        .collect();

    let mut out = Vec::new();
    let mut largest_size = 0;
    find_maximal_cliques(&adj, n, FixedBitSet::with_capacity(n), (0..n).collect(), FixedBitSet::with_capacity(n), &mut out, &mut largest_size);

    let origin = Point3::origin();
    out.iter()
        .map(|solution| {
            solution
                .ones()
                .map(|i| nanobots[i].pos.distance_to(&origin).saturating_sub(nanobots[i].r))
                .max()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[test]
fn part2example() {
    assert_eq!(part2("pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"), 36);
}

fn main() {
    aoc::main(part1, part2);
}
