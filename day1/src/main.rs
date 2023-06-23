use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut stim: Vec<_> = file_to_vec("input");
    let (one, two) = sum_to(&mut stim, 2020).unwrap();
    println!("{}", one * two);

    let sol = find_sum(&stim, 2020, 3).unwrap();
    println!("{}", sol[0] * sol[1] * sol[2]);
}

fn sum_to<T: num::Integer + Copy>(list: &mut [T], target: T) -> Result<(T, T), &str> {
    match list.len() {
        len if len >= 2 => {
            list.sort();
            let mut front = 0;
            let mut back = len - 1;
            while front != back {
                match list[front] + list[back] {
                    sum if sum == target => return Ok((list[front], list[back])),
                    sum if sum > target => back -= 1,
                    sum if sum < target => front += 1,
                    _ => (),
                }
            }
            Err("List has no valid pairs")
        }
        _ => Err("List is < 2 entries"),
    }
}

fn find_sum<T: num::Integer + Copy>(list: &[T], target: T, number: usize) -> Result<Vec<T>, &str> {
    match list.len() {
        len if len >= number => match number {
            1 => match list.binary_search(&target) {
                Ok(_) => Ok(vec![target]),
                Err(_) => Err("Not found in list"),
            },
            _ => {
                for val in list {
                    if let Ok(mut matches) = find_sum(list, target - *val, number - 1) {
                        matches.push(*val);
                        return Ok(matches);
                    }
                }
                Err("No matching set in list")
            }
        },
        _ => Err("List is smaller than given number"),
    }
}

// https://stackoverflow.com/questions/65100493/how-to-read-a-list-of-numbers-from-a-file-into-a-vec/65100529#65100529
fn file_to_vec(name: &str) -> Vec<i32> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_match() {
        let mut stim: Vec<_> = file_to_vec("test-input");
        assert_eq!(sum_to(&mut stim, 2020).unwrap(), (299, 1721));
    }

    #[test]
    fn part1_sum() {
        let mut stim: Vec<_> = file_to_vec("test-input");
        let (one, two) = sum_to(&mut stim, 2020).unwrap();
        assert_eq!(one * two, 514579);
    }

    #[test]
    fn part2_match() {
        let stim: Vec<_> = file_to_vec("test-input");
        let mut found_sum = find_sum(&stim, 2020, 3).unwrap();
        found_sum.sort();
        let mut expected = [979, 366, 675];
        expected.sort();
        assert_eq!(found_sum, expected);
    }

    #[test]
    fn part2_sum() {
        let stim: Vec<_> = file_to_vec("test-input");
        let sol = find_sum(&stim, 2020, 3).unwrap();
        assert_eq!(sol[0] * sol[1] * sol[2], 241861950);
    }
}
