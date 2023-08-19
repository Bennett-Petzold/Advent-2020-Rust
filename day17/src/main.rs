use day17_lib::{four_dim::Point4D, three_dim::Point3D, Dimension};

fn main() {
    let mut dim: Dimension<Point3D> = Dimension::from_file("input");
    dim.cycle_count(6);
    println!("{}", dim.num_active());

    let mut four_dim: Dimension<Point4D> = Dimension::from_file("input");
    four_dim.cycle_count(6);
    println!("{}", four_dim.num_active());
}
