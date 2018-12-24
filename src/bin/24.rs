use regex::Regex;
use std::fmt::{Display, Formatter};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
struct Group {
    id: u32,
    num_units: u32,
    hit_points_per_unit: u32,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    attack_damage: u32,
    attack_type: String,
    initiative: u32,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.num_units * self.attack_damage
    }

    fn damage_to(&self, enemy: &Group) -> u32 {
        // "By default, an attacking group would deal damage equal to its effective power to the
        // defending group."
        let default_damage = self.effective_power();
        if enemy.immunities.contains(&self.attack_type) {
            // "However, if the defending group is immune to the attacking group's attack type, the
            // defending group instead takes no damage"
            0
        } else if enemy.weaknesses.contains(&self.attack_type) {
            // "if the defending group is weak to the attacking group's attack type, the defending
            // group instead takes double damage."
            2 * default_damage
        } else {
            default_damage
        }
    }

    fn units_killed(&self, damage: u32) -> u32 {
        // "The defending group only loses whole units from damage; damage is always dealt in such
        // a way that it kills the most units possible, and any remaining damage to a unit that
        // does not immediately kill it is ignored."
        std::cmp::min(self.num_units, damage / self.hit_points_per_unit)
    }

    fn is_defeated(&self) -> bool {
        self.num_units == 0
    }
}

#[derive(Clone, Debug)]
struct Army {
    name: String,
    groups: Vec<Group>,
}

impl Army {
    fn is_defeated(&self) -> bool {
        self.groups.iter().all(|group| group.is_defeated())
    }
}

