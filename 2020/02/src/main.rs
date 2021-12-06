#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

fn main() {
    let input = get_input(INPUT_FILE);

    let count = parse(&input);
    println!("Valid passwords: {}", count);

    let count = parse2(&input);
    println!("Valid passwords round 2: {}", count);
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

enum Index {
    Min = 0,
    Max,
    Char,
    Password,
}

fn parse(input: &str) -> usize {
    // line becomes array
    // [0=min, 1=max, 2=char, 3=password]
    input
        .lines()
        .filter(|line| {
            let vals = line
                .trim()
                .split(": ")
                .flat_map(|v| v.split(&['-', ' '][..]).collect::<Vec<&str>>())
                .collect::<Vec<&str>>();

            let min = vals[Index::Min as usize].parse::<usize>().unwrap();
            let max = vals[Index::Max as usize].parse::<usize>().unwrap();

            let count = vals[Index::Password as usize]
                .matches(vals[Index::Char as usize])
                .count();

            if count < min || count > max {
                false
            } else {
                true
            }
        })
        .count()
}

fn parse2(input: &str) -> usize {
    // line becomes array
    // [0=min, 1=max, 2=char, 3=password]
    input
        .lines()
        .filter(|line| {
            let vals = line
                .trim()
                .split(": ")
                .flat_map(|v| v.split(&['-', ' '][..]).collect::<Vec<&str>>())
                .collect::<Vec<&str>>();

            let min = vals[Index::Min as usize].parse::<usize>().unwrap();
            let max = vals[Index::Max as usize].parse::<usize>().unwrap();
            let c = vals[Index::Char as usize].chars().nth(0).unwrap();
            let mut p = vals[Index::Password as usize].chars();
            let min_c = p.nth(min - 1).expect(line);
            let max_c = p.nth(max - min - 1).expect(line);

            if (min_c == c && max_c != c) || (min_c != c && max_c == c) {
                true
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_parse() {
        let count = parse(TEST_INPUT);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_parse2() {
        let count = parse2(TEST_INPUT);
        assert_eq!(count, 1);
    }
}
