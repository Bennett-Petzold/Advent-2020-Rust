use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;
use range_div::RangeDiv;

fn main() {
    let passes = BoardingPass::from_file("input");
    println!("{}", passes.iter().map(|pass| pass.id).max().unwrap());

    println!("{}", BoardingPass::find_missing(&passes).unwrap());
}

mod range_div {
    pub struct RangeDiv {
        top: u32,
        bottom: u32,
    }

    impl RangeDiv {
        pub fn new(bottom: u32, top: u32) -> Self {
            Self { top, bottom }
        }

        #[inline(always)]
        fn diff(&self) -> u32 {
            //println!("Top: {}, Bottom: {}", self.top, self.bottom);
            ((self.top + 1) - self.bottom) / 2
        }

        pub fn higher(&mut self) {
            //println!("Bottom: {}, Diff: {}", self.bottom, self.diff());
            self.bottom += self.diff()
        }

        pub fn lower(&mut self) {
            //println!("Top: {}, Diff: {}", self.top, self.diff());
            self.top -= self.diff()
        }

        pub fn pos(&self) -> Result<u32, String> {
            if self.top == self.bottom {
                Ok(self.top)
            } else {
                Err(format!(
                    "Top != Bottom, Top: {}, Bottom: {}",
                    self.top, self.bottom
                ))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u32,
    column: u32,
    id: u32,
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();

        let mut row = RangeDiv::new(0, 127);
        for mov in it.by_ref().take(7) {
            match mov {
                'F' => row.lower(),
                'B' => row.higher(),
                mov => return Err(format!("Invalid row character: {}", mov)),
            }
        }

        let mut col = RangeDiv::new(0, 7);
        for mov in it.by_ref().take(3) {
            match mov {
                'L' => col.lower(),
                'R' => col.higher(),
                mov => return Err(format!("Invalid column character: {}", mov)),
            }
        }

        match (row.pos(), col.pos()) {
            (Ok(row), Ok(col)) => Ok(BoardingPass::new(row, col)),
            (Err(row), Err(col)) => Err(format!("Both failed. Row -> {}; Column -> {}", row, col)),
            (Err(row), _) => Err(format!("Row failed. Row -> {}", row)),
            (_, Err(col)) => Err(format!("Col failed. Col -> {}", col)),
        }
    }
}

impl BoardingPass {
    fn new(row: u32, column: u32) -> Self {
        BoardingPass {
            row,
            column,
            id: (row * 8) + column,
        }
    }

    fn from_file(name: &str) -> Vec<Self> {
        BufReader::new(File::open(name).unwrap())
            .lines()
            .map(|line| BoardingPass::from_str(&line.unwrap()).unwrap())
            .collect()
    }

    fn find_missing(passes: &[BoardingPass]) -> Option<u32> {
        let sorted_ids = passes.iter().map(|pass| pass.id).sorted();
        let gap_pair = sorted_ids
            .clone()
            .zip(sorted_ids.skip(1))
            .find(|(prev, cur)| cur - prev == 2);
        match gap_pair {
            Some((_, cur)) => Some(cur - 1),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        let test_set = [
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ];
        for test in test_set {
            assert_eq!(
                BoardingPass::from_str(test.0).unwrap(),
                BoardingPass {
                    row: test.1,
                    column: test.2,
                    id: test.3
                }
            )
        }
    }

    #[test]
    fn parse() {
        assert_eq!(
            BoardingPass::from_file("test-input")[0],
            BoardingPass {
                row: 44,
                column: 5,
                id: 357
            }
        )
    }
}
