use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let groups = Group::from_file("input");
    println!(
        "{}",
        groups.iter().map(|group| group.num_unique()).sum::<usize>()
    );
    println!(
        "{}",
        groups.iter().map(|group| group.num_agree()).sum::<usize>()
    );
}

struct Group {
    answers: String,
    size: usize,
}

impl Group {
    fn from_file(name: &str) -> Vec<Self> {
        BufReader::new(File::open(name).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .group_by(|line| line != "")
            .into_iter()
            .filter(|(key, _)| *key)
            .map(|(_, entries)| Group::new(entries))
            .collect()
    }

    fn new<I, T>(letters: I) -> Group
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let mut answers: String = String::new();
        let mut size = 0;
        for string in letters.into_iter() {
            size += 1;
            answers.push_str(&string.as_ref().chars().collect::<String>())
        }
        Group { answers, size }
    }

    fn num_unique(&self) -> usize {
        self.answers.chars().unique().count()
    }

    fn num_agree(&self) -> usize {
        self.answers
            .chars()
            .sorted()
            .group_by(|&letter| letter)
            .into_iter()
            .map(|(_, it)| it.count())
            .sorted_by(|x, y| Ord::cmp(y, x))
            .fold_while(0, |acc, next| match next == self.size {
                true => Continue(acc + 1),
                false => Done(acc),
            })
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_group() {
        let group = ["abcx", "abcy", "abcz"];
        assert_eq!(Group::new(group).num_unique(), 6);
    }

    #[test]
    fn parse() {
        let groups = Group::from_file("test-input");
        assert_eq!(groups[1].answers.len(), 3);
    }

    #[test]
    fn part1() {
        let groups = Group::from_file("test-input");
        assert_eq!(
            groups.iter().map(|group| group.num_unique()).sum::<usize>(),
            11
        )
    }

    #[test]
    fn part2() {
        let groups = Group::from_file("test-input");
        let results = [3, 0, 1, 1, 1];
        let mut idx = 0;
        for (real, expect) in groups.iter().map(|group| group.num_agree()).zip(results) {
            println!("idx: {}, group: {}", idx, groups[idx].answers);
            assert_eq!(real, expect);
            idx += 1;
        }
    }

    #[test]
    fn part2_sum() {
        let groups = Group::from_file("test-input");
        assert_eq!(
            groups.iter().map(|group| group.num_agree()).sum::<usize>(),
            6
        )
    }
}
