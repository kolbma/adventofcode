#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

struct Position {
    pub horizontal: u16,
    pub depth: u16,
    pub aim: u32,
    pub aim_depth: u32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
            aim_depth: 0,
        }
    }

    fn forward(self: &mut Self, n: u16) {
        self.horizontal += n;
        self.aim_depth += self.aim.checked_mul(n as u32).unwrap();
    }

    fn down(self: &mut Self, n: u16) {
        self.depth += n;
        self.aim += n as u32;
    }

    fn up(self: &mut Self, n: u16) {
        self.depth -= n;
        self.aim -= n as u32;
    }

    fn result(self: &Self) -> u32 {
        (self.horizontal as u32)
            .checked_mul(self.depth as u32)
            .unwrap()
    }

    fn result_aimed(self: &Self) -> u32 {
        (self.horizontal as u32)
            .checked_mul(self.aim_depth as u32)
            .unwrap()
    }
}

fn control(pos: &mut Position, input: &str) {
    input.lines().for_each(|line| {
        let mut iter = line.trim().split_whitespace();
        let cmd = iter.next().unwrap();
        let n = iter.next().unwrap().parse::<u16>().unwrap();
        match cmd {
            "forward" => pos.forward(n),
            "down" => pos.down(n),
            "up" => pos.up(n),
            _ => {}
        }
    });
}

fn main() {
    let input = get_input(INPUT_FILE);

    let mut pos = Position::new();

    control(&mut pos, &input);
    let res = pos.result();

    println!("Multiplied position result: {}", res);

    let res = pos.result_aimed();

    println!("Multiplied position result aimed: {}", res);
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_control_result() {
        let mut pos = Position::new();

        control(&mut pos, TEST_INPUT);
        let res = pos.result();

        assert_eq!(res, 150);
    }

    #[test]
    fn test_control_result_aimed() {
        let mut pos = Position::new();

        control(&mut pos, TEST_INPUT);
        let res = pos.result_aimed();

        assert_eq!(res, 900);
    }
}
