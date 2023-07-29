use anyhow::{anyhow, bail};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    array,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    println!("{}", EmulatedMemory::from_file("input").unwrap().sum());
    println!("{}", EmulatedMemoryV2::from_file("input").unwrap().sum());
}

/// 36 bits wide, little endian integer
#[derive(Debug, Clone, Copy)]
struct EmulatedInt {
    bits: [bool; 36],
}

// Unused but matches spec
impl Default for EmulatedInt {
    fn default() -> Self {
        Self { bits: [false; 36] }
    }
}

impl FromStr for EmulatedInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: u64 = s.parse()?;
        let bits = array::from_fn(|idx| {
            let idx = 35 - idx;
            let pos = u64::pow(2, idx as u32);
            if num >= pos {
                num -= pos;
                true
            } else {
                false
            }
        });

        if num != 0 {
            bail!("{num} is larger than 36 bits");
        }

        Ok(Self { bits })
    }
}

impl From<&EmulatedInt> for u64 {
    fn from(value: &EmulatedInt) -> Self {
        value
            .bits
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, val)| if *val { u64::pow(2, idx as u32) } else { 0 })
            .sum()
    }
}

impl TryFrom<u64> for EmulatedInt {
    type Error = anyhow::Error;

    fn try_from(mut value: u64) -> Result<Self, Self::Error> {
        let bits = array::from_fn(|idx| {
            let idx = u64::pow(2, 35 - (idx as u32));
            if value >= idx {
                value -= idx;
                true
            } else {
                false
            }
        });

        if value != 0 {
            bail!("{value} cannot be truncated to 32 bits");
        }

        Ok(Self { bits })
    }
}

// -------------------- END STRUCT --------------------

#[derive(Debug)]
struct EmulatedMemory {
    map: HashMap<usize, EmulatedInt>,
    mask: [Option<bool>; 36],
}

impl Default for EmulatedMemory {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            mask: [None; 36],
        }
    }
}

#[derive(Debug)]
enum EmulatedMemoryParseError {
    NotOperation(anyhow::Error),
    BadFormatting(anyhow::Error),
    Internal(anyhow::Error),
}

impl EmulatedMemory {
    fn apply_mask(&mut self, mask_str: &str) -> Result<(), anyhow::Error> {
        if mask_str.len() != 36 {
            bail!(
                "Length is {}, but mask is exactly 36 characters wide",
                mask_str.len()
            );
        };

        mask_str.bytes().enumerate().try_for_each(|(idx, byte)| {
            self.mask[idx] = match byte {
                b'X' => None,
                b'0' => Some(false),
                b'1' => Some(true),
                x => bail!("{x} is not a valid character (X, 0, or 1)"),
            };
            Ok(())
        })?;
        Ok(())
    }

    fn map_memory(&mut self, index: usize, raw: EmulatedInt) {
        self.map.insert(
            index,
            EmulatedInt {
                bits: array::from_fn(|idx| {
                    if let Some(x) = self.mask[idx] {
                        x
                    } else {
                        raw.bits[idx]
                    }
                }),
            },
        );
    }

    fn parse_mask(s: &str) -> Result<&str, EmulatedMemoryParseError> {
        lazy_static! {
            static ref MASK: Regex = Regex::new(r#"mask = ((?:X|1|0){36})"#).unwrap();
        }
        let cap = MASK
            .captures(s)
            .ok_or(EmulatedMemoryParseError::NotOperation(anyhow!(
                "{s} is not a MASK operation"
            )))?;
        Ok(cap
            .get(1)
            .ok_or(EmulatedMemoryParseError::BadFormatting(anyhow!(
                "Mask formatting incorrect:\n\t{s}"
            )))?
            .as_str())
    }

    fn parse_mem(s: &str) -> Result<(usize, EmulatedInt), EmulatedMemoryParseError> {
        lazy_static! {
            static ref MEM: Regex = Regex::new(r#"mem\[(\d+)\] = (\d+)"#).unwrap();
        }
        let cap = MEM
            .captures(s)
            .ok_or(EmulatedMemoryParseError::NotOperation(anyhow!(
                "{s} is not a MEM operation"
            )))?;
        let index = cap
            .get(1)
            .ok_or(EmulatedMemoryParseError::BadFormatting(anyhow!(
                "Memory operation formatting incorrect:\n\t{s}"
            )))?
            .as_str()
            .parse::<usize>()
            .map_err(|err| EmulatedMemoryParseError::Internal(err.into()))?;
        let int = cap
            .get(2)
            .ok_or(EmulatedMemoryParseError::BadFormatting(anyhow!(
                "Memory operation formatting incorrect:\n\t{s}"
            )))?
            .as_str()
            .parse::<EmulatedInt>()
            .map_err(EmulatedMemoryParseError::Internal)?;
        Ok((index, int))
    }

    fn apply_operation(&mut self, s: &str) -> Result<(), anyhow::Error> {
        match Self::parse_mask(s) {
            Ok(mask) => {
                self.apply_mask(mask)?;
                Ok(())
            }
            Err(EmulatedMemoryParseError::NotOperation(_)) => match Self::parse_mem(s) {
                Ok((index, emulated_int)) => {
                    self.map_memory(index, emulated_int);
                    Ok(())
                }
                Err(EmulatedMemoryParseError::NotOperation(_)) => {
                    Err(anyhow!("{s} is neither a MASK nor MEM operation"))
                }
                Err(EmulatedMemoryParseError::BadFormatting(x))
                | Err(EmulatedMemoryParseError::Internal(x)) => Err(x),
            },
            Err(EmulatedMemoryParseError::BadFormatting(x))
            | Err(EmulatedMemoryParseError::Internal(x)) => Err(x),
        }
    }

    fn batch_operations<I, T>(&mut self, ops: I) -> Result<(), anyhow::Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        ops.into_iter()
            .map(|op| self.apply_operation(op.as_ref()))
            .fold_ok((), |_, _| ())
    }

    fn from_file(name: &str) -> Result<Self, anyhow::Error> {
        let mut mem = Self::default();
        mem.batch_operations(
            BufReader::new(File::open(name)?)
                .lines()
                .map(|line| line.unwrap()),
        )?;
        Ok(mem)
    }

    fn sum(&self) -> u64 {
        self.map.values().map(u64::from).sum()
    }
}

// -------------------- END STRUCT --------------------

#[derive(Debug, Default)]
struct EmulatedMemoryV2 {
    mem: EmulatedMemory,
}

impl EmulatedMemoryV2 {
    fn map_memory(&mut self, mut index: EmulatedInt, value: EmulatedInt) {
        self.mem.mask.iter().enumerate().for_each(|(idx, &val)| {
            if val == Some(true) {
                index.bits[idx] = true
            }
        });

        let mut index_vec = vec![index];
        self.mem.mask.iter().enumerate().for_each(|(idx, &val)| {
            if val.is_none() {
                index_vec.extend(
                    index_vec
                        .iter()
                        .map(|&index| {
                            let mut flip_index = index;
                            flip_index.bits[idx] = !flip_index.bits[idx];
                            flip_index
                        })
                        .collect_vec(),
                );
            }
        });

        index_vec.iter().for_each(|index| {
            self.mem.map.insert(u64::from(index) as usize, value);
        });
    }

