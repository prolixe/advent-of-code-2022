use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

type TreeNodeRc = Rc<RefCell<TreeNode>>;

enum FileSystem {
    Directory { name: String },
    File { name: String, size: usize },
}

struct TreeNode {
    value: FileSystem,
    parent: Option<TreeNodeRc>,
    children: Vec<TreeNodeRc>,
}

impl TreeNode {
    fn new() -> TreeNode {
        return TreeNode {
            value: FileSystem::Directory {
                name: "/".to_string(),
            },
            children: vec![],
            parent: None,
        };
    }
}

enum Command {
    ChangeDirDown(String),
    ChangeDirUp,
    ChangeDirRoot,
    List,
}

pub fn day07() {
    let contents = include_str!("../resources/day07_example.txt");

    let filesystem = parse(contents);
    println!("Day 07, part 1:");
    println!("Day 07, part 2:");
}

fn parse(contents: &str) -> FileSystem {
    let mut current = Rc::new(RefCell::new(TreeNode::new()));

    for line in contents.split("\n$ ") {
        let command = line.split('\n').collect::<Vec<_>>()[0];
        let command = parse_command(command);
        match command {
            Command::ChangeDirDown(name) => todo!(),
            Command::ChangeDirUp => {
                current = current.borrow_mut().parent.unwrap();
            }
            Command::ChangeDirRoot => continue,
            Command::List => {
                let outputs = line.split('\n').collect::<Vec<_>>()[1..].to_vec();
                // Add the content into the current directory
            }
        };
    }
    todo!()
}

fn parse_command(line: &str) -> Command {
    match line.trim() {
        "ls" => Command::List,
        "dir /" => Command::ChangeDirRoot,
        "dir .." => Command::ChangeDirUp,
        _ => {
            let (_, dir_name) = line.split_once(' ').unwrap();
            Command::ChangeDirDown(dir_name.to_string())
        }
    }
}

fn parse_outputs(outputs: &Vec<&str>, node: &mut TreeNodeRc) {
    for output in outputs {
        //node.borrow_mut().children.push(FileSyst)
    }
}

fn parse_output(output: &str) -> FileSystem {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s+) (\s+)").unwrap();
    }
    let cap: Captures = RE.captures(output).unwrap();
    if cap.get(1).unwrap().as_str().starts_with("dir") {
        let dir_name = cap.get(2).unwrap().as_str().to_string();
        FileSystem::Directory { name: dir_name }
    } else {
        let file_name = cap.get(2).unwrap().as_str().to_string();
        let filesize = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        FileSystem::File {
            name: file_name,
            size: filesize,
        }
    }
}
