use std::{
    convert::TryFrom,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

fn main() {
    println!("{}", Seating::from_file("input").unwrap().occupied_stable());
    println!(
        "{}",
        Seating::from_file("input").unwrap().seen_occupied_stable()
    );
}

#[derive(Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Seating {
    seats: Vec<Vec<Seat>>,
}

impl TryFrom<char> for Seat {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(anyhow!("{} is not a seat code", value)),
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Occupied => write!(f, "#"),
        }
    }
}

impl Clone for Seat {
    fn clone(&self) -> Self {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => Self::Empty,
            Self::Occupied => Self::Occupied,
        }
    }
}

impl Seating {
    fn try_from<I, T>(value: I) -> Result<Self, anyhow::Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let seats: Result<Vec<_>, _> = value
            .into_iter()
            .map(|row| row.as_ref().chars().map(Seat::try_from).collect())
            .collect();
        Ok(Self { seats: seats? })
    }

    fn from_file(name: &str) -> Result<Self, anyhow::Error> {
        let file: Result<Vec<_>, _> = BufReader::new(File::open(name).unwrap()).lines().collect();
        Self::try_from(file?)
    }
}

impl Seating {
    fn adjacent_occupied(&self, row: usize, col: usize) -> usize {
        let adjacent_rows = row.saturating_sub(1)..row + 2;
        let adjacent_cols = col.saturating_sub(1)..col + 2;
        let adjacent = adjacent_rows
            .flat_map(|adj_row| adjacent_cols.clone().map(move |adj_col| (adj_row, adj_col)))
            .filter(|(adj_row, adj_col)| *adj_row != row || *adj_col != col);

        adjacent
            .filter(|(row, col)| {
                let row_val = self.seats.get(*row);
                match row_val {
                    Some(row_val) => *row_val.get(*col).unwrap_or(&Seat::Floor) == Seat::Occupied,
                    None => false,
                }
            })
            .count()
    }

    fn seen_occupied(&self, row: usize, col: usize) -> usize {
        const PROGRESSIONS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        PROGRESSIONS
            .iter()
            .filter(|(x, y)| {
                let mut opt_x = row.checked_add_signed(*x);
                let mut opt_y = col.checked_add_signed(*y);
                loop {
                    if let (Some(pos_x), Some(pos_y)) = (opt_x, opt_y) {
                        if let Some(row_val) = self.seats.get(pos_x) {
                            if let Some(seat) = row_val.get(pos_y) {
                                match seat {
                                    Seat::Occupied => {
                                        return true;
                                    }
                                    Seat::Empty => {
                                        return false;
                                    }
                                    Seat::Floor => {
                                        opt_x = pos_x.checked_add_signed(*x);
                                        opt_y = pos_y.checked_add_signed(*y);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    return false;
                }
            })
            .count()
    }

    fn update(
        &mut self,
        occupied_limit: usize,
        occupied: fn(&Self, usize, usize) -> usize,
    ) -> bool {
        let next = self
            .seats
            .iter()
            .enumerate()
            .map(|(row_num, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_num, col)| match col {
                        Seat::Empty if occupied(self, row_num, col_num) == 0 => Seat::Occupied,
                        Seat::Occupied if occupied(self, row_num, col_num) >= occupied_limit => {
                            Seat::Empty
                        }
                        x => x.clone(),
                    })
                    .collect()
            })
            .collect();

        let changed = !self.seats.eq(&next);
        self.seats = next;
        changed
    }

    fn occupied_stable(&mut self) -> u32 {
        while self.update(4, Self::adjacent_occupied) {}
        self.seats.iter().flatten().fold(0, |acc, seat| {
            acc + if *seat == Seat::Occupied { 1 } else { 0 }
        })
    }

    fn seen_occupied_stable(&mut self) -> u32 {
        while self.update(5, Self::seen_occupied) {}
        self.seats.iter().flatten().fold(0, |acc, seat| {
            acc + if *seat == Seat::Occupied { 1 } else { 0 }
        })
    }
}

impl Display for Seating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.seats {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::Seat::*;
    use super::*;

    #[test]
    fn parse() {
        let layout = Seating::from_file("test-input").unwrap();
        assert_eq!(
            layout.seats[0],
            [Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty]
        );
    }

    #[test]
    fn part1() {
        let mut layout = Seating::from_file("test-input").unwrap();
        let count = layout.occupied_stable();
        println!("{layout}");
        assert_eq!(count, 37);
    }

    #[test]
    fn part2() {
        let mut layout = Seating::from_file("test-input").unwrap();
        let count = layout.seen_occupied_stable();
        println!("{layout}");
        assert_eq!(count, 26);
    }
}
