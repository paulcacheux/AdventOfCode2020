use crate::common::*;
use core::panic;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default, Clone)]
struct PassportInfo {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

macro_rules! part2_check {
    ($field:expr, $checker:expr) => {
        match &$field {
            None => return false,
            Some(value) if !$checker(value) => return false,
            _ => {}
        }
    };
}

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

impl PassportInfo {
    fn is_valid_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_part2(&self) -> bool {
        fn year_check_builder(min: u32, max: u32) -> impl Fn(&str) -> bool {
            move |input: &str| match input.parse::<u32>() {
                Ok(value) => min <= value && value <= max,
                Err(_) => false,
            }
        }

        fn regex_builder(regex: &'static Regex) -> impl Fn(&str) -> bool {
            move |input: &str| regex.is_match(input)
        }

        fn hgt_checker(hgt: &str) -> bool {
            if let Some(captures) = HGT_RE.captures(hgt) {
                let value = &captures[1];
                let unit = &captures[2];

                match (value.parse::<u32>(), unit) {
                    (Ok(value), "cm") => 150 <= value && value <= 193,
                    (Ok(value), "in") => 59 <= value && value <= 76,
                    _ => false,
                }
            } else {
                false
            }
        }

        part2_check!(self.byr, year_check_builder(1920, 2002));
        part2_check!(self.iyr, year_check_builder(2010, 2020));
        part2_check!(self.eyr, year_check_builder(2020, 2030));
        part2_check!(self.hgt, hgt_checker);
        part2_check!(self.hcl, regex_builder(&HCL_RE));
        part2_check!(self.ecl, regex_builder(&ECL_RE));
        part2_check!(self.pid, regex_builder(&PID_RE));

        true
    }
}

fn read_pairs<'a>(lines: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut pairs = Vec::new();
    for line in lines {
        for part in line.split_ascii_whitespace() {
            let raw_pair: Vec<_> = part.split(':').collect();
            assert_eq!(raw_pair.len(), 2);
            pairs.push((raw_pair[0], raw_pair[1]))
        }
    }
    pairs
}

fn read_passport(lines: &[&str]) -> PassportInfo {
    let mut info = PassportInfo::default();

    for (key, value) in read_pairs(lines) {
        let field = match key {
            "byr" => &mut info.byr,
            "iyr" => &mut info.iyr,
            "eyr" => &mut info.eyr,
            "hgt" => &mut info.hgt,
            "hcl" => &mut info.hcl,
            "ecl" => &mut info.ecl,
            "pid" => &mut info.pid,
            "cid" => &mut info.cid,
            _ => panic!("Unknown field"),
        };
        *field = Some(value.into());
    }

    info
}

fn split_blocks(content: &str) -> Vec<Vec<&str>> {
    let mut blocks = Vec::new();
    let mut current_block = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() && !current_block.is_empty() {
            blocks.push(current_block);
            current_block = Vec::new();
        } else {
            current_block.push(line);
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }
    blocks
}

fn read_passports(path: &str) -> AdventResult<Vec<PassportInfo>> {
    let content = std::fs::read_to_string(path)?;

    let passports = split_blocks(&content)
        .iter()
        .map(|block| read_passport(block))
        .collect();
    Ok(passports)
}

pub fn run(path: &str) {
    let passports = read_passports(path).expect("Cannot read passports");

    let part1_count = passports.iter().filter(|pi| pi.is_valid_part1()).count();
    println!("day4 part1: {}", part1_count);

    let part2_count = passports.iter().filter(|pi| pi.is_valid_part2()).count();
    println!("day4 part2: {}", part2_count);
}
