use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::VecDeque;
use std::str;

type Stack = VecDeque<char>;

struct Command {
    count: i32,
    pos_start: usize,
    pos_end: usize,
}

pub fn day05() {
    let contents = include_str!("../resources/day05_example.txt");

    let (stacks, commands) = parse(contents);
    println!("Day 05, part 1:");
    println!("Day 05, part 1:");
}

fn parse(contents: &str) -> (Vec<Stack>, Vec<Command>) {
    let (stacks, commands) = contents.split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_commands(commands))
}

fn parse_stacks(contents: &str) -> Vec<Stack> {
    let length = contents
        .split('\n')
        .last()
        .unwrap()
        .split_whitespace()
        .count();
    println!("Debug stack length: {}", length);

    let mut boxes: Vec<&str> = contents.split('\n').collect();
    boxes.truncate(boxes.len() - 1);
    println!("Debug box : {:?}", boxes);

    let mut stacks: Vec<Stack> = vec![VecDeque::new(); length];

    for line in boxes.into_iter() {
        let row: Vec<_> = line.as_bytes().chunks(4).enumerate().collect();
        for r in row.into_iter() {
            let s = str::from_utf8(r.1)
                .unwrap()
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .last();

            if s.is_some() {
                stacks[r.0].push_front(s.unwrap());
            }
        }
    }

    stacks
}

fn parse_commands(contents: &str) -> Vec<Command> {
    contents.split('\n').map(parse_command).collect()
}

fn parse_command(line: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    for cap in RE.captures_iter(line) {
        println!("cap {:?}", cap);
    }
    let cap: Captures = RE.captures(line).unwrap();

    Command {
        count: cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        pos_start: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        pos_end: cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
    }
}
