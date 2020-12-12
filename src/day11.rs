use crate::common::AdventResult;
use std::fmt;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Floor,
    OccupiedSeat,
    EmptySeat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn read_from_path(path: &str) -> AdventResult<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in reader.lines() {
            let line = line?;

            width = 0;
            for c in line.chars() {
                let cell = match c {
                    'L' => Cell::EmptySeat,
                    '.' => Cell::Floor,
                    '#' => Cell::OccupiedSeat,
                    _ => panic!("Unknown cell char"),
                };

                cells.push(cell);
                width += 1;
            }

            height += 1;
        }

        Ok(Map {
            cells,
            width,
            height,
        })
    }

    fn assert_in_bounds(&self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.assert_in_bounds(x, y);
        self.cells[y * self.width + x] = cell;
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.assert_in_bounds(x, y);
        self.cells[y * self.width + x]
    }

    fn get_number_of_inline_neighbors_occupied(&self, x: usize, y: usize) -> usize {
        let mut counter = 0;

        for &dy in &[-1, 0, 1] {
            'inner_main: for &dx in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                for coeff in 1.. {
                    let nx = x as isize + dx * coeff;
                    let ny = y as isize + dy * coeff;

                    if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                        continue 'inner_main;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    let cell = self.get_cell(nx, ny);
                    match cell {
                        Cell::Floor => continue,
                        Cell::OccupiedSeat => {
                            counter += 1;
                            break;
                        }
                        Cell::EmptySeat => {
                            break;
                        }
                    }
                }
            }
        }

        counter
    }

    fn get_number_of_direct_neighbors_occupied(&self, x: usize, y: usize) -> usize {
        let mut counter = 0;

        for &dy in &[-1, 0, 1] {
            for &dx in &[-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                let cell = self.get_cell(nx, ny);
                if let Cell::OccupiedSeat = cell {
                    counter += 1;
                }
            }
        }

        counter
    }

    fn next_step(&self, counter: fn(&Map, usize, usize) -> usize, threshold: usize) -> Map {
        let mut new_map = Map {
            cells: vec![Cell::Floor; self.width * self.height],
            ..*self
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let near_count = counter(self, x, y);
                let current_cell = self.get_cell(x, y);

                let next_cell = match (current_cell, near_count) {
                    (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
                    (Cell::OccupiedSeat, count) if count >= threshold => Cell::EmptySeat,
                    (cell, _) => cell,
                };

                new_map.set_cell(x, y, next_cell);
            }
        }

        new_map
    }

    fn next_step_part1(&self) -> Map {
        self.next_step(Map::get_number_of_direct_neighbors_occupied, 4)
    }

    fn next_step_part2(&self) -> Map {
        self.next_step(Map::get_number_of_inline_neighbors_occupied, 5)
    }

    fn count_occupied_stables(&self) -> usize {
        self.cells
            .iter()
            .filter(|&&cell| cell == Cell::OccupiedSeat)
            .count()
    }

    fn step_until(&self, next_fn: fn(&Map) -> Map) -> Self {
        let mut prec = self.clone();
        loop {
            let current = next_fn(&prec);

            if current == prec {
                return current;
            }

            prec = current;
        }
    }

    fn step_until_stable_part1(&self) -> Self {
        self.step_until(Map::next_step_part1)
    }

    fn step_until_stable_part2(&self) -> Self {
        self.step_until(Map::next_step_part2)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get_cell(x, y) {
                    Cell::Floor => '.',
                    Cell::OccupiedSeat => '#',
                    Cell::EmptySeat => 'L',
                };
                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run(path: &str) {
    let map = Map::read_from_path(path).expect("Cannot read map");
    let stable1 = map.step_until_stable_part1();
    println!("day11 part1: {}", stable1.count_occupied_stables());

    let stable2 = map.step_until_stable_part2();
    println!("day11 part2: {}", stable2.count_occupied_stables());
}
