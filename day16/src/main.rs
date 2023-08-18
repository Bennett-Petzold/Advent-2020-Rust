use anyhow::{anyhow, bail};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

fn main() {
    let mut notes = Notes::new("input").unwrap();
    println!("{}", notes.sum_nearby_invalid());
    lazy_static! {
        static ref DEPARTURE: Regex = Regex::new(r#"^departure"#).unwrap();
    }
    let departure_product: usize = notes
        .find_rule_poses()
        .unwrap()
        .iter()
        .filter_map(|(name, val)| {
            if DEPARTURE.is_match(name) {
                Some(val)
            } else {
                None
            }
        })
        .map(|&val| notes.self_ticket.values[val])
        .product();
    println!("{departure_product}");
}

#[derive(Debug, PartialEq, Clone)]
struct TicketRule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

#[derive(Debug, PartialEq)]
struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug)]
struct Notes {
    rules: Vec<TicketRule>,
    self_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for TicketRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RULE: Regex = Regex::new(r#"(.+): ((?:\d+-\d+(?: or )?)+)"#).unwrap();
        }
        let captures = RULE
            .captures(s)
            .ok_or(anyhow!("Incorrect ticket rule formatting"))?;
        let name = captures
            .get(1)
            .ok_or(anyhow!("Incorrect ticket rule formatting"))?
            .as_str()
            .to_string();
        let body = captures
            .get(2)
            .ok_or(anyhow!("Incorrect ticket rule formatting"))?
            .as_str();
        Ok(Self {
            name,
            ranges: body
                .split(" or ")
                .map(|pair| {
                    pair.split('-')
                        .map(|num| num.parse::<usize>())
                        .collect::<Result<Vec<_>, ParseIntError>>()?
                        .into_iter()
                        .collect_tuple()
                        .ok_or(anyhow!("Pair not split by -"))
                })
                .collect::<Result<Vec<_>, Self::Err>>()?
                .into_iter()
                .map(|(start, end)| start..=end)
                .collect(),
        })
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .split(',')
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?,
        })
    }
}

impl Notes {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut lines_it = BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|line| line.unwrap());

        let rules = lines_it
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .map(|line| TicketRule::from_str(&line))
            .collect::<Result<Vec<_>, _>>()?;

        if lines_it.by_ref().next().ok_or(anyhow!("Input too short"))? != "your ticket:" {
            bail!("Missing \"your ticket\" field")
        }

        let self_ticket =
            Ticket::from_str(&lines_it.by_ref().next().ok_or(anyhow!("Input too short"))?)?;

        let _ = lines_it.next(); // Skip blank line
        if lines_it.by_ref().next().ok_or(anyhow!("Input too short"))? != "nearby tickets:" {
            bail!("Missing \"nearby tickets\" field")
        }

        let nearby_tickets = lines_it
            .map(|line| Ticket::from_str(&line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rules,
            self_ticket,
            nearby_tickets,
        })
    }
}

impl TicketRule {
    fn valid(&self, value: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

impl Ticket {
    fn count_invalid(&self, rules: &[TicketRule]) -> usize {
        self.values
            .iter()
            .filter(|&value| !rules.iter().any(|rule| rule.valid(*value)))
            .sum()
    }

    fn sum_invalid<'a, I>(tickets: I, rules: &[TicketRule]) -> usize
    where
        I: IntoIterator<Item = &'a Self>,
    {
        tickets
            .into_iter()
            .map(|ticket| ticket.count_invalid(rules))
            .sum()
    }

    fn valid(&self, rules: &[TicketRule]) -> bool {
        self.values
            .iter()
            .all(|&value| rules.iter().any(|rule| rule.valid(value)))
    }

    fn filter_valid(tickets: &mut Vec<Ticket>, rules: &[TicketRule]) {
        tickets.retain(|ticket| ticket.valid(rules))
    }
}

impl Notes {
    fn sum_nearby_invalid(&self) -> usize {
        Ticket::sum_invalid(self.nearby_tickets.iter(), &self.rules)
    }

    fn remove_invalid(&mut self) {
        Ticket::filter_valid(&mut self.nearby_tickets, &self.rules);
    }

    fn rule_pos(&self, rule: &TicketRule) -> Vec<usize> {
        let filter_fn = |(idx, field): (usize, &usize)| {
            if rule.valid(*field) {
                Some(idx)
            } else {
                None
            }
        };

        let mut candidates: Vec<_> = self
            .self_ticket
            .values
            .iter()
            .enumerate()
            .filter_map(filter_fn)
            .collect();

        let mut ticket_it = self.nearby_tickets.iter();

        while candidates.len() >= 2 {
            match ticket_it.next() {
                None => return candidates,
                Some(ticket) => {
                    candidates = ticket
                        .values
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| candidates.contains(idx))
                        .filter_map(filter_fn)
                        .collect();
                }
            }
        }

        candidates
    }

    fn find_rule_poses(&mut self) -> anyhow::Result<Vec<(String, usize)>> {
        self.remove_invalid();
        let mut possible: Vec<_> = self
            .rules
            .iter()
            .map(|rule| {
                (rule.name.clone(), self.rule_pos(rule))
                //    .ok_or(anyhow!("Rule ({:?}) has no valid position", rule))
            })
            .collect();
        let mut found = Vec::with_capacity(possible.len());
        let mut new_found = Vec::with_capacity(possible.len());
        while !possible.is_empty() {
            possible.retain(|possiblity| {
                if possiblity.1.len() == 1 {
                    new_found.push((possiblity.0.clone(), *possiblity.1.first().unwrap()));
                    false
                } else {
                    true
                }
            });
            possible.iter_mut().for_each(|(_, positions)| {
                positions.retain(|pos| new_found.iter().all(|(_, val)| val != pos))
            });
            found.append(&mut new_found);
        }
        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_parse() {
        assert_eq!(
            TicketRule::from_str("class: 1-3 or 5-7").unwrap(),
            TicketRule {
                name: "class".to_string(),
                ranges: vec![1..=3, 5..=7],
            }
        );
    }

    #[test]
    fn ticket_parse() {
        assert_eq!(
            Ticket::from_str("7,3,47").unwrap(),
            Ticket {
                values: vec![7, 3, 47]
            }
        );
    }

    #[test]
    fn ticket_test() {
        let rules = [
            TicketRule::from_str("class: 1-3 or 5-7").unwrap(),
            TicketRule::from_str("row: 6-11 or 33-44").unwrap(),
            TicketRule::from_str("seat: 13-40 or 45-50").unwrap(),
        ];
        let ticket = Ticket::from_str("55,2,20").unwrap();
        assert_eq!(Ticket::sum_invalid(&[ticket], &rules), 55);
    }

    #[test]
    fn part1() {
        let notes = Notes::new("test-input").unwrap();
        assert_eq!(notes.sum_nearby_invalid(), 71);
    }

    #[test]
    fn remove_invalid() {
        let mut notes = Notes::new("test-input").unwrap();
        notes.remove_invalid();
        assert_eq!(
            notes.nearby_tickets,
            vec![Ticket {
                values: vec![7, 3, 47]
            }]
        );
    }

    #[test]
    fn find_rule_poses() {
        let mut notes = Notes::new("test-input-2").unwrap();
        notes
            .find_rule_poses()
            .unwrap()
            .iter()
            .sorted()
            .zip([
                ("class".to_string(), 1),
                ("row".to_string(), 0),
                ("seat".to_string(), 2),
            ])
            .for_each(|(rule, expected)| assert_eq!(*rule, expected));
    }
}
