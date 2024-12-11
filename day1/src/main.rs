use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::empty;
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part2() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut similarity = 0;
        let mut left: Vec<i64> = vec![];
        let mut freq: HashMap<i64, i64> = HashMap::new();

        for line in lines.flatten() {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            left.push(nums[0]);

            if let Some(n) = freq.get_mut(&nums[1]) {
                *n += 1;
            } else {
                freq.insert(nums[1], 1);
            }
        }

        for num in left {
            similarity += freq.get(&num).unwrap_or(&0) * num;
        }

        println!("Total: {}", similarity);
    }
}
fn part1() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total = 0;
        let mut left: Vec<i64> = vec![];
        let mut right: Vec<i64> = vec![];

        for line in lines.flatten() {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            left.push(nums[0]);
            right.push(nums[1]);
        }
        left.sort();
        right.sort();

        for i in (0..left.len()) {
            total += left[i].abs_diff(right[i]).try_into().unwrap_or(0);
        }

        println!("Total: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
