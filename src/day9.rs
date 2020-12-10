use crate::common::*;

fn parse_input_numbers(path: &str) -> AdventResult<Vec<u64>> {
    let content = std::fs::read_to_string(path)?;

    let mut numbers = Vec::new();
    for line in content.lines() {
        let value = line.parse()?;
        numbers.push(value);
    }
    Ok(numbers)
}

const WINDOW_LEN: usize = 25;

fn find_weakness(numbers: &[u64]) -> Option<u64> {
    for window in numbers.windows(WINDOW_LEN + 1) {
        let current = window[WINDOW_LEN];
        let prelude = &window[0..WINDOW_LEN];
        assert_eq!(prelude.len(), WINDOW_LEN);

        let mut found_pair = false;
        for a in prelude {
            for b in prelude {
                if a != b && a + b == current {
                    found_pair = true;
                }
            }
        }

        if !found_pair {
            return Some(current);
        }
    }
    None
}

fn find_consecutive_sum(numbers: &[u64], weakness: u64) -> Option<&[u64]> {
    for len in 2..(numbers.len() + 1) {
        for window in numbers.windows(len) {
            let sum: u64 = window.iter().sum();
            if sum == weakness {
                return Some(window);
            }
        }
    }
    None
}

fn compute_consec_signature(numbers: &[u64], weakness: u64) -> Option<u64> {
    let sum_window = find_consecutive_sum(numbers, weakness);
    sum_window.and_then(|window| {
        let min = window.iter().min();
        let max = window.iter().max();

        match (min, max) {
            (Some(min), Some(max)) => Some(min + max),
            _ => None,
        }
    })
}

pub fn run(path: &str) {
    let numbers = parse_input_numbers(path).expect("Cannot read input numbers");
    let weakness = find_weakness(&numbers).expect("No weakness found");
    println!("day9 part1: {}", weakness);

    let signature = compute_consec_signature(&numbers, weakness).expect("No signature found");
    println!("day9 part2: {}", signature);
}
