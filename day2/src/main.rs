use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let vals = PasswordEntry::from_file("input");
    println!("{}", PasswordEntry::count_valid(&vals));

    let new_vals = NewPasswordEntry::from_file("input");
    println!("{}", NewPasswordEntry::count_valid(&new_vals));
}

#[derive(Debug, PartialEq)]
struct PasswordEntry {
    min: usize,
    max: usize,
    required: char,
    password: String,
}

#[derive(Debug, PartialEq)]
struct NewPasswordEntry {
    pos1: usize,
    pos2: usize,
    required: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FORMAT: Regex = Regex::new(r#"(\d+)-(\d+) (.): (.+)"#).unwrap();
        }
        FORMAT
            .captures(s)
            .map(|caps| PasswordEntry {
                min: caps.get(1).unwrap().as_str().parse::<_>().unwrap(),
                max: caps.get(2).unwrap().as_str().parse::<_>().unwrap(),
                required: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                password: caps.get(4).unwrap().as_str().to_string(),
            })
            .ok_or(())
    }
}

impl PasswordEntry {
    fn from_file(name: &str) -> Vec<PasswordEntry> {
        BufReader::new(File::open(name).unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
            .collect()
    }

    fn validate(&self) -> bool {
        let mut count = 0;
        for letter in self.password.chars() {
            if letter == self.required {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }

    fn count_valid(set: &[PasswordEntry]) -> i32 {
        let mut count = 0;
        for pass in set {
            if pass.validate() {
                count += 1
            }
        }
        count
    }
}

impl NewPasswordEntry {
    fn update(old: &PasswordEntry) -> NewPasswordEntry {
        NewPasswordEntry {
            pos1: old.min - 1,
            pos2: old.max - 1,
            required: old.required,
            password: old.password.clone(),
        }
    }

    fn from_file(name: &str) -> Vec<NewPasswordEntry> {
        PasswordEntry::from_file(name)
            .iter()
            .map(NewPasswordEntry::update)
            .collect()
    }

    fn validate(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        chars[self.pos1] != chars[self.pos2]
            && (chars[self.pos1] == self.required || chars[self.pos2] == self.required)
    }

    fn count_valid(set: &[NewPasswordEntry]) -> i32 {
        let mut count = 0;
        for pass in set {
            if pass.validate() {
                count += 1
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_parse() {
        let vals = PasswordEntry::from_file("test-input");
        assert_eq!(
            vals[0],
            PasswordEntry {
                min: 1,
                max: 3,
                required: 'a',
                password: "abcde".to_string()
            }
        )
    }

    #[test]
    fn part1_single() {
        let vals = PasswordEntry::from_file("test-input");
        assert!(vals[0].validate());
    }

    #[test]
    fn part1_all() {
        let vals = PasswordEntry::from_file("test-input");
        assert_eq!(PasswordEntry::count_valid(&vals), 2);
    }

    #[test]
    fn part2_single() {
        let vals = NewPasswordEntry::from_file("test-input");
        assert!(vals[0].validate());
    }

    #[test]
    fn part2_all() {
        let vals = NewPasswordEntry::from_file("test-input");
        assert_eq!(NewPasswordEntry::count_valid(&vals), 1);
    }
}
