#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

fn main() {
    let input = get_input(INPUT_FILE);
    let count = count_increased(&input);

    println!("Measurements larger than previous: {}", count);

    let count = count_increased_window(&input);

    println!("Measurements larger than previous window: {}", count);
}

#[inline(always)]
fn get_input(path: &str) -> String {
    // read text file
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn count_increased(input: &str) -> usize {
    let mut last_val_opt = None;
    input
        .lines()
        .filter(|line| {
            if let Some(last_val) = last_val_opt {
                let val = line.trim().parse::<u16>().unwrap();
                let res = val > last_val;
                last_val_opt.replace(val);
                res
            } else {
                last_val_opt.replace(line.trim().parse::<u16>().unwrap());
                false
            }
        })
        .count()
}

fn count_increased_window(input: &str) -> usize {
    let mut last_vals = Vec::new();
    input
        .lines()
        .filter(|line| {
            if last_vals.len() == 3 {
                let val = line.trim().parse::<u16>().unwrap();
                let sum_old = last_vals.iter().sum::<u16>();
                let sum = sum_old - last_vals.remove(0) + val;
                last_vals.push(val);

                if sum_old < sum {
                    true
                } else {
                    false
                }
            } else {
                last_vals.push(line.trim().parse::<u16>().unwrap());
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_count_increased() {
        let count = count_increased(TEST_INPUT);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_increased_window() {
        let count = count_increased_window(TEST_INPUT);
        assert_eq!(count, 5);
    }
}
