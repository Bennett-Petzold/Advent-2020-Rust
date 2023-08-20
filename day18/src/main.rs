use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use evalexpr::eval_int;
use evalexpr_second::eval_int as eval_int_second;

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input").unwrap())
            .lines()
            .map(|line| eval_int(&line.unwrap()).unwrap())
            .sum::<i64>()
    );
    println!(
        "{}",
        BufReader::new(File::open("input").unwrap())
            .lines()
            .map(|line| eval_int_second(&line.unwrap()).unwrap())
            .sum::<i64>()
    );
}