impl Display for Army {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "{:}:", self.name)?;
        for group in &self.groups {
            if group.num_units > 0 {
                writeln!(f, "Group {:} contains {:} units", group.id, group.num_units)?;
            }
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Army> {
    let mut lines = input.lines();
    let name_re = Regex::new(r"^(?P<name>.*?):$").unwrap();
    let units_re = Regex::new(r"(\d+) units").unwrap();
    let hit_points_re = Regex::new(r"(\d+) hit points").unwrap();
    let immunities_re = Regex::new(r"immune to ([\w, ]+)").unwrap();
    let weaknesses_re = Regex::new(r"weak to ([\w, ]+)").unwrap();
    let attack_re = Regex::new(r"(\d+) (\w+) damage").unwrap();
    let initiative_re = Regex::new(r"initiative (\d+)").unwrap();
    (0..2)
        .map(|_| {
            let line = lines.next().unwrap();
            let captures = name_re.captures(line).unwrap();
            let name = captures.name("name").unwrap().as_str().to_string();
            let mut groups = Vec::new();
            let mut next_id = 1;
            while let Some(line) = lines.next() {
                if line.len() == 0 {
                    break;
                }
                groups.push(Group {
                    id: next_id,
                    num_units: units_re.captures(line).unwrap()
                        .get(1).unwrap().as_str()
                        .parse::<u32>().unwrap(),
                    hit_points_per_unit: hit_points_re.captures(line).unwrap()
                        .get(1).unwrap().as_str()
                        .parse::<u32>().unwrap(),
                    immunities: immunities_re.captures(line)
                        .map(|captures| {
                            captures.get(1).unwrap().as_str()
                                .split(", ").map(str::to_string).collect()
                        })
                        .unwrap_or(Vec::new()),
                    weaknesses: weaknesses_re.captures(line)
                        .map(|captures| {
                            captures.get(1).unwrap().as_str()
                                .split(", ").map(str::to_string).collect()
                        }).unwrap_or(Vec::new()),
                    attack_damage: attack_re.captures(line).unwrap()
                        .get(1).unwrap().as_str()
                        .parse::<u32>().unwrap(),
                    attack_type: attack_re.captures(line).unwrap()
                        .get(2).unwrap().as_str().to_string(),
                    initiative: initiative_re.captures(line).unwrap()
                        .get(1).unwrap().as_str()
                        .parse::<u32>().unwrap(),
                });
                next_id += 1;
            }
            Army { name: name, groups: groups }
        })
        .collect()
}

fn fight_round(armies: &mut Vec<Army>) -> u32 {
    // for army in armies.iter() { print!("{:}", army); }
    // println!("");

    // "During the target selection phase, each group attempts to choose one target."
    let target_indices: Vec<Vec<Option<usize>>> = (0..2)
        .map(|army_idx| {
            let groups = &armies[army_idx].groups;
            let enemy_idx = 1 - army_idx;
            let defending_groups = &armies[enemy_idx].groups;
            let mut choose_order: Vec<usize> = (0..groups.len()).collect();
            choose_order.sort_by_key(|&group_idx| {
                (
                    // "In decreasing order of effective power, groups choose their targets"
                    -(groups[group_idx].effective_power() as i32),
                    // "in a tie, the group with the higher initiative chooses first."
                    -(groups[group_idx].initiative as i32),
                )
            });
            let mut target_indices: BTreeSet<usize> = (0..defending_groups.len()).collect();
            let mut targets = vec![None; groups.len()];
            for group_idx in choose_order {
                let attacking_group = &armies[army_idx].groups[group_idx];
                let target_idx = target_indices.iter()
                    // "If it cannot deal any defending groups damage, it does not choose a
                    // target."
                    .filter(|&&target_idx| {
                        let defending_group = &defending_groups[target_idx];
                        !defending_group.is_defeated() &&
                            attacking_group.damage_to(defending_group) > 0
                    })
                    .max_by_key(|&&target_idx| {
                        let defending_group = &defending_groups[target_idx];
                        let damage = attacking_group.damage_to(defending_group);
                        // println!("{:} group {:} would deal defending group {:} {:} damage",
                        //     armies[army_idx].name, attacking_group.id, defending_group.id, damage);
                        (
                            // "The attacking group chooses to target the group in the enemy
                            // army to which it would deal the most damage"
                            damage,
                            // "If an attacking group is considering two defending groups to
                            // which it would deal equal damage, it chooses to target the
                            // defending group with the largest effective power"
                            defending_group.effective_power(),
                            // "if there is still a tie, it chooses the defending group with
                            // the highest initiative"
                            defending_group.initiative,
                        )
                    })
                    .cloned();
                // "Defending groups can only be chosen as a target by one attacking group."
                if let Some(target_idx) = target_idx {
                    targets[group_idx] = Some(target_idx);
                    target_indices.remove(&target_idx);
                }
            }
            targets
        })
        .collect();

    // println!("");

    // "During the attacking phase, each group deals damage to the target it selected, if any."
    let mut attack_order: Vec<(usize, usize)> = (0..2)
        .flat_map(|army_idx| {
            (0..armies[army_idx].groups.len())
                .map(move |group_idx| (army_idx, group_idx))
        })
        .collect();
    // "Groups attack in decreasing order of initiative, regardless of whether they are part of the
    // infection or the immune system."
    attack_order.sort_by_key(|(army_idx, group_idx)| {
        -(armies[*army_idx].groups[*group_idx].initiative as i32)
    });
    let mut num_kills = 0;
    for (army_idx, group_idx) in attack_order {
        let attacking_group = &armies[army_idx].groups[group_idx];
        // println!("{:?}", attacking_group);
        // "(If a group contains no units, it cannot attack.)"
        if attacking_group.is_defeated() {
            continue;
        }
        if let Some(target_idx) = target_indices[army_idx][group_idx] {
            let defending_group = &armies[1 - army_idx].groups[target_idx];
            let damage = attacking_group.damage_to(defending_group);
            let units_killed = defending_group.units_killed(damage);
            // println!("{:} group {:} attacks defending group {:}, killing {:} units",
            //     armies[army_idx].name, attacking_group.id, defending_group.id, units_killed);

            let defending_group = &mut armies[1 - army_idx].groups[target_idx];
            defending_group.num_units -= units_killed;
            num_kills += units_killed;
        }
    }

    // println!("");

    num_kills
}

fn winning_army(armies: &mut Vec<Army>) -> Option<usize> {
    // "combat only ends once one army has lost all of its units."
    armies.iter()
        .enumerate()
        .find(|(_idx, army)| army.is_defeated())
        .map(|(idx, _army)| 1 - idx)
}

fn fight_until_end(armies: &mut Vec<Army>) -> Option<usize> {
    // "After the fight is over, if both armies still contain units, a new fight begins"
    loop {
        let winning_army = winning_army(armies);
        if winning_army.is_some() {
            break winning_army;
        }
        let num_kills = fight_round(armies);
        if num_kills == 0 {
            break None;
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut armies = parse_input(input);
    let winning_army = fight_until_end(&mut armies).unwrap();
    armies[winning_army].groups.iter().map(|group| group.num_units).sum()
}

#[cfg(test)]
const EXAMPLE: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

#[test]
fn part1example() {
    assert_eq!(part1(EXAMPLE), 5216);
}

fn boost_army(armies: &Vec<Army>, name: &str, boost: u32) -> Vec<Army> {
    let mut boosted_armies = armies.clone();
    for army in &mut boosted_armies {
        if army.name == name {
            for group in &mut army.groups {
                group.attack_damage += boost;
            }
        }
    }
    boosted_armies
}

fn part2(input: &str) -> u32 {
    let armies = parse_input(input);
    let immune_system_name = "Immune System";
    for boost in 0.. {
        println!("Boost: {}", boost);
        let mut boosted_armies = boost_army(&armies, immune_system_name, boost);
        let winning_army = fight_until_end(&mut boosted_armies);
        if let Some(winning_army) = winning_army {
            if boosted_armies[winning_army].name == immune_system_name {
                return boosted_armies[winning_army].groups.iter().map(|group| group.num_units).sum()
            }
        }
    }
    panic!();
}

#[test]
fn part2example() {
   assert_eq!(part2(EXAMPLE), 51);
}

fn main() {
    aoc::main(part1, part2);
}
