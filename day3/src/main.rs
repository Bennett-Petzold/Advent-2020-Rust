use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use deref_derive::Deref;

fn main() {
    println!("{}", Slope::from_file("input").descend(3, 1));
    println!(
        "{}",
        Slope::from_file("input")
            .all_descend()
            .iter()
            .product::<u64>()
    );
}

#[derive(Deref)]
struct SlopeLine {
    trees: Vec<bool>,
}

#[derive(Deref)]
struct Slope {
    lines: Vec<SlopeLine>,
}

impl FromStr for SlopeLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SlopeLine {
            trees: s.chars().map(|pos| pos == '#').collect(),
        })
    }
}

impl SlopeLine {
    fn at(&self, pos: usize) -> bool {
        self.trees[pos % self.trees.len()]
    }
}

impl Slope {
    fn from_file(name: &str) -> Slope {
        Slope {
            lines: BufReader::new(File::open(name).unwrap())
                .lines()
                .map(|line| line.unwrap().parse::<SlopeLine>().unwrap())
                .collect(),
        }
    }

    fn descend(&self, step: usize, slope_step: usize) -> u64 {
        let mut count = 0;
        let mut pos = 0;
        for line in self.lines.iter().step_by(slope_step) {
            if line.at(pos) {
                count += 1
            }
            pos += step
        }
        count
    }

    fn all_descend(self) -> Vec<u64> {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(horz, vert)| self.descend(*horz, *vert))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            *Slope::from_file("test-input")[0],
            vec![false, false, true, true, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn part1() {
        assert_eq!(Slope::from_file("test-input").descend(3, 1), 7);
    }

    #[test]
    fn part2() {
        let mut descents = Slope::from_file("test-input").all_descend();
        descents.sort();
        let mut expected = [2, 7, 3, 4, 2];
        expected.sort();
        assert_eq!(descents, expected);
        assert_eq!(descents.iter().product::<u64>(), 336);
    }
}
