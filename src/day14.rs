use crate::common::AdventResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
    Mask(Vec<u8>),
    Set { address: u64, value: u64 },
}

fn combine_with_mask(mask: &[u8], value: u64) -> u64 {
    let mut res = value;
    for (i, mask_bit) in mask.iter().rev().enumerate() {
        assert!(i < 36);

        match mask_bit {
            b'1' => {
                res |= 0b1 << i;
            }
            b'0' => {
                res &= !(0b1 << i);
            }
            _ => {}
        }
    }
    res
}

fn combine_with_mask_v2(mask: &[u8], value: u64) -> Vec<u64> {
    let mut res = vec![value];
    for (i, mask_bit) in mask.iter().rev().enumerate() {
        assert!(i < 36);

        match mask_bit {
            b'1' => {
                res = res.into_iter().map(|value| value | 0b1 << i).collect();
            }
            b'X' => {
                res = res
                    .into_iter()
                    .flat_map(|value| vec![value | 0b1 << i, value & !(0b1 << i)].into_iter())
                    .collect();
            }
            _ => {}
        }
    }
    res
}

fn read_instructions(path: &str) -> AdventResult<Vec<Instruction>> {
    let content = std::fs::read_to_string(path)?;
    let mut instructions = Vec::new();
    for line in content.lines() {
        let inst = if line.starts_with("mask") {
            let mask = line.trim_start_matches("mask = ").trim_end().to_owned();
            let mask = mask.bytes().collect();
            Instruction::Mask(mask)
        } else {
            let parts: Vec<_> = line.split(" = ").collect();
            let start = parts[0];
            let address = start
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()?;

            let value = parts[1].parse()?;

            Instruction::Set { address, value }
        };

        instructions.push(inst);
    }
    Ok(instructions)
}

#[derive(Debug, Default)]
struct Memory {
    mem: HashMap<u64, u64>,
    mask: Vec<u8>,
}

impl Memory {
    fn run_inst(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::Set { address, value } => {
                let value = combine_with_mask(&self.mask, *value);
                self.mem.insert(*address, value);
            }
        }
    }

    fn run_inst_v2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::Set { address, value } => {
                let addresses = combine_with_mask_v2(&self.mask, *address);
                for addr in &addresses {
                    self.mem.insert(*addr, *value);
                }
            }
        }
    }

    fn sum_values(&self) -> u64 {
        self.mem.values().sum()
    }
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut mem = Memory::default();

    for inst in instructions {
        mem.run_inst(inst);
    }
    mem.sum_values()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut mem = Memory::default();

    for inst in instructions {
        mem.run_inst_v2(inst);
    }
    mem.sum_values()
}

pub fn run(path: &str) {
    let instructions = read_instructions(path).expect("Cannot read instructions");
    println!("day14 part1: {}", part1(&instructions));
    println!("day14 part2: {}", part2(&instructions));
}
