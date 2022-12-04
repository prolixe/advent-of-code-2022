use std::ops::Range;

type SectionRange = Range<i32>;

type Ranges = (SectionRange, SectionRange);

pub fn day04() {
    let contents = include_str!("../resources/day04.txt");
    let ranges = parse(contents);

    let count_fully_contains = ranges
        .iter()
        .filter(|(f, s)| contains_range(f, s.clone()) || contains_range(s, f.clone()))
        .count();

    println!("Day 04, part 1: {}", count_fully_contains);

    let count_contains_any = ranges
        .iter()
        .filter(|(f, s)| contains_some(f, s.clone()))
        .count();

    println!("Day 04, part 2: {}", count_contains_any);
}

fn parse(contents: &str) -> Vec<Ranges> {
    contents.trim().split('\n').map(parse_row).collect()
}

fn parse_row(contents: &str) -> Ranges {
    let (first, second) = contents.split_once(',').unwrap();
    let first_range = parse_range(first);
    let second_range = parse_range(second);
    (first_range, second_range)
}

fn parse_range(contents: &str) -> SectionRange {
    let (start, end) = contents.split_once('-').unwrap();
    SectionRange {
        start: start.parse().unwrap(),
        end: (end.parse::<i32>().unwrap() + 1), // A range's end is exclusive
    }
}

// Does the first range contains the seconds?
fn contains_range(first: &SectionRange, other: SectionRange) -> bool {
    other.into_iter().all(|item| first.contains(&item))
}

fn contains_some(first: &SectionRange, other: SectionRange) -> bool {
    other.into_iter().any(|item| first.contains(&item))
}
