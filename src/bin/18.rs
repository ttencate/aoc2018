use aoc::geom::Matrix;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::collections::HashMap;

const OPEN: u8 = '.' as u8;
const TREES: u8 = '|' as u8;
const LUMBERYARD: u8 = '#' as u8;
const NONE: u8 = ' ' as u8;

fn iterate(input: &Matrix<u8>) -> Matrix<u8> {
    let mut output = Matrix::new(input.rect(), NONE);
    for p in input.coords() {
        let mut counts = [OPEN, TREES, LUMBERYARD, NONE]
            .iter()
            .map(|c| (*c, 0))
            .collect::<HashMap<u8, usize>>();
        let center = input[p];
        for &p in &p.neighbors_diagonal() {
            *counts.get_mut(input.get(p).unwrap_or(&NONE)).unwrap() += 1;
        }
        output[p] = match center {
            OPEN if counts[&TREES] >= 3 => TREES,
            TREES if counts[&LUMBERYARD] >= 3 => LUMBERYARD,
            LUMBERYARD if counts[&LUMBERYARD] < 1 || counts[&TREES] < 1 => OPEN,
            _ => center
        }
    }
    output
}

fn part1(input: &str) -> usize {
    let mut mat = input.lines().collect::<Matrix<u8>>();
    for _ in 0..10 {
        mat = iterate(&mat);
    }
    mat.coords().filter(|&p| mat[p] == TREES).count() * mat.coords().filter(|&p| mat[p] == LUMBERYARD).count()
}

#[test]
fn part1example() {
    assert_eq!(part1(".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."), 1147);
}

fn hash(mat: &Matrix<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.input(mat.as_slice());
    hasher.result_str()
}

fn part2(input: &str) -> usize {
    let mut mat = input.lines().collect::<Matrix<u8>>();
    let mut hashes = HashMap::new();
    let mut i = 0;
    while i < 1000000000 {
        let hash = hash(&mat);

        if hashes.contains_key(&hash) {
            let step = i - hashes[&hash];
            i += (1000000000 - i) / step * step;
            hashes.clear();
        }

        hashes.insert(hash, i);

        mat = iterate(&mat);
        i += 1;
    }
    mat.coords().filter(|&p| mat[p] == TREES).count() * mat.coords().filter(|&p| mat[p] == LUMBERYARD).count()
}

fn main() {
    aoc::main(part1, part2);
}
