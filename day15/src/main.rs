use std::{collections::HashMap, fs::read_to_string, num::ParseIntError};

use anyhow::anyhow;

fn main() {
    println!(
        "{}",
        NumberGame::from_file("input").unwrap().progress_to(2020)
    );
    println!(
        "{}",
        NumberGame::from_file("input")
            .unwrap()
            .progress_to(30000000)
    );
}

#[derive(Debug, Clone)]
enum GameValue {
    Empty,
    Single(usize),
    Double(usize, usize),
}

impl GameValue {
    fn append(&mut self, next: usize) -> Self {
        *self = match self {
            Self::Empty => Self::Single(next),
            Self::Single(x) => Self::Double(*x, next),
            Self::Double(_, x) => Self::Double(*x, next),
        };
        self.clone()
    }

    fn next(&self) -> anyhow::Result<usize> {
        match self {
            Self::Empty => Err(anyhow!("Empty is an invalid state")),
            Self::Single(_) => Ok(0),
            Self::Double(x, y) => Ok(y - x),
        }
    }

    fn contains(&self, value: usize) -> bool {
        match self {
            Self::Empty => false,
            Self::Single(x) => *x == value,
            Self::Double(x, y) => (*x == value) || (*y == value),
        }
    }
}

#[derive(Debug, Default)]
struct NumberGame {
    cur_turn: usize,
    turns: HashMap<usize, GameValue>,
}

impl FromIterator<usize> for NumberGame {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let iter = iter.into_iter().collect::<Vec<_>>().into_iter();
        Self {
            cur_turn: iter.clone().len(),
            turns: iter
                .into_iter()
                .enumerate()
                .map(|(idx, val)| (val, GameValue::Single(idx)))
                .collect(),
        }
    }
}

impl NumberGame {
    fn from_str<T: AsRef<str>>(values: T) -> Result<Self, ParseIntError> {
        values
            .as_ref()
            .split(',')
            .map(|val| val.parse::<usize>())
            .collect::<Result<NumberGame, ParseIntError>>()
    }

    fn from_file<T: AsRef<str>>(name: T) -> anyhow::Result<Self> {
        Ok(Self::from_str(read_to_string(name.as_ref())?.trim())?)
    }

    fn add_entry(&mut self, val: usize, turn: usize) -> GameValue {
        self.turns
            .entry(val)
            .or_insert(GameValue::Empty)
            .append(turn)
    }

    fn progress_to(&mut self, target: usize) -> usize {
        let mut last_value = *self
            .turns
            .iter()
            .find(|(_, value)| value.contains(self.cur_turn - 1))
            .unwrap()
            .0;
        (self.cur_turn..target).for_each(|turn| {
            last_value = self.add_entry(last_value, turn).next().unwrap();
        });
        last_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            NumberGame::from_file("test-input")
                .unwrap()
                .progress_to(2020),
            436
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            NumberGame::from_file("test-input")
                .unwrap()
                .progress_to(30000000),
            175594
        );
    }
}
