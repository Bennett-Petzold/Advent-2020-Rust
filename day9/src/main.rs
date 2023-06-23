use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;
use itertools::Itertools;

fn main() {
    let invalid = Xmas::new(file_to_i32("input"), 25).invalid().unwrap();
    println!("{}", invalid);
    let mut sequence = Xmas::new(file_to_i32("input"), 25)
        .series_sum(invalid)
        .unwrap();
    sequence.sort();
    println!("{}", sequence.first().unwrap() + sequence.last().unwrap());
}

struct Xmas {
    numbers: VecDeque<i32>,
    it: Box<dyn Iterator<Item = i32>>,
}

impl Xmas {
    fn new<I>(value: I, count: usize) -> Self
    where
        I: Iterator<Item = i32> + 'static,
    {
        let mut it = value;
        Xmas {
            numbers: it.by_ref().take(count).collect(),
            it: Box::new(it),
        }
    }
}

impl Xmas {
    fn invalid(mut self) -> anyhow::Result<i32> {
        for entry in self.it {
            let prev: Vec<_> = self.numbers.iter().sorted().cloned().collect();
            let mut found = false;
            for val in &prev[..prev.len() - 1] {
                if prev.binary_search(&(entry - val)).is_ok() {
                    self.numbers.pop_front();
                    self.numbers.push_back(entry);
                    found = true;
                    break;
                }
            }
            if !found {
                return Ok(entry);
            }
        }
        Err(anyhow!("The exchange is fully valid"))
    }

    fn series_sum(self, target: i32) -> anyhow::Result<Vec<i32>> {
        let mut num_it = self.numbers.iter().copied();
        let mut set: VecDeque<i32> = num_it.by_ref().take(2).collect();
        let mut full_it = num_it.chain(self.it);

        // Guaranteed to end as long as iterator is finite
        loop {
            match set.iter().sum::<i32>().cmp(&target) {
                Ordering::Equal => return Ok(Vec::from(set)),
                Ordering::Greater => {
                    set.pop_front();
                }
                Ordering::Less => set.push_back(match full_it.next() {
                    Some(x) => x,
                    None => return Err(anyhow!("Exhausted iterator with no matches")),
                }),
            }
        }
    }
}

fn file_to_i32(name: &str) -> impl Iterator<Item = i32> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let it = file_to_i32("test-input");
        let data = Xmas::new(it, 5);
        assert_eq!(data.invalid().unwrap(), 127);
    }

    #[test]
    fn part2() {
        let weakness = Xmas::new(file_to_i32("test-input"), 5).invalid().unwrap();
        let data = Xmas::new(file_to_i32("test-input"), 5);
        assert_eq!(data.series_sum(weakness).unwrap(), [15, 25, 47, 40]);
    }
}
