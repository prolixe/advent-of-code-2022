use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::VecDeque;
use std::str;

type Stack = VecDeque<char>;

struct Command {
    count: usize,
    pos_start: usize,
    pos_end: usize,
}

pub fn day05() {
    let contents = include_str!("../resources/day05.txt");

    let (mut stacks, commands) = parse(contents);

    for command in commands {
        for _ in 0..command.count {
            let b = stacks[command.pos_start - 1].pop_back().unwrap();
            stacks[command.pos_end - 1].push_back(b);
        }
    }

    let top_each_stacks: Vec<_> = stacks.iter().filter_map(|s| s.back()).copied().collect();

    println!(
        "Day 05, part 1: {:?}",
        top_each_stacks.into_iter().collect::<String>()
    );
    let (mut stacks, commands) = parse(contents);

    for command in commands {
        let stack = stacks[command.pos_start - 1].clone();
        stacks[command.pos_start - 1].truncate(stack.len() - command.count);
        // copy the end of the deque and collect it back as a stack
        let mut box_stack = stack
            .range(stack.len() - command.count..stack.len())
            .copied()
            .collect::<Stack>();
        stacks[command.pos_end - 1].append(&mut box_stack);
    }
    let top_each_stacks: Vec<_> = stacks.iter().filter_map(|s| s.back()).copied().collect();

    println!(
        "Day 05, part 2: {:?}",
        top_each_stacks.into_iter().collect::<String>()
    );
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

    let mut boxes: Vec<&str> = contents.split('\n').collect();
    /*
    Remove the number line
    ```
            [D]
        [N] [C]
        [Z] [M] [P]
        1   2   3
    ```
    */
    boxes.truncate(boxes.len() - 1);

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
    let cap: Captures = RE.captures(line).unwrap();
    Command {
        count: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        pos_start: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        pos_end: cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
    }
}
