use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut stim: Vec<_> = file_to_vec("input");
    let (one, two) = sum_to(&mut stim, 2020).unwrap();
    println!("{}", one * two);

    let sol = find_sum(&mut stim, 2020, 3).unwrap();
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
            return Err("List has no valid pairs");
        }
        _ => Err("List is < 2 entries"),
    }
}

fn find_sum<T: num::Integer + Copy>(list: &[T], target: T, number: usize) -> Result<Vec<T>, &str> {
    match list.len() {
        len if len >= number => match number {
            num if num <= 0 => Err("Cannot find the sum of zero or fewer values"),
            1 => match list.binary_search(&target) {
                Ok(_) => Ok(vec![target]),
                Err(_) => Err("Not found in list"),
            },
            _ => {
                for val in list {
                    match find_sum(list, target - *val, number - 1) {
                        Ok(mut matches) => {
                            matches.push(*val);
                            return Ok(matches);
                        }
                        Err(_) => (),
                    }
                }
                return Err("No matching set in list");
            }
        },
        _ => Err("List is smaller than given number"),
    }
}

// https://stackoverflow.com/questions/65100493/how-to-read-a-list-of-numbers-from-a-file-into-a-vec/65100529#65100529
fn file_to_vec(name: &str) -> Vec<i32> {
    return BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
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
        let mut stim: Vec<_> = file_to_vec("test-input");
        assert_eq!(
            find_sum(&mut stim, 2020, 3).unwrap().sort(),
            [979, 366, 675].sort()
        );
    }

    #[test]
    fn part2_sum() {
        let mut stim: Vec<_> = file_to_vec("test-input");
        let sol = find_sum(&mut stim, 2020, 3).unwrap();
        assert_eq!(sol[0] * sol[1] * sol[2], 241861950);
    }
}
