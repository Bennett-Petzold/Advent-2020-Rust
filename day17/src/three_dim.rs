use crate::Point;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point for Point3D {
    fn from_2_d(x: isize, y: isize) -> Self {
        Self { x, y, z: 0 }
    }

    /// Returns all points that differ by at most 1 on any given axis
    ///
    /// # Examples
    /// ```
    /// use day17_lib::Point;
    /// use day17_lib::three_dim::Point3D;
    ///
    /// let mut neighbors = Point3D { x: 0, y: 0, z: 0 }.neighbors();
    /// let mut expected = vec![
    ///     Point3D {
    ///         x: -1,
    ///         y: -1,
    ///         z: -1,
    ///     },
    ///     Point3D { x: -1, y: 0, z: -1 },
    ///     Point3D { x: -1, y: 1, z: -1 },
    ///     Point3D { x: 0, y: -1, z: -1 },
    ///     Point3D { x: 0, y: 0, z: -1 },
    ///     Point3D { x: 0, y: 1, z: -1 },
    ///     Point3D { x: 1, y: -1, z: -1 },
    ///     Point3D { x: 1, y: 0, z: -1 },
    ///     Point3D { x: 1, y: 1, z: -1 },
    ///     Point3D { x: -1, y: -1, z: 0 },
    ///     Point3D { x: -1, y: 0, z: 0 },
    ///     Point3D { x: -1, y: 1, z: 0 },
    ///     Point3D { x: 0, y: -1, z: 0 },
    ///     Point3D { x: 0, y: 1, z: 0 },
    ///     Point3D { x: 1, y: -1, z: 0 },
    ///     Point3D { x: 1, y: 0, z: 0 },
    ///     Point3D { x: 1, y: 1, z: 0 },
    ///     Point3D { x: -1, y: -1, z: 1 },
    ///     Point3D { x: -1, y: 0, z: 1 },
    ///     Point3D { x: -1, y: 1, z: 1 },
    ///     Point3D { x: 0, y: -1, z: 1 },
    ///     Point3D { x: 0, y: 0, z: 1 },
    ///     Point3D { x: 0, y: 1, z: 1 },
    ///     Point3D { x: 1, y: -1, z: 1 },
    ///     Point3D { x: 1, y: 0, z: 1 },
    ///     Point3D { x: 1, y: 1, z: 1 },
    /// ];
    ///
    /// neighbors.sort();
    /// expected.sort();
    /// assert_eq!(neighbors, expected);
    /// ```
    fn neighbors(&self) -> Vec<Self> {
        let x_range = (self.x - 1)..=(self.x + 1);
        let y_range = (self.y - 1)..=(self.y + 1);
        let z_range = (self.z - 1)..=(self.z + 1);

        x_range
            .flat_map(|x| {
                y_range
                    .clone()
                    .flat_map(|y| {
                        z_range
                            .clone()
                            .map(|z| Self { x, y, z })
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
        let dim: Dimension<Point3D> = input.iter().collect();

        assert_eq!(
            dim.active,
            vec![
                Point3D { x: 1, y: 0, z: 0 },
                Point3D { x: 2, y: 1, z: 0 },
                Point3D { x: 0, y: 2, z: 0 },
                Point3D { x: 1, y: 2, z: 0 },
                Point3D { x: 2, y: 2, z: 0 },
            ]
            .into()
        )
    }

    #[test]
    fn part1() {
        let mut dim: Dimension<Point3D> = Dimension::from_file("test-input");
        dim.cycle_count(6);
        assert_eq!(dim.num_active(), 112);
    }
}
