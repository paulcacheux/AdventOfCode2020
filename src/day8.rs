use std::collections::HashSet;

use crate::common::AdventResult;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    value: i32,
}

fn parse_instruction(line: &str) -> AdventResult<Instruction> {
    let line = line.trim();

    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    assert_eq!(parts.len(), 2);

    let value: i32 = parts[1].parse()?;

    let kind = match parts[0] {
        "nop" => InstructionKind::Nop,
        "acc" => InstructionKind::Acc,
        "jmp" => InstructionKind::Jmp,
        _ => panic!("Unknown instruction"),
    };
    Ok(Instruction { kind, value })
}

fn read_instructions(path: &str) -> AdventResult<Vec<Instruction>> {
    let content = std::fs::read_to_string(path)?;
    content.lines().map(parse_instruction).collect()
}

struct VM<'i> {
    instructions: &'i [Instruction],
    pc: usize,
    acc: i32,
    visited_pcs: HashSet<usize>,
}

impl<'i> VM<'i> {
    fn new(instructions: &'i [Instruction]) -> Self {
        VM {
            instructions,
            pc: 0,
            acc: 0,
            visited_pcs: HashSet::new(),
        }
    }

    fn is_finished(&self) -> bool {
        self.pc >= self.instructions.len()
    }

    fn part1_check(&self) -> Option<i32> {
        if self.visited_pcs.contains(&self.pc) {
            Some(self.acc)
        } else {
            None
        }
    }

    fn step(&mut self) {
        self.visited_pcs.insert(self.pc);

        let inst = self.instructions[self.pc];
        match inst.kind {
            InstructionKind::Nop => {
                self.pc += 1;
            }
            InstructionKind::Acc => {
                self.acc += inst.value;
                self.pc += 1;
            }
            InstructionKind::Jmp => {
                let offset = inst.value;
                if offset < 0 {
                    self.pc -= (-offset) as usize;
                } else {
                    self.pc += offset as usize;
                }
            }
        }
    }
}

fn vm_run(instructions: &[Instruction]) -> Result<i32, i32> {
    let mut vm = VM::new(instructions);
    while !vm.is_finished() {
        if let Some(acc) = vm.part1_check() {
            return Err(acc);
        }
        vm.step();
    }
    Ok(vm.acc)
}

fn part2_search(instructions: &[Instruction]) -> Option<i32> {
    for (index, inst) in instructions.iter().enumerate() {
        let replacement_kind = match inst.kind {
            InstructionKind::Nop => Some(InstructionKind::Jmp),
            InstructionKind::Jmp => Some(InstructionKind::Nop),
            _ => None,
        };

        let replacement = replacement_kind.map(|kind| Instruction {
            kind,
            value: inst.value,
        });

        if let Some(replacement) = replacement {
            let mut copy = instructions.to_vec();
            copy[index] = replacement;
            let new_res = vm_run(&copy);
            if let Ok(acc) = new_res {
                return Some(acc);
            }
        }
    }
    None
}

pub fn run(path: &str) {
    let instructions = read_instructions(path).expect("Cannot read instructions");

    let part1_res = vm_run(&instructions);
    match part1_res {
        Ok(acc) => {
            println!("day8 part1: {} program halted", acc);
        }
        Err(acc) => {
            println!("day8 part1: {} infinite loop", acc);
        }
    }

    if let Some(acc) = part2_search(&instructions) {
        println!("day8 part2: {}", acc);
    } else {
        println!("day8 part2: no solution found");
    }
}
