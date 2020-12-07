use crate::common::*;
use itertools::Itertools;
use std::collections::HashSet;

type AnswerGroup = Vec<HashSet<char>>;

fn read_answer_groups(path: &str) -> AdventResult<Vec<AnswerGroup>> {
    let content = std::fs::read_to_string(path)?;
    let mut groups = Vec::new();
    for block in split_blocks(&content) {
        let mut group = Vec::new();
        for answer in block {
            group.push(answer.chars().collect());
        }
        groups.push(group);
    }
    Ok(groups)
}

fn inner_part_count(
    path: &str,
    merger: fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
) -> usize {
    let groups = read_answer_groups(path).expect("Cannot read answer groups");

    groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold1(|a, b| merger(&a, &b))
                .unwrap_or_default()
        })
        .map(|answers| answers.len())
        .sum()
}

fn part1_count(path: &str) -> usize {
    inner_part_count(path, |a, b| a | b)
}

fn part2_count(path: &str) -> usize {
    inner_part_count(path, |a, b| a & b)
}

pub fn run(path: &str) {
    let part1_count: usize = part1_count(path);
    println!("day6 part1: {}", part1_count);

    let part2_count: usize = part2_count(path);
    println!("day6 part1: {}", part2_count);
}
