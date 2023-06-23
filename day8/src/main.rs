use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::anyhow;

fn main() {
    println!(
        "{}",
        Console::from_file("input").unwrap().accumulator_at_repeat()
    );
    println!(
        "{}",
        Console::from_file("input")
            .unwrap()
            .accumulator_with_fix()
            .unwrap()
    );
}

#[derive(Debug, PartialEq, Clone)]
enum Code {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
struct Console {
    accumulator: isize,
    instructions: Vec<(Code, isize, bool)>,
}

impl FromStr for Code {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            "nop" => Ok(Self::Nop),
            _ => Err(()),
        }
    }
}

impl Console {
    fn from_file(name: &str) -> anyhow::Result<Self> {
        let mut sentinel: anyhow::Result<()> = Ok(());
        let lines = BufReader::new(File::open(name)?)
            .lines()
            .map_while(|line| match line {
                Ok(line) => Some(line),
                Err(e) => {
                    sentinel = Err(e.into());
                    None
                }
            });
        let result = Self::from_lines(lines);
        sentinel?;
        result
    }

    fn from_lines<I>(lines: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = String>,
    {
        let mut sentinel: anyhow::Result<()> = Ok(());
        let instructions: Vec<_> = lines
            .into_iter()
            .map_while(|line| match line.find(" ") {
                Some(space) => {
                    match (
                        Code::from_str(&line[..space]),
                        &line[space + 1..].parse::<isize>(),
                    ) {
                        (Ok(code), Ok(num)) => Some((code, num.to_owned(), false)),
                        (Err(_), Ok(_)) => {
                            sentinel = Err(anyhow!("Code malformed: {}", line));
                            None
                        }
                        (Ok(_), Err(_)) => {
                            sentinel = Err(anyhow!("Argument number malformed: {}", line));
                            None
                        }
                        _ => {
                            sentinel = Err(anyhow!("Code and/or number malformed: {}", line));
                            None
                        }
                    }
                }
                None => {
                    sentinel = Err(anyhow!("Line does not match expected pattern: {}", line));
                    None
                }
            })
            .collect();
        sentinel?;
        Ok(Self {
            accumulator: 0,
            instructions,
        })
    }
}

impl Console {
    fn accumulator_at_repeat(mut self) -> isize {
        let mut idx = 0;
        loop {
            let next_idx = match self.instructions[idx] {
                (_, _, true) => break,
                (Code::Acc, num, false) => {
                    self.accumulator += num;
                    idx + 1
                }
                (Code::Jmp, num, false) => {
                    usize::try_from(isize::try_from(idx).unwrap() + num).unwrap()
                }
                (Code::Nop, _, false) => idx + 1,
            };
            self.instructions[idx].2 = true;
            idx = next_idx;
        }
        self.accumulator
    }

    fn accumulator_with_fix(mut self) -> anyhow::Result<isize> {
        let mut idx = 0;
        while idx <= self.instructions.len() {
            let next_idx = match self.instructions[idx] {
                (_, _, true) => return Err(anyhow!("Infinite loop")),
                (Code::Acc, num, false) => {
                    self.accumulator += num;
                    idx + 1
                }
                (Code::Jmp, num, false) => {
                    let mut mod_copy = self.clone();
                    mod_copy.instructions[idx] = (Code::Nop, num, false);
                    if let Ok(acc) = Self::accumulator_terminate(mod_copy, idx.clone()) {
                        return Ok(acc);
                    }
                    usize::try_from(isize::try_from(idx)? + num)?
                }
                (Code::Nop, num, false) => {
                    let mut mod_copy = self.clone();
                    mod_copy.instructions[idx] = (Code::Jmp, num, false);
                    if let Ok(acc) = Self::accumulator_terminate(mod_copy, idx.clone()) {
                        return Ok(acc);
                    }
                    idx + 1
                }
            };
            self.instructions[idx].2 = true;
            idx = next_idx;
        }
        Ok(self.accumulator)
    }

    fn accumulator_terminate(mut self, mut idx: usize) -> anyhow::Result<isize> {
        while idx < self.instructions.len() {
            let next_idx = match self.instructions[idx] {
                (_, _, true) => return Err(anyhow!("Infinite loop")),
                (Code::Acc, num, false) => {
                    self.accumulator += num;
                    idx + 1
                }
                (Code::Jmp, num, false) => usize::try_from(isize::try_from(idx)? + num)?,
                (Code::Nop, _, false) => idx + 1,
            };
            self.instructions[idx].2 = true;
            idx = next_idx;
        }

        if idx == self.instructions.len() {
            Ok(self.accumulator)
        } else {
            Err(anyhow!("Jump past last instruction"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Console::from_file("test-input").unwrap().instructions[1],
            (Code::Acc, 1, false)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            Console::from_file("test-input")
                .unwrap()
                .accumulator_at_repeat(),
            5
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            Console::from_file("test-input")
                .unwrap()
                .accumulator_with_fix()
                .unwrap(),
            8
        )
    }
}
