use crate::common::AdventResult;
use std::collections::HashMap;

fn parse_input_ratings(path: &str) -> AdventResult<Vec<u64>> {
    let content = std::fs::read_to_string(path)?;
    let mut ratings = Vec::new();
    for line in content.lines() {
        let rating = line.parse()?;
        ratings.push(rating);
    }
    Ok(ratings)
}

fn build_chain(mut ratings: Vec<u64>) -> Vec<u64> {
    // Sorted ratings + [0] + [max + 3]
    ratings.push(0);
    ratings.sort();
    ratings.push(ratings.last().unwrap() + 3);
    ratings
}

fn count_diffs(chain: &[u64]) -> Option<[usize; 3]> {
    let mut count = [0; 3];

    for window in chain.windows(2) {
        let diff = window[1] - window[0];
        if diff == 0 || diff > 3 {
            return None;
        }
        count[diff as usize - 1] += 1;
    }

    Some(count)
}

fn compute_signature(chain: &[u64]) -> Option<usize> {
    let counts = count_diffs(chain);
    if let Some([d1, _, d3]) = counts {
        Some(d3 * d1)
    } else {
        None
    }
}

fn count_part2(ratings: &[u64]) -> usize {
    fn inner_count(ratings: &[u64], i: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if cache.contains_key(&i) {
            return *cache.get(&i).unwrap();
        }

        if i == ratings.len() - 1 {
            return 1;
        }

        let mut res = 0;
        for j in (i + 1)..=(i + 3) {
            if j < ratings.len() && ratings[j] - ratings[i] <= 3 {
                res += inner_count(&ratings, j, cache);
            }
        }

        cache.insert(i, res);
        res
    }

    let mut cache = HashMap::new();
    inner_count(ratings, 0, &mut cache)
}

pub fn run(path: &str) {
    let ratings = parse_input_ratings(path).expect("Cannot read ratings");
    let chain = build_chain(ratings);
    let signature = compute_signature(&chain).expect("Cannot find a suitable chain");

    println!("day10 part1: {}", signature);

    let comb = count_part2(&chain);
    println!("day10 part2: {}", comb);
}
