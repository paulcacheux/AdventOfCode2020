use crate::common::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\d+) (.+) bags?").unwrap();
}

#[derive(Debug, Clone)]
struct Rule {
    color: String,
    containing: HashMap<String, usize>,
}

fn parse_sub_rule(sub_rule: &str) -> (String, usize) {
    if let Some(captures) = BAG_RE.captures(sub_rule) {
        let count = captures[1].parse().expect("Cannot parse sub rule count");
        let color = captures[2].to_owned();
        (color, count)
    } else {
        panic!("Cannot parse sub rule:  {}", sub_rule)
    }
}

fn parse_rule(line: &str) -> Rule {
    let line = line.trim_end_matches('.');

    let main_parts: Vec<_> = line.split(" contain ").collect();
    assert_eq!(main_parts.len(), 2);
    let color = main_parts[0].trim_end_matches(" bags").to_owned();

    let containing_part = main_parts[1];
    let containing = if containing_part == "no other bags" {
        HashMap::new()
    } else {
        containing_part.split(", ").map(parse_sub_rule).collect()
    };

    Rule { color, containing }
}

fn parse_rules(path: &str) -> AdventResult<Vec<Rule>> {
    let content = std::fs::read_to_string(path)?;
    let rules = content.lines().map(parse_rule).collect();
    Ok(rules)
}

fn build_parent_tree(rules: &[Rule]) -> HashMap<String, Vec<String>> {
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for rule in rules {
        for (child, _) in &rule.containing {
            tree.entry(child.clone())
                .or_default()
                .push(rule.color.clone());
        }
    }
    tree
}

fn part1_count(tree: &HashMap<String, Vec<String>>) -> usize {
    let mut open_queue = vec!["shiny gold".to_owned()];
    let mut visited = HashSet::new();

    while let Some(current) = open_queue.pop() {
        if let Some(parents) = tree.get(&current) {
            for parent in parents {
                if !visited.contains(parent) {
                    open_queue.push(parent.clone());
                }
            }
        }
        visited.insert(current);
    }
    visited.len() - 1 // -1 for the shiny gold that was visited
}

fn count_bags(bag_color: &str, tree: &HashMap<String, HashMap<String, usize>>) -> usize {
    if let Some(children) = tree.get(bag_color) {
        let mut res = 1;
        for (child, count) in children {
            res += count_bags(child, tree) * count;
        }
        res
    } else {
        1
    }
}

fn part2_count(rules: &[Rule]) -> usize {
    let mut children: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for rule in rules {
        children.insert(rule.color.clone(), rule.containing.clone());
    }
    count_bags("shiny gold", &children) - 1 // -1 for the main bag
}

pub fn run(path: &str) {
    let rules = parse_rules(path).expect("Cannot parse rules");

    let tree = build_parent_tree(&rules);
    println!("day7 part1: {}", part1_count(&tree));

    println!("day7 part2: {}", part2_count(&rules));
}
