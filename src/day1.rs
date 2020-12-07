use crate::common::*;

fn read_input(path: &str) -> AdventResult<Vec<u32>> {
    let file_content = std::fs::read_to_string(path)?;
    let mut values = Vec::new();
    for line in file_content.lines() {
        let value: u32 = line.parse()?;
        values.push(value);
    }
    Ok(values)
}

fn find_sum_of_2(values: &[u32]) -> Option<(u32, u32)> {
    for &a in values {
        for &b in values {
            if a + b == 2020 {
                return Some((a, b));
            }
        }
    }
    None
}

fn find_sum_of_3(values: &[u32]) -> Option<(u32, u32, u32)> {
    for &a in values {
        for &b in values {
            for &c in values {
                if a + b + c == 2020 {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

pub fn run(path: &str) {
    let values = read_input(path).expect("Cannot read input file");

    if let Some((a, b)) = find_sum_of_2(&values) {
        println!("day1 part1: {}", a * b);
    } else {
        println!("day1 part1: no solution found")
    }

    if let Some((a, b, c)) = find_sum_of_3(&values) {
        println!("day1 part2: {}", a * b * c);
    } else {
        println!("day1 part2: no solution found")
    }
}
