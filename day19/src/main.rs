use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    println!("{}", Rule::count_matches_from_file("input", 0).unwrap());
}

#[derive(Debug, PartialEq)]
struct RuleSet {
    set: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
struct RuleOptions {
    options: Vec<RuleSet>,
}

#[derive(Debug, PartialEq)]
enum Rule {
    Value(char),
    SubRules(RuleOptions),
    Number(usize),
}

impl Rule {
    fn from_str(s: &str) -> anyhow::Result<(usize, Self)> {
        lazy_static! {
            static ref VALUE: Regex = Regex::new(r#"(\d+): "([[:alpha:]])""#).unwrap();
            static ref SUB_RULES: Regex = Regex::new(r#"(\d+): (.+)"#).unwrap();
        }
        match VALUE.captures(s) {
            Some(caps) => Ok((
                caps.get(1).unwrap().as_str().parse()?,
                Self::Value(caps.get(2).unwrap().as_str().parse()?),
            )),
            None => {
                let caps = SUB_RULES
                    .captures(s)
                    .ok_or(anyhow!("Invalid format: {s}"))?;
                Ok((
                    caps.get(1).unwrap().as_str().parse()?,
                    Self::SubRules(RuleOptions::from_str(caps.get(2).unwrap().as_str())?),
                ))
            }
        }
    }

    fn from_lines<I, T>(it: &mut I) -> anyhow::Result<HashMap<usize, Self>>
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        it.map(|s| Self::from_str(s.as_ref())).collect()
    }

    /// Returns if the number of characters used if rule matched
    fn inner_verify(
        &self,
        line: &[char],
        rules: &HashMap<usize, Self>,
    ) -> anyhow::Result<Option<usize>> {
        println!("{}, {:?}", line.iter().collect::<String>(), self);
        match self {
            Self::Value(check) => {
                if let Some(character) = line.first() {
                    if character == check {
                        Ok(Some(1))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            Self::Number(x) => rules
                .get(x)
                .ok_or(anyhow!("Invalid index: {x}"))?
                .inner_verify(line, rules),
            Self::SubRules(or_pairs) => {
                for pair in &or_pairs.options {
                    println!("\tPAIR {:?}", pair);
                    let mut idx = 0;
                    let mut all_valid = true;

                    for rule in &pair.set {
                        println!("\tRULE {:?}", rule);
                        match rule.inner_verify(&line[idx..], rules) {
                            Ok(Some(x)) => idx += x,
                            Err(_) | Ok(None) => {
                                all_valid = false;
                                break;
                            }
                        }
                    }

                    if all_valid {
                        return Ok(Some(idx));
                    };
                }
                Ok(None)
            }
        }
    }

    // Returns if line matches given rule
    fn verify(&self, line: &[char], rules: &HashMap<usize, Self>) -> anyhow::Result<bool> {
        if let Some(verified_len) = self.inner_verify(line, rules)? {
            println!("PASSED: {:?}", line.iter().collect::<String>());
            Ok(verified_len == line.len())
        } else {
            Ok(false)
        }
    }

    fn count_matches_from_file(filename: &str, target_idx: usize) -> anyhow::Result<usize> {
        let mut lines = BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|line| line.unwrap());
        let rules = Rule::from_lines(&mut lines.by_ref().take_while(|line| !line.is_empty()))?;
        let target_rule = rules
            .get(&0)
            .ok_or(anyhow!("No target rule: {target_idx}"))?;
        Ok(lines
            .map(|line| target_rule.verify(&line.chars().collect::<Vec<char>>(), &rules))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .filter(|&res| *res)
            .count())
    }
}

impl FromStr for RuleOptions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            options: s
                .split(" | ")
                .map(RuleSet::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromStr for RuleSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            set: s
                .split(' ')
                .filter(|line| !line.is_empty())
                .map(|num| Ok(Rule::Number(num.parse::<usize>()?)))
                .collect::<Result<Vec<_>, <usize as std::str::FromStr>::Err>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value() {
        assert_eq!(Rule::from_str(r#"1: "a""#).unwrap(), (1, Rule::Value('a')));
    }

    #[test]
    fn parse_nested() {
        assert_eq!(
            Rule::from_str("0: 1 2").unwrap(),
            (
                0,
                Rule::SubRules(RuleOptions {
                    options: vec![RuleSet {
                        set: vec![Rule::Number(1), Rule::Number(2)]
                    }]
                })
            )
        );
    }

    #[test]
    fn parse_or() {
        assert_eq!(
            Rule::from_str("1: 2 3 | 3 2").unwrap(),
            (
                1,
                Rule::SubRules(RuleOptions {
                    options: vec![
                        RuleSet {
                            set: vec![Rule::Number(2), Rule::Number(3)],
                        },
                        RuleSet {
                            set: vec![Rule::Number(3), Rule::Number(2)],
                        }
                    ]
                })
            )
        );
    }

    #[test]
    fn part1() {
        assert_eq!(Rule::count_matches_from_file("test-input", 0).unwrap(), 2);
    }

    #[test]
    fn part2_no_mod() {
        assert_eq!(
            Rule::count_matches_from_file("test-input-2-no-mod", 0).unwrap(),
            3
        );
    }

    #[test]
    fn part_2_single() {
        assert_eq!(
            Rule::count_matches_from_file("test-input-2-single", 0).unwrap(),
            1
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Rule::count_matches_from_file("test-input-2", 0).unwrap(),
            12
        );
    }
}
