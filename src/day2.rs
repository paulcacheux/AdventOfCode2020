use crate::common::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref LINE_RE: Regex =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>\w):\s(?P<password>\w+)").unwrap();
}

#[derive(Debug, Clone)]
struct PasswordLine {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl PasswordLine {
    fn is_valid_part1(&self) -> bool {
        let mut counter = 0;
        for c in self.password.chars() {
            if c == self.letter {
                counter += 1;
            }
        }
        self.min <= counter && counter <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        let is_min = chars[self.min as usize - 1] == self.letter;
        let is_max = chars[self.max as usize - 1] == self.letter;
        is_min ^ is_max
    }
}

fn read_password_line(line: &str) -> Option<PasswordLine> {
    if let Some(captures) = LINE_RE.captures(line) {
        Some(PasswordLine {
            min: captures["min"].parse().unwrap(),
            max: captures["max"].parse().unwrap(),
            letter: captures["letter"].chars().next().unwrap(),
            password: captures["password"].into(),
        })
    } else {
        None
    }
}

fn read_password_file(path: &str) -> AdventResult<Vec<PasswordLine>> {
    let content = std::fs::read_to_string(path)?;
    let lines = content
        .lines()
        .map(read_password_line)
        .map(Option::unwrap)
        .collect();
    Ok(lines)
}

pub fn run(path: &str) {
    let rows = read_password_file(path).expect("Cannot read input file");

    let part1 = rows.iter().filter(|pl| pl.is_valid_part1()).count();
    println!("day2 part1: {}", part1);

    let part2 = rows.iter().filter(|pl| pl.is_valid_part2()).count();
    println!("day2 part2: {}", part2);
}
