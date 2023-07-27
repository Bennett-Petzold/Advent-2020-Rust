use anyhow::anyhow;
use num::{
    integer::{div_floor, ExtendedGcd},
    Integer,
};
use std::fs::read_to_string;

fn main() {
    let (arrival, buses) = part1_from_file("input").unwrap();
    println!("{}", Bus::part1_solution(arrival, buses));
    let buses = part2_from_file("input").unwrap();
    println!("{}", Bus::part2_solution(buses).unwrap());
}

#[derive(Debug, Clone)]
struct Bus {
    interval: u32,
}

impl TryFrom<&str> for Bus {
    type Error = <u32 as std::str::FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            interval: value.parse()?,
        })
    }
}

impl Bus {
    fn from_str(value: &str) -> Vec<Bus> {
        value
            .split(',')
            .filter_map(|value| Bus::try_from(value).ok())
            .collect()
    }

    fn from_str_indexed(value: &str) -> Vec<(u32, Bus)> {
        let mut i = -1;
        value
            .split(',')
            .map(|value| {
                i += 1;
                (i.try_into().unwrap(), Bus::try_from(value).ok())
            })
            .filter_map(|(i, bus)| bus.map(|bus| (i, bus)))
            .collect()
    }

    fn first_overlap(&self, time: u32) -> u32 {
        let estimate = (time / self.interval) * self.interval;
        if (time % self.interval) > 0 {
            estimate + self.interval
        } else {
            estimate
        }
    }

    fn part1_solution<I>(arrival: u32, buses: I) -> u32
    where
        I: IntoIterator<Item = Bus>,
    {
        let mut min = u32::MAX;
        let mut bus_id = u32::default();
        for bus in buses {
            let wait_time = bus.first_overlap(arrival) - arrival;
            if wait_time < min {
                min = wait_time;
                bus_id = bus.interval;
            }
        }
        min * bus_id
    }

    fn part2_solution<I>(buses: I) -> Result<i128, anyhow::Error>
    where
        I: IntoIterator<Item = (u32, Bus)>,
    {
        let mut buses = buses
            .into_iter()
            .map(|(offset, bus): (u32, Bus)| match offset {
                0 => (0, bus.interval as i128),
                offset => (
                    (bus.interval - (offset % bus.interval)) as i128,
                    bus.interval as i128,
                ),
            });
        let first = buses.by_ref().next().ok_or(anyhow!("No first entry"))?;

        Ok(buses
            .try_fold(first, |acc, next| {
                let ExtendedGcd { gcd, x, y } = Integer::extended_gcd(&acc.1, &next.1);
                if gcd != 1 {
                    return Err(anyhow!("gcd of {} and {} == {gcd}, not 1", acc.1, next.1));
                }

                let step = acc.1 * next.0 * x + next.1 * acc.0 * y;
                let interval = acc.1 * next.1;
                // Smallest possible offset
                let offset = step + (interval * -(div_floor(step, interval)));
                Ok((offset, interval))
            })?
            .0)
    }
}

fn part1_from_file(name: &str) -> Result<(u32, Vec<Bus>), anyhow::Error> {
    let file = read_to_string(name).unwrap();
    let mut lines = file.lines();
    Ok((
        lines.next().ok_or(anyhow!("Missing first line"))?.parse()?,
        Bus::from_str(lines.next().ok_or(anyhow!("Missing second line"))?),
    ))
}

fn part2_from_file(name: &str) -> Result<Vec<(u32, Bus)>, anyhow::Error> {
    let file = read_to_string(name).unwrap();
    let mut lines = file.lines();
    lines.next().ok_or(anyhow!("Missing first line"))?;
    Ok(Bus::from_str_indexed(
        lines.next().ok_or(anyhow!("Missing second line"))?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let (arrival, buses) = part1_from_file("test-input").unwrap();
        assert_eq!(Bus::part1_solution(arrival, buses), 295);
    }

    #[test]
    fn part2_trivial() {
        let buses = part2_from_file("trivial-test-input").unwrap();
        assert_eq!(Bus::part2_solution(buses).unwrap(), 8);
    }

    #[test]
    fn part2() {
        let buses = part2_from_file("test-input").unwrap();
        assert_eq!(Bus::part2_solution(buses).unwrap(), 1068781);
    }
}
