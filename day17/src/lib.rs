use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use sorted_vec::SortedVec;

pub mod four_dim;
pub mod three_dim;

#[derive(Debug)]
pub struct Dimension<T: Point> {
    active: SortedVec<T>,
}

pub trait Point: Clone + Copy + PartialOrd + PartialEq + Eq + Ord {
    /// Constructs a point with zero on any further axes
    fn from_2_d(x: isize, y: isize) -> Self;
    /// Returns all points that differ by at most 1 on any given axis
    fn neighbors(&self) -> Vec<Self>;
}

impl<T: Point> Dimension<T> {
    pub fn from_file(filename: &str) -> Self {
        BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }
}

impl<T: AsRef<str>, P: Point> FromIterator<T> for Dimension<P> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let active: Vec<_> = iter
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.as_ref()
                    .chars()
                    .enumerate()
                    .filter_map(|(x, val)| {
                        if val == '#' {
                            Some(Point::from_2_d(x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self {
            active: Into::into(active),
        }
    }
}

impl<T: Point> Dimension<T> {
    pub fn cycle(&mut self) {
        self.active = self
            .active
            .iter()
            .flat_map(|point| point.neighbors())
            .sorted() // group_by only groups consecutive runs
            .group_by(|neighbor| *neighbor)
            .into_iter()
            .map(|(value, it)| (value, it.count()))
            .filter(|(value, count)| match count {
                3 => true,
                2 if self.active.binary_search(value).is_ok() => true,
                _ => false,
            })
            .map(|(value, _)| value)
            .collect_vec()
            .into();
    }

    pub fn cycle_count(&mut self, count: usize) {
        for _ in 0..count {
            self.cycle();
        }
    }

    pub fn num_active(&self) -> usize {
        self.active.len()
    }
}
