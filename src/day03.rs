use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Rucksack {
    fn duplicate(&self) -> Option<char> {
        let mut dup = self.first.intersection(&self.second);
        dup.next().copied()
    }
    fn all(&self) -> HashSet<char> {
        self.first.union(&self.second).copied().collect()
    }
}

pub fn day03() {
    let contents = include_str!("../resources/day03.txt");
    let rucksacks: Vec<&str> = contents.trim().split('\n').collect();

    let rucksacks: Vec<_> = rucksacks.into_iter().map(split_sack).collect();

    let sum_priorities: i32 = rucksacks
        .iter()
        .filter_map(|r| r.duplicate())
        .map(|d| priority(&d))
        .sum();

    println!("Day 03 part 1: {}", sum_priorities);

    let grouped_sacks: Vec<_> = rucksacks
        .chunks(3)
        .map(|rucksacks| {
            rucksacks[1..]
                .iter()
                .fold(rucksacks[0].all(), |group, val| {
                    group.intersection(&val.all()).copied().collect()
                })
        })
        .filter_map(|set| set.iter().next().copied())
        .collect();

    let group_priorities: i32 = grouped_sacks.iter().map(priority).sum();
    println!("Day 03 part 2: {}", group_priorities);
}

fn split_sack(sack: &str) -> Rucksack {
    let length = sack.len();
    let first_half = sack.chars().collect::<Vec<char>>()[0..length / 2].to_vec();
    let first: HashSet<char> = first_half.into_iter().collect();
    let second_half = sack.chars().collect::<Vec<char>>()[length / 2..].to_vec();
    let second: HashSet<char> = second_half.into_iter().collect();
    Rucksack { first, second }
}

fn priority(c: &char) -> i32 {
    if c.is_lowercase() {
        *c as i32 - 'a' as i32 + 1
    } else {
        *c as i32 - 'A' as i32 + 27
    }
}
