use crate::utils::Result;

pub fn day01() {
    let contents = include_str!("../resources/day01.txt");

    let elfs: Vec<Vec<i32>> = contents
        .trim()
        .split("\n\n")
        .map(|snacks| process_snacks(snacks).unwrap())
        .collect();

    let mut elfs_total_calories: Vec<i32> = elfs.iter().map(|snacks| snacks.iter().sum()).collect();
    println!(
        "Part 1: Largest amount of calories: {:?}",
        elfs_total_calories.iter().max().unwrap()
    );
    elfs_total_calories.sort();

    let elfs_top_3_sum: i32 = elfs_total_calories[elfs_total_calories.len() - 3..]
        .iter()
        .sum();
    println!(
        "Part 2: Largest amount of calories of top 3 elfs' snacks: {:?}",
        elfs_top_3_sum
    );
}

fn process_snacks(snacks: &str) -> Result<Vec<i32>> {
    Ok(snacks
        .trim()
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect())
}
