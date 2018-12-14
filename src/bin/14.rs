use itertools::Itertools;

fn part1(input: &str) -> String {
    let num_recipes_to_make = input.trim().parse::<usize>().unwrap();

    let mut scoreboard: Vec<u8> = Vec::with_capacity(num_recipes_to_make + 10);
    scoreboard.push(3);
    scoreboard.push(7);

    let mut elves = vec![0, 1];

    while scoreboard.len() < num_recipes_to_make + 10 {
        let score_sum = elves.iter().map(|&idx| scoreboard[idx]).sum();
        if score_sum <= 9 {
            scoreboard.push(score_sum);
        } else {
            scoreboard.push(score_sum / 10);
            scoreboard.push(score_sum % 10);
        }

        for i in 0..elves.len() {
            elves[i] = (elves[i] + 1 + scoreboard[elves[i]] as usize) % scoreboard.len();
        }
    }

    scoreboard.iter().skip(num_recipes_to_make).take(10).map(u8::to_string).join("")
}

#[test]
fn part1example() {
    assert_eq!(part1("9"), "5158916779");
    assert_eq!(part1("5"), "0124515891");
    assert_eq!(part1("18"), "9251071085");
    assert_eq!(part1("2018"), "5941429882");
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
