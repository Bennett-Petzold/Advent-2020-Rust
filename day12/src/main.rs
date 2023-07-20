use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

fn main() {
    let mut ship = Ship::default();
    ship.apply_iter(Action::from_file("input").unwrap());
    println!("{}", ship.manhattan_distance());

    let mut waypoint = Waypoint::default();
    waypoint.apply_iter(Action::from_file("input").unwrap());
    println!("{}", waypoint.manhattan_distance());
}

#[derive(Debug)]
enum Action {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Left(u8),  // 90 degree (1/2 pi radians) increments
    Right(u8), // 90 degree (1/2 pi radians) increments
    Forward(u16),
}

#[derive(Debug, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Ship {
    pub x: i32,
    pub y: i32,
    orient: Orientation,
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
    ship: Ship,
}

impl TryFrom<&str> for Action {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let amount: u16 = value[1..].parse()?;
        match value.chars().next() {
            Some('N') => Ok(Self::North(amount)),
            Some('S') => Ok(Self::South(amount)),
            Some('E') => Ok(Self::East(amount)),
            Some('W') => Ok(Self::West(amount)),
            Some('F') => Ok(Self::Forward(amount)),
            // One step left or right is 90 degrees, and this wraps
            Some('L') => Ok(Self::Left(((amount / 90) % 4).try_into()?)),
            Some('R') => Ok(Self::Right(((amount / 90) % 4).try_into()?)),
            Some(x) => Err(anyhow!("First letter ({x}) is not a valid action")),
            None => Err(anyhow!("No first character.")),
        }
    }
}

impl Action {
    fn from_file(name: &str) -> Result<Vec<Self>, anyhow::Error> {
        BufReader::new(File::open(name).unwrap())
            .lines()
            .map(|line| Self::try_from(line.unwrap().as_str()))
            .collect()
    }
}

impl Orientation {
    fn turn_left(&mut self, count: u8) {
        let mut result = self.clone();
        for _ in 0..count {
            result = match result {
                Self::East => Self::North,
                Self::North => Self::West,
                Self::West => Self::South,
                Self::South => Self::East,
            }
        }
        *self = result;
    }

    fn turn_right(&mut self, count: u8) {
        let mut result = self.clone();
        for _ in 0..count {
            result = match result {
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
                Self::North => Self::East,
            }
        }
        *self = result;
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            orient: Orientation::East,
        }
    }
}

impl Ship {
    fn forward_direction(&self, forward: u16) -> Action {
        match self.orient {
            Orientation::East => Action::East(forward),
            Orientation::North => Action::North(forward),
            Orientation::West => Action::West(forward),
            Orientation::South => Action::South(forward),
        }
    }

    fn apply(&mut self, act: Action) {
        match act {
            Action::Forward(x) => self.apply(self.forward_direction(x)),
            Action::Left(x) => self.orient.turn_left(x),
            Action::Right(x) => self.orient.turn_right(x),
            Action::North(x) => self.y += x as i32,
            Action::South(x) => self.y -= x as i32,
            Action::East(x) => self.x += x as i32,
            Action::West(x) => self.x -= x as i32,
        }
    }

    fn apply_iter<I>(&mut self, list: I)
    where
        I: IntoIterator<Item = Action>,
    {
        for act in list.into_iter() {
            self.apply(act);
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            x: 10,
            y: 1,
            ship: Ship::default(),
        }
    }
}

impl Waypoint {
    fn ship_forward(&mut self, count: i32) {
        self.ship.x += self.x * count;
        self.ship.y += self.y * count;
    }

    fn rotate_clockwise(&mut self, count: i32) {
        for _ in 0..count {
            let new_x = self.y;
            self.y = -self.x;
            self.x = new_x;
        }
    }

    fn rotate_counterclockwise(&mut self, count: i32) {
        self.rotate_clockwise(4 - (count % 4));
    }

    fn apply(&mut self, act: Action) {
        match act {
            Action::North(x) => self.y += x as i32,
            Action::South(x) => self.y -= x as i32,
            Action::East(x) => self.x += x as i32,
            Action::West(x) => self.x -= x as i32,
            Action::Forward(x) => self.ship_forward(x as i32),
            Action::Right(x) => self.rotate_clockwise((x % 4) as i32),
            Action::Left(x) => self.rotate_counterclockwise(x as i32),
        }
    }

    fn apply_iter<I>(&mut self, list: I)
    where
        I: IntoIterator<Item = Action>,
    {
        for act in list.into_iter() {
            self.apply(act);
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.ship.manhattan_distance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut ship = Ship::default();
        ship.apply_iter(Action::from_file("test-input").unwrap());
        assert_eq!(ship.manhattan_distance(), 25);
    }

    #[test]
    fn part2() {
        let mut waypoint = Waypoint::default();
        waypoint.apply_iter(Action::from_file("test-input").unwrap());
        assert_eq!(waypoint.manhattan_distance(), 286);
    }
}
