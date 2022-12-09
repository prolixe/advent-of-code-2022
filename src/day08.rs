// Tree can keep the value of the highest tree in each direction to quickly answer if it's
// visible or not.
#[derive(Debug)]
struct Tree {
    height: usize,
    top: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    down: Option<usize>,
}

impl Tree {
    fn new(height: usize) -> Self {
        Tree {
            height,
            top: None,
            left: None,
            right: None,
            down: None,
        }
    }

    fn is_visible(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn evaluate(&mut self) {
        // Loop over all trees and update the value of the largest tree from each
        // direction
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (d_x, d_y) in directions.iter() {
            for y in 0..self.trees.len() {
                for x in 0..self.trees[0].len() {
                    let tree = &self.trees[y][x];
                    // Excellent example of how types saves my skin
                    let other_tree = self.trees.get(y + dy).get(x + d_x);
                }
            }
        }
    }
}

pub fn day08() {
    let contents = include_str!("../resources/day08.txt");

    let trees = contents
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| Tree::new(usize::try_from(c.to_digit(10).unwrap()).unwrap()))
                .collect()
        })
        .collect();
    let mut forest = Forest { trees };
    forest.evaluate();

    println!("Day 08, part 1: {:?}", forest);
    println!("Day 08, part 2: ");
}
