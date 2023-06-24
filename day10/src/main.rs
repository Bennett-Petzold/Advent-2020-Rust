use adapters::*;

fn main() {
    let listing = Adapters::from_file("input").unwrap();
    println!("{:#?}", listing.differences().product());
    println!("{}", listing.num_arrangements());
}

mod adapters {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        iter,
    };

    use itertools::Itertools;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Adapters {
        adapters: Vec<u32>,
    }

    #[derive(Debug, PartialEq)]
    pub struct JoltageDiff {
        pub one: u32,
        pub three: u32,
    }

    impl JoltageDiff {
        pub fn product(&self) -> u32 {
            self.one * self.three
        }
    }

    impl<I> From<I> for Adapters
    where
        I: IntoIterator<Item = u32>,
    {
        fn from(value: I) -> Self {
            let mut adapters: Vec<_> = iter::once(0).chain(value.into_iter().sorted()).collect();
            adapters.push(adapters.last().unwrap() + 3);
            Adapters { adapters }
        }
    }

    impl Adapters {
        pub fn from_file(name: &str) -> anyhow::Result<Self> {
            Ok(Self::from(
                BufReader::new(File::open(name).unwrap())
                    .lines()
                    .map(|line| -> anyhow::Result<u32> { Ok(line?.parse::<u32>()?) })
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ))
        }

        pub fn differences(&self) -> JoltageDiff {
            let mut diffs = JoltageDiff { one: 0, three: 0 };
            for (first, second) in self.adapters.iter().zip(self.adapters.iter().skip(1)) {
                match second - first {
                    1 => diffs.one += 1,
                    3 => diffs.three += 1,
                    _ => (),
                }
            }
            diffs
        }

        pub fn num_arrangements(&self) -> usize {
            let mut counts: HashMap<u32, usize> =
                self.adapters.iter().cloned().map(|x| (x, 0)).collect();
            *counts.get_mut(&0).unwrap() = 1;
            for adapter in &self.adapters {
                let num = *counts.get(adapter).unwrap();
                for idx in adapter + 1..adapter + 4 {
                    if let Some(count) = counts.get_mut(&idx) {
                        *count += num;
                    }
                }
            }
            *counts.get(self.adapters.last().unwrap()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let diffs = Adapters::from_file("test-input").unwrap().differences();
        assert_eq!(diffs, JoltageDiff { one: 22, three: 10 });
    }

    #[test]
    fn part2_small() {
        assert_eq!(
            Adapters::from_file("test-input-small")
                .unwrap()
                .num_arrangements(),
            8
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Adapters::from_file("test-input")
                .unwrap()
                .num_arrangements(),
            19208
        );
    }
}
