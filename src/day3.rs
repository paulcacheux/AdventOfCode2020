use crate::common::*;

#[derive(Debug, Clone)]
struct TreeMap {
    trees: Vec<Vec<bool>>,
}

impl TreeMap {
    fn height(&self) -> usize {
        self.trees.len()
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        let line = &self.trees[y];
        let x = x % line.len();
        line[x]
    }

    fn count_slope(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut y = 0;

        let mut counter = 0;
        while y < self.height() {
            if self.is_tree(x, y) {
                counter += 1;
            }

            x += dx;
            y += dy;
        }
        counter
    }

    fn part1_count(&self) -> usize {
        self.count_slope(3, 1)
    }

    fn part2_count(&self) -> usize {
        let diffs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut res = 1;
        for &(dx, dy) in &diffs {
            res *= self.count_slope(dx, dy);
        }
        res
    }
}

impl std::fmt::Display for TreeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.trees {
            for t in line {
                write!(f, "{}", if *t { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn read_map(path: &str) -> AdventResult<TreeMap> {
    let content = std::fs::read_to_string(path)?;

    let mut lines = Vec::new();
    for line in content.lines() {
        let tree_line = line.chars().map(|c| c == '#').collect();
        lines.push(tree_line);
    }
    Ok(TreeMap { trees: lines })
}

pub fn day3_main() {
    let path = "inputs/day3/input.txt";
    let tm = read_map(path).expect("Cannot read tree map");
    println!("day3 part1: {}", tm.part1_count());
    println!("day3 part2: {}", tm.part2_count());
}
