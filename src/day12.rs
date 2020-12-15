use crate::common::AdventResult;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn get_offsets(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            _ => panic!("Can't compute offsets"),
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            _ => panic!("Can't turn left"),
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            _ => panic!("Can't turn right"),
        }
    }

    fn turn_repeat(self, turn_fn: fn(Self) -> Self, angle: i32) -> Self {
        let count = angle / 90;
        let mut res = self;
        for _ in 0..count {
            res = turn_fn(res);
        }
        res
    }

    fn turn_left_angle(self, angle: i32) -> Self {
        self.turn_repeat(Direction::turn_left, angle)
    }

    fn turn_right_angle(self, angle: i32) -> Self {
        self.turn_repeat(Direction::turn_right, angle)
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl Instruction {
    fn read_from_line(line: &str) -> Self {
        let direction = match line.chars().next().expect("Cannot read direction") {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => panic!("Unknown direction"),
        };

        let value = line[1..].parse().expect("Cannot read count");
        Instruction { direction, value }
    }
}

trait State {
    fn new() -> Self;
    fn apply(&mut self, instruction: &Instruction);
    fn manhattan_distance(&self) -> i32;
}

#[derive(Debug, Clone, Copy)]
struct State1 {
    x: i32,
    y: i32,
    direction: Direction,
}

impl State for State1 {
    fn new() -> Self {
        State1 {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let current_direction = match instruction.direction {
            Direction::Left => {
                self.direction = self.direction.turn_left_angle(instruction.value);
                return;
            }
            Direction::Right => {
                self.direction = self.direction.turn_right_angle(instruction.value);
                return;
            }
            Direction::Forward => self.direction,
            other => other,
        };

        let (dx, dy) = current_direction.get_offsets();
        self.x += dx * instruction.value;
        self.y += dy * instruction.value;
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn part<S: State + std::fmt::Debug>(instructions: &[Instruction]) -> i32 {
    let mut state = S::new();
    for inst in instructions {
        state.apply(inst);
    }

    state.manhattan_distance()
}

#[derive(Debug, Clone, Copy)]
struct State2 {
    boat_x: i32,
    boat_y: i32,
    way_dx: i32,
    way_dy: i32,
    direction: Direction,
}

impl State for State2 {
    fn new() -> Self {
        State2 {
            boat_x: 0,
            boat_y: 0,
            way_dx: 10,
            way_dy: -1,
            direction: Direction::East,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::North | Direction::South | Direction::East | Direction::West => {
                let (dx, dy) = instruction.direction.get_offsets();
                self.way_dx += dx * instruction.value;
                self.way_dy += dy * instruction.value
            }
            Direction::Left => {
                let count = instruction.value / 90;
                for _ in 0..count {
                    let (nx, ny) = (self.way_dy, -self.way_dx);
                    self.way_dx = nx;
                    self.way_dy = ny;
                }
            }
            Direction::Right => {
                let count = instruction.value / 90;
                for _ in 0..count {
                    let (nx, ny) = (-self.way_dy, self.way_dx);
                    self.way_dx = nx;
                    self.way_dy = ny;
                }
            }
            Direction::Forward => {
                self.boat_x += self.way_dx * instruction.value;
                self.boat_y += self.way_dy * instruction.value;
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.boat_x.abs() + self.boat_y.abs()
    }
}

fn read_instructions(path: &str) -> AdventResult<Vec<Instruction>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(Instruction::read_from_line).collect())
}

pub fn run(path: &str) {
    let instructions = read_instructions(path).expect("Cannot read instructions");
    println!("day12 part1: {}", part::<State1>(&instructions));
    println!("day12 part2: {}", part::<State2>(&instructions));
}
