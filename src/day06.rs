use std::collections::HashSet;

pub fn day06() {
    let contents = include_str!("../resources/day06.txt");

    let part1: Vec<_> = contents
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter(|(i, chars)| chars.iter().copied().collect::<HashSet<u8>>().len() == 4)
        .collect();

    println!("Day 06, part 1: {:?}", part1[0].0 + 4);
    let part2: Vec<_> = contents
        .as_bytes()
        .windows(14)
        .enumerate()
        .filter(|(i, chars)| chars.iter().copied().collect::<HashSet<u8>>().len() == 14)
        .collect();
    println!("Day 06, part 2: {:?}", part2[0].0 + 14);
}
