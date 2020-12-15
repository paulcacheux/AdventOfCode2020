use core::panic;

use crate::common::AdventResult;

fn mod_inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

fn main() {
    println!("{}", mod_inv(42, 2017))
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    start_timestamp: u32,
    bus_ids: Vec<Option<u32>>,
}

impl PuzzleInput {
    fn find_first_bus(&self) -> Option<(u32, u32)> {
        for ts in self.start_timestamp.. {
            for bus_id in &self.bus_ids {
                if let Some(bus_id) = bus_id {
                    if ts % bus_id == 0 {
                        return Some((*bus_id, ts - self.start_timestamp));
                    }
                }
            }
        }
        None
    }

    fn is_valid_ts_part1(&self, ts: u32) -> bool {
        for (current_ts, bus_id) in (ts..).into_iter().zip(self.bus_ids.iter()) {
            if let Some(bus_id) = bus_id {
                if current_ts % bus_id != 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

fn read_input(path: &str) -> AdventResult<PuzzleInput> {
    let content = std::fs::read_to_string(path)?;
    let mut lines_iter = content.lines();

    let first_line = lines_iter.next().unwrap();
    let start_timestamp = first_line.parse()?;

    let second_line = lines_iter.next().unwrap();

    let mut bus_ids = Vec::new();
    for part in second_line.split(",") {
        if let Ok(id) = part.parse() {
            bus_ids.push(Some(id));
        } else {
            bus_ids.push(None);
        }
    }

    Ok(PuzzleInput {
        start_timestamp,
        bus_ids,
    })
}

fn part1(puzzle_input: &PuzzleInput) -> u32 {
    let (bus_id, ts) = puzzle_input
        .find_first_bus()
        .expect("Cannot find first bus");
    bus_id * ts
}

fn part2_raw(puzzle_input: &PuzzleInput) -> u32 {
    let first_id = puzzle_input
        .bus_ids
        .iter()
        .filter_map(|id| *id)
        .next()
        .expect("No first id");

    for coeff in 0.. {
        let ts = first_id * coeff;
        if puzzle_input.is_valid_ts_part1(ts) {
            return ts;
        }
    }
    panic!("No ts found");
}

fn part2(puzzle_input: &PuzzleInput) -> i64 {
    // https://fr.wikipedia.org/wiki/Th%C3%A9or%C3%A8me_des_restes_chinois
    let mut nis = Vec::new();
    let mut ais = Vec::new();
    for (offset, bus_id) in puzzle_input.bus_ids.iter().enumerate() {
        let offset = offset as u64;
        if let Some(bus_id) = bus_id.clone() {
            let mut ai = -(offset as i64);
            while ai < 0 {
                ai += bus_id as i64;
            }

            nis.push(bus_id as i64);
            ais.push(ai);
        }
    }

    assert_eq!(nis.len(), ais.len());

    let mut eis = Vec::new();

    for (i, &ni) in nis.iter().enumerate() {
        let mut ni_chap = 1;
        for (j, &nj) in nis.iter().enumerate() {
            if j != i {
                ni_chap *= nj;
            }
        }

        let vi = mod_inv(ni_chap, ni);
        let ei = vi * ni_chap;
        eis.push(ei);
    }

    let n: i64 = nis.iter().product();

    assert_eq!(eis.len(), ais.len());
    let x: i64 = eis
        .into_iter()
        .zip(ais.into_iter())
        .map(|(ei, ai)| ei * ai)
        .sum();

    x % n
}

pub fn run(path: &str) {
    let puzzle_input = read_input(path).expect("Cannot read puzzle input");
    println!("day13 part1: {}", part1(&puzzle_input));
    println!("day13 part2: {}", part2(&puzzle_input));
}
