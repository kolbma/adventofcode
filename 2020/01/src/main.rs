#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

type StarsType = u32;
const YEAR: StarsType = 2020;

fn main() {
    let input = get_input(INPUT_FILE);
    let (no1, no2) = find_year(&input);

    println!(
        "Found the two entries that sum to 2020; Multiplied to {}",
        no1 * no2
    );

    let summands = find_year_vec(&input, 3);

    println!(
        "Found the three entries that sum to 2020; Multiplied to {}",
        summands.iter().product::<StarsType>()
    );
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

#[inline(always)]
fn find_year(input: &str) -> (StarsType, StarsType) {
    let expenses: Vec<StarsType> = input
        .lines()
        .map(|line| line.trim().parse::<StarsType>().unwrap())
        .collect();
    for expense in &expenses {
        for find in &expenses {
            if find + expense == YEAR {
                return (*find, *expense);
            }
        }
    }
    panic!("not found 2020");
}

#[inline(always)]
fn find_year_vec(input: &str, number: usize) -> Vec<StarsType> {
    let expenses: Vec<StarsType> = input
        .lines()
        .map(|line| line.trim().parse::<StarsType>().unwrap())
        .collect();

    fn f(
        summands: &mut Vec<StarsType>,
        vals: &Vec<StarsType>,
        number: usize,
        depth: usize,
    ) -> bool {
        if summands.len() == depth {
            summands.push(0);
        }

        for val in vals {
            {
                summands[depth] = *val;
            }
            if depth + 1 == number {
                if summands.iter().sum::<StarsType>() == YEAR {
                    return true;
                } else {
                    continue;
                }
            } else if f(summands, vals, number, depth + 1) {
                return true;
            }
        }
        return false;
    }

    let mut summands = Vec::new();
    f(&mut summands, &expenses, number, 0);

    summands
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"1721
    979
    366
    299
    675
    1456";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_find_2020() {
        let (no1, no2) = find_year(TEST_INPUT);
        assert_eq!(no1 + no2, YEAR);
    }

    #[test]
    fn test_multiply_2020_vals() {
        let (no1, no2) = find_year(TEST_INPUT);
        assert_eq!(no1 * no2, 514579);
    }

    #[test]
    fn test_multiply_2020_vec() {
        let summands = find_year_vec(TEST_INPUT, 2);
        assert_eq!(summands.iter().product::<StarsType>(), 514579);
        let summands = find_year_vec(TEST_INPUT, 3);
        assert_eq!(summands.iter().product::<StarsType>(), 241861950);
    }
}
