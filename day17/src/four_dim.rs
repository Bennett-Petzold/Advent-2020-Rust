use crate::Point;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct Point4D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub w: isize,
}

impl Point for Point4D {
    fn from_2_d(x: isize, y: isize) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    /// Returns all points that differ by at most 1 on any given axis
    fn neighbors(&self) -> Vec<Self> {
        let x_range = (self.x - 1)..=(self.x + 1);
        let y_range = (self.y - 1)..=(self.y + 1);
        let z_range = (self.z - 1)..=(self.z + 1);
        let w_range = (self.w - 1)..=(self.w + 1);

        x_range
            .flat_map(|x| {
                y_range
                    .clone()
                    .flat_map(|y| {
                        z_range
                            .clone()
                            .flat_map(|z| {
                                w_range
                                    .clone()
                                    .map(|w| Self { x, y, z, w })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|point| point != self)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Dimension;

    use super::*;

    #[test]
    fn parse() {
        let input = [".#.", "..#", "###"];
        let dim: Dimension<Point4D> = input.iter().collect();

        assert_eq!(
            dim.active,
            vec![
                Point4D {
                    x: 1,
                    y: 0,
                    z: 0,
                    w: 0
                },
                Point4D {
                    x: 2,
                    y: 1,
                    z: 0,
                    w: 0
                },
                Point4D {
                    x: 0,
                    y: 2,
                    z: 0,
                    w: 0
                },
                Point4D {
                    x: 1,
                    y: 2,
                    z: 0,
                    w: 0
                },
                Point4D {
                    x: 2,
                    y: 2,
                    z: 0,
                    w: 0
                },
            ]
            .into()
        )
    }

    #[test]
    fn part2() {
        let mut dim: Dimension<Point4D> = Dimension::from_file("test-input");
        dim.cycle_count(6);
        assert_eq!(dim.num_active(), 848);
    }
}
