use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::empty;
use std::num;
use std::ops::RangeBounds;
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_lines("./src/input.txt").unwrap();

    let mut total = 0;

    for line in lines.flatten() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        let increasing = nums[0] < nums[1];
        let mut is_safe = true;
        for i in 0..nums.len() - 1 {
            if increasing {
                if nums[i] > nums[i + 1] {
                    is_safe = false;
                    break;
                }
            }

            if !increasing {
                if nums[i] < nums[i + 1] {
                    is_safe = false;
                    break;
                }
            }

            let diff = nums[i].abs_diff(nums[i + 1]);
            if diff > 3 || diff < 1 {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            total += 1;
        }
    }

    println!("Total: {}", total);
}

fn check_list(numbers: &[i64]) -> Result<(), usize> {
    if numbers.len() < 2 {
        return Err(0); // Return index 0 if the list is too short
    }

    let mut increasing = true;
    if numbers[0] > numbers[1] {
        increasing = false;
    }
    if !(1..=3).contains(&(numbers[0] - numbers[1]).abs()) {
        return Err(1); // Return index 1 for the first invalid difference
    }

    for i in 2..numbers.len() {
        let diff = (numbers[i] - numbers[i - 1]).abs();
        if !(1..=3).contains(&diff) {
            return Err(i); // Return the current index for an invalid difference
        }
        if increasing && numbers[i] < numbers[i - 1] {
            return Err(i); // Return the current index if the sequence is not increasing
        }
        if !increasing && numbers[i] > numbers[i - 1] {
            return Err(i); // Return the current index if the sequence is not decreasing
        }
    }

    Ok(())
}

fn part2() {
    let lines = read_lines("./src/input.txt").unwrap();

    let mut total = 0;

    for line in lines.flatten() {
        let mut nums: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        match check_list(&nums) {
            Ok(_) => {
                total += 1;
            }
            Err(i) => {
                for n in 0..nums.len() {
                    let reduced: Vec<i64> = nums
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != n)
                        .map(|(_, value)| *value)
                        .collect();

                    if check_list(&reduced).is_ok() {
                        total += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("Total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
