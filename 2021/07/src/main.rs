#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

fn main() {
    let input = get_input(INPUT_FILE);

    let pos = get_lowcost_position(&input);
    let cost = calc_cost(&pos.0, pos.1);
    println!("Position {} with lowest cost {}", pos.1, cost);

    let pos = get_lowcost_position2(&input);
    let cost = pos.2;

    println!("Position {} with lowest cost in round 2 {}", pos.1, cost);
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

fn get_lowcost_position(input: &str) -> (Vec<u16>, u16) {
    let positions = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|pos| pos.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let crab_count = positions.len() as u16;

    let mut mean_vec = positions.clone();
    mean_vec.sort_unstable();
    if crab_count % 2 == 0 {
        (
            positions,
            (mean_vec[crab_count as usize / 2 - 1] + mean_vec[crab_count as usize / 2]) / 2,
        )
    } else {
        (positions, mean_vec[(crab_count as usize + 1) / 2])
    }
}

fn get_lowcost_position2(input: &str) -> (Vec<u16>, u16, u32) {
    let p = get_lowcost_position(input);
    let mut positions = p.0;
    positions.sort_unstable();
    let mean = p.1;

    let crab_count = positions.len() as u16;

    // starting at mean pos and decide for bigger side
    let go_up = positions[crab_count as usize - 1] - positions[mean as usize]
        >= positions[mean as usize] - positions[0];

    let mut pos = mean;
    let mut cur_cost = calc_cost2(&positions, pos);
    let mut check_cost = cur_cost;

    if go_up {
        while pos < crab_count - 1 {
            cur_cost = check_cost;
            check_cost = calc_cost2(&positions, pos + 1);
            if check_cost > cur_cost {
                break;
            }
            pos += 1;
        }
    } else {
        // go down
        while pos > 0 {
            cur_cost = check_cost;
            check_cost = calc_cost2(&positions, pos - 1);
            if check_cost > cur_cost {
                break;
            }
            pos -= 1;
        }
    }

    (positions, pos, cur_cost)
}

fn calc_cost(positions: &Vec<u16>, pos: u16) -> u32 {
    positions
        .iter()
        .map(|&p| {
            // moves
            if p > pos {
                (p - pos) as u32
            } else {
                (pos - p) as u32
            }
        })
        .sum::<u32>()
}

#[inline(always)]
fn calc_cost2(positions: &Vec<u16>, pos: u16) -> u32 {
    positions
        .iter()
        .map(|&p| {
            // moves
            let diff = if p > pos {
                (p - pos) as u32
            } else {
                (pos - p) as u32
            };
            diff * (diff + 1) / 2
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_get_lowcost_position() {
        let pos = get_lowcost_position(TEST_INPUT);
        assert_eq!(pos.1, 2);
    }

    #[test]
    fn test_calc_cost() {
        let pos = get_lowcost_position(TEST_INPUT);
        let cost = calc_cost(&pos.0, pos.1);
        assert_eq!(cost, 37);
    }

    #[test]
    fn test_get_lowcost_position2() {
        let pos = get_lowcost_position2(TEST_INPUT);
        assert_eq!(pos.1, 5);
        assert_eq!(pos.2, 168);
    }
}
