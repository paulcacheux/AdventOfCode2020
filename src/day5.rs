use crate::common::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pass {
    row: u32,
    column: u32,
}

impl Pass {
    fn seat_id(self) -> u32 {
        self.row * 8 + self.column
    }
}

fn read_pass(line: &str) -> Pass {
    let line = line.trim();

    let mut row_coeff = 64;
    let mut row = 0;
    for c in line.chars().take(7) {
        match c {
            'B' => row += row_coeff,
            'F' => {}
            _ => unreachable!(),
        }
        row_coeff >>= 1;
    }

    let mut col_coeff = 4;
    let mut column = 0;
    for c in line.chars().skip(7).take(3) {
        match c {
            'R' => column += col_coeff,
            'L' => {}
            _ => unreachable!(),
        }
        col_coeff >>= 1;
    }

    Pass { row, column }
}

fn read_passes(path: &str) -> AdventResult<Vec<Pass>> {
    let content = std::fs::read_to_string(path)?;
    let passes = content.lines().map(read_pass).collect();
    Ok(passes)
}

fn find_missing_passes(pass_set: &HashSet<Pass>) -> Vec<Pass> {
    let mut missing_passes = Vec::new();
    for row in 0..128 {
        for column in 0..8 {
            let search_pass = Pass { row, column };
            if !pass_set.contains(&search_pass) {
                missing_passes.push(search_pass);
            }
        }
    }
    missing_passes
}

fn find_possible_passes(passes: &[Pass]) -> Vec<Pass> {
    let pass_set: HashSet<Pass> = passes.into_iter().copied().collect();
    let possibles = find_missing_passes(&pass_set)
        .into_iter()
        .filter(|pass| pass.row != 0 && pass.row != 127);

    let ids: HashSet<_> = passes.iter().map(|p| p.seat_id()).collect();
    possibles
        .filter(|pass| {
            let seat_id = pass.seat_id();
            ids.contains(&(seat_id - 1)) && ids.contains(&(seat_id + 1))
        })
        .collect()
}

fn find_part2_seat_id(passes: &[Pass]) -> u32 {
    let possibles = find_possible_passes(passes);
    assert_eq!(possibles.len(), 1);

    possibles[0].seat_id()
}

pub fn run(path: &str) {
    let passes = read_passes(path).expect("Cannot read passes");

    let part1_max = passes
        .iter()
        .map(|pass| pass.seat_id())
        .max()
        .expect("Cannot take the max of the seat ids");

    println!("day5 part1: {}", part1_max);
    println!("day5 part2: {:?}", find_part2_seat_id(&passes));
}
