use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let passes = &Passport::from_file("input");
    println!("{}", Passport::num_valid(passes));
    println!(
        "{}",
        passes
            .iter()
            .fold(0, |acc: u32, pass| if pass.check_all_rules().unwrap() {
                acc + 1
            } else {
                acc
            })
    );
}

struct Passport {
    fields: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FIELD: Regex = Regex::new(r#"([^\s]+):([^\s]+)"#).unwrap();
        }
        Ok(Passport {
            fields: FIELD
                .captures_iter(s)
                .map(|cap| (cap[1].to_string(), cap[2].to_string()))
                .collect(),
        })
    }
}

impl Passport {
    const REQ_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn from_file(s: &str) -> Vec<Passport> {
        BufReader::new(File::open(s).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .group_by(|line| line == "")
            .into_iter()
            .filter(|(blank, _)| !blank)
            .map(|(_, line)| line.reduce(|acc, x| format!("{} {}", acc, x)).unwrap())
            .map(|line| Passport::from_str(&line).unwrap())
            .collect()
    }

    fn valid(&self) -> bool {
        for req in Self::REQ_FIELDS {
            match self.fields.keys().find(|key| *key == req) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }

    fn num_valid(passes: &[Passport]) -> u32 {
        let mut count = 0;
        for pass in passes {
            if pass.valid() {
                count += 1
            }
        }
        count
    }

    fn check_all_rules(&self) -> Result<bool, Box<dyn Error>> {
        match self.valid() {
            true => {
                for (entry, value) in &self.fields {
                    if !Self::check_rule(entry, value)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            false => Ok(false),
        }
    }

    fn digit_check(value: &str, min: u32, max: u32) -> Result<bool, Box<dyn Error>> {
        lazy_static! {
            static ref NUMBER: Regex = Regex::new(r#"^\d{4}$"#).unwrap();
        }
        match NUMBER.is_match(value) {
            true => {
                let year = value.parse::<u32>()?;
                Ok(year >= min && year <= max)
            }
            false => Ok(false),
        }
    }

    fn check_rule(entry: &str, value: &str) -> Result<bool, Box<dyn Error>> {
        match entry {
            "byr" => Self::digit_check(value, 1920, 2002),
            "iyr" => Self::digit_check(value, 2010, 2020),
            "eyr" => Self::digit_check(value, 2020, 2030),
            "hgt" => {
                lazy_static! {
                    static ref PARTS: Regex = Regex::new(r#"^(\d+)((in)|(cm))$"#).unwrap();
                }
                match PARTS.captures(value) {
                    Some(cap) => match (cap[1].parse::<u32>()?, &cap[2]) {
                        (num, "cm") => Ok(num >= 150 && num <= 193),
                        (num, "in") => Ok(num >= 59 && num <= 76),
                        _ => Err("Invalid unit for hgt")?,
                    },
                    _ => Ok(false),
                }
            }
            "hcl" => {
                lazy_static! {
                    static ref PATTERN: Regex = Regex::new(r##"^#[0-9a-f]{6}$"##).unwrap();
                }
                Ok(PATTERN.is_match(value))
            }
            "ecl" => match value {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(true),
                _ => Ok(false),
            },
            "pid" => {
                lazy_static! {
                    static ref PATTERN: Regex = Regex::new(r#"^\d{9}$"#).unwrap();
                }
                Ok(PATTERN.is_match(value))
            }
            "cid" => Ok(true),
            _ => Err("Invalid entry")?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let pass = &Passport::from_file("test-input")[0];
        for (k, v) in &pass.fields {
            println!("{}: {}", k, v);
        }
        assert_eq!(pass.fields.get("ecl").unwrap(), "gry");
    }

    #[test]
    fn two_passes() {
        let passes = &Passport::from_file("test-input");

        println!("PASS 1");
        for (k, v) in &passes[0].fields {
            println!("{}: {}", k, v);
        }
        assert!(passes[0].valid());

        println!("PASS 2");
        for (k, v) in &passes[1].fields {
            println!("{}: {}", k, v);
        }
        assert!(!passes[1].valid());
    }

    #[test]
    fn part1() {
        let passes = &Passport::from_file("test-input");
        assert_eq!(Passport::num_valid(passes), 2)
    }

    #[test]
    fn invalid_part2() {
        for pass in Passport::from_file("invalid-test") {
            assert!(!pass.check_all_rules().unwrap());
        }
    }

    #[test]
    fn valid_part2() {
        for pass in Passport::from_file("valid-test") {
            assert!(pass.check_all_rules().unwrap());
        }
    }
}