    fn apply_operation(&mut self, s: &str) -> Result<(), anyhow::Error> {
        match EmulatedMemory::parse_mask(s) {
            Ok(mask) => {
                self.mem.apply_mask(mask)?;
                Ok(())
            }
            Err(EmulatedMemoryParseError::NotOperation(_)) => match EmulatedMemory::parse_mem(s) {
                Ok((index, emulated_int)) => {
                    self.map_memory((index as u64).try_into()?, emulated_int);
                    Ok(())
                }
                Err(EmulatedMemoryParseError::NotOperation(_)) => {
                    Err(anyhow!("{s} is neither a MASK nor MEM operation"))
                }
                Err(EmulatedMemoryParseError::BadFormatting(x))
                | Err(EmulatedMemoryParseError::Internal(x)) => Err(x),
            },
            Err(EmulatedMemoryParseError::BadFormatting(x))
            | Err(EmulatedMemoryParseError::Internal(x)) => Err(x),
        }
    }

    fn batch_operations<I, T>(&mut self, ops: I) -> Result<(), anyhow::Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        ops.into_iter()
            .map(|op| self.apply_operation(op.as_ref()))
            .fold_ok((), |_, _| ())
    }

    fn from_file(name: &str) -> Result<Self, anyhow::Error> {
        let mut mem = Self::default();
        mem.batch_operations(
            BufReader::new(File::open(name)?)
                .lines()
                .map(|line| line.unwrap()),
        )?;
        Ok(mem)
    }

    fn sum(&self) -> u64 {
        self.mem.map.values().map(u64::from).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_mask() {
        let mut mem = EmulatedMemory::default();
        mem.apply_operation("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
            .unwrap();
        println!("{:?}", mem.mask);
        assert_eq!(mem.mask[mem.mask.len() - 2], Some(false));
        assert_eq!(mem.mask[mem.mask.len() - 7], Some(true));
        assert!(mem
            .mask
            .iter()
            .enumerate()
            .filter_map(
                |(idx, val)| if idx == mem.mask.len() - 2 || idx == mem.mask.len() - 7 {
                    None
                } else {
                    Some(val)
                }
            )
            .all(|&val| val.is_none()))
    }

    #[test]
    fn set_memory() {
        let mut mem = EmulatedMemory::default();
        mem.apply_operation("mem[8] = 11").unwrap();
        println!("{:?}", mem.map);
        assert_eq!(mem.map.keys().collect_vec(), [&8]);
        assert_eq!(mem.map[&8].bits[32..36], [true, false, true, true]);
        assert!(mem.map[&8].bits.iter().rev().skip(4).all(|&val| !val));
    }

    #[test]
    fn convert() {
        assert_eq!(u64::from(&EmulatedInt { bits: [true; 36] }), 68719476735);
    }

    #[test]
    fn part_1() {
        assert_eq!(EmulatedMemory::from_file("test-input").unwrap().sum(), 165);
    }

    #[test]
    fn from_int() {
        let emulated_int = EmulatedInt::try_from(3).unwrap();
        assert_eq!(emulated_int.bits[34..36], [true, true]);
        assert!(emulated_int.bits.iter().rev().skip(2).all(|&val| !val));
        assert_eq!(u64::from(&emulated_int), 3);
    }

    #[test]
    fn multi_address_write() {
        let mut v2 = EmulatedMemoryV2::default();
        v2.apply_operation("mask = 000000000000000000000000000000X1001X")
            .unwrap();
        v2.apply_operation("mem[42] = 100").unwrap();
        assert_eq!(
            v2.mem.map.keys().copied().sorted().collect_vec(),
            [26, 27, 58, 59]
        );
        assert_eq!(v2.sum(), 100 * 4);
    }

    #[test]
    fn part_2() {
        println!("PART 2 Test");
        assert_eq!(
            EmulatedMemoryV2::from_file("test-input-2").unwrap().sum(),
            208
        );
    }
}
