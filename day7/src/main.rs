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
    let rules = BagRule::from_file("input").unwrap();
    println!("{}", BagRule::num_contain("shiny gold", &rules));
    println!(
        "{}",
        BagMap::from(rules).bags_contained("shiny gold").unwrap()
    );
}

#[derive(Debug)]
struct BagMap {
    hmap: HashMap<String, Box<[(u32, String)]>>,
}

#[derive(Debug, PartialEq, Clone)]
struct BagRule {
    name: String,
    contains: Box<[(u32, String)]>,
}

impl FromStr for BagRule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG: Regex = Regex::new(r#"(\w+ \w+) bags contain "#).unwrap();
            static ref CONTAINED: Regex = Regex::new(r#"(\d+) (\w+ \w+) bags?(?:,|\.)"#).unwrap();
        }

        let name = BAG
            .captures(s)
            .ok_or("Malformed input: no bag name")?
            .get(1)
            .ok_or("Malformed input: no bag name")?;

        let mut parse_sentinel: Result<(), Box<dyn Error>> = Ok(());
        let contains = CONTAINED
            .captures_iter(&s[name.end()..])
            .map_while(|cap| {
                let num = match cap[1].parse::<u32>() {
                    Ok(num) => num,
                    Err(e) => {
                        parse_sentinel = Err(Box::new(e));
                        return None;
                    }
                };
                Some((num, String::from(&cap[2])))
            })
            .collect();
        parse_sentinel?;

        Ok(Self {
            name: name.as_str().into(),
            contains,
        })
    }
}

impl BagRule {
    fn from_file(name: &str) -> Result<Vec<BagRule>, Box<dyn Error>> {
        let mut build_sentinel: Result<(), Box<dyn Error>> = Ok(());
        let bufs = BufReader::new(File::open(name).unwrap())
            .lines()
            .map_while(|line| match BagRule::from_str(&line.unwrap()) {
                Ok(rule) => Some(rule),
                Err(e) => {
                    build_sentinel = Err(e);
                    return None;
                }
            })
            .collect();
        build_sentinel?;
        Ok(bufs)
    }

    fn num_contain(name: &str, rules: &[BagRule]) -> usize {
        let mut names = vec![String::from(name)];
        let update_rules = |cur_rules: &[BagRule], remove: &[String]| -> Vec<BagRule> {
            cur_rules
                .iter()
                .filter(|rule| !(remove.contains(&rule.name)))
                .cloned()
                .collect()
        };

        let search = |cur_rules: &[BagRule], names: &[String]| -> Vec<String> {
            cur_rules
                .iter()
                .filter_map(|rule| {
                    match rule
                        .contains
                        .iter()
                        .filter(|(_, inside)| names.contains(&inside))
                        .next()
                    {
                        Some(_) => Some(String::from(&rule.name)),
                        None => None,
                    }
                })
                .collect()
        };

        let mut rules_remaining = update_rules(rules, &names);
        let mut found = search(&rules_remaining, &names);

        let mut count = 0;
        while found.len() > 0 {
            count += found.len();
            names.append(&mut found);
            rules_remaining = update_rules(&rules_remaining, &names);
            found.extend(search(&rules_remaining, &names));
        }
        count
    }
}

impl<I> From<I> for BagMap
where
    I: IntoIterator<Item = BagRule>,
{
    fn from(value: I) -> Self {
        BagMap {
            hmap: value
                .into_iter()
                .map(|rule| (rule.name, rule.contains))
                .collect(),
        }
    }
}

impl BagMap {
    fn bags_contained(&self, name: &str) -> Option<u32> {
        let mut sum = 0;
        for (count, name) in self.hmap.get(name)?.as_ref() {
            sum += count * (1 + self.bags_contained(name)?);
        }
        Some(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single() {
        let rule =
            BagRule::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap();
        assert_eq!(
            rule,
            BagRule {
                name: "light red".into(),
                contains: Box::new([
                    (1, String::from("bright white")),
                    (2, String::from("muted yellow"))
                ])
            }
        );
    }

    #[test]
    fn parse_file() {
        let rules = BagRule::from_file("test-input").unwrap();
        assert_eq!(
            rules[1],
            BagRule {
                name: "dark orange".into(),
                contains: Box::new([
                    (3, String::from("bright white")),
                    (4, String::from("muted yellow"))
                ]),
            }
        );
        assert_eq!(
            rules[7],
            BagRule {
                name: "faded blue".into(),
                contains: Box::new([]),
            }
        );
    }

    #[test]
    fn part1() {
        let rules = BagRule::from_file("test-input").unwrap();
        assert_eq!(BagRule::num_contain("shiny gold", &rules), 4);
    }

    #[test]
    fn part2() {
        let rules = BagRule::from_file("test-input-pt2").unwrap();
        assert_eq!(
            BagMap::from(rules).bags_contained("shiny gold").unwrap(),
            126
        );
    }
}
