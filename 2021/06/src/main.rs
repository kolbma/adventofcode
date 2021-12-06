#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

type FishType = usize;

fn main() {
    let input = get_input(INPUT_FILE);

    let mut fish = read_fish_population(&input);

    earth_rotation(&mut fish, 80);
    println!("Fish population size after 80 days {}", count(&fish));

    earth_rotation(&mut fish, 256 - 80);
    println!("Fish population size after 256 days {}", count(&fish));
}

#[inline(always)]
fn read_fish_population(input: &str) -> Vec<FishType> {
    let mut fish: Vec<FishType> = vec![0; 9];

    input.trim().split(',').for_each(|reproduction_timer| {
        let reproduction_timer = reproduction_timer.parse::<FishType>().unwrap();
        fish[reproduction_timer] += 1;
    });

    fish
}

fn birth(fish: &mut Vec<FishType>) {
    // midnight birth
    let fish_born = fish[0];
    for day in 1..=8 {
        fish[day - 1] = fish[day];
    }
    fish[6] += fish_born;
    fish[8] = fish_born;
}

fn earth_rotation(fish: &mut Vec<FishType>, days: u16) {
    for _ in 1..=days {
        birth(fish);
        // println!("{} fish len {}", n, fish.len());
    }
}

#[inline(always)]
fn count(fish: &Vec<FishType>) -> FishType {
    fish.iter().sum()
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

    const TEST_INPUT: &str = r"3,4,3,1,2";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_read_fish_population() {
        let fish = read_fish_population(TEST_INPUT);
        assert_eq!(fish[0], 0);
        assert_eq!(fish[1], 1);
        assert_eq!(fish[2], 1);
        assert_eq!(fish[3], 2);
        assert_eq!(fish[4], 1);
        assert_eq!(fish[5], 0);
        assert_eq!(fish[6], 0);
        assert_eq!(fish[7], 0);
        assert_eq!(fish[8], 0);
    }
    #[test]
    fn test_birth() {
        let mut fish = read_fish_population(TEST_INPUT);
        assert_eq!(fish.len(), 9);

        birth(&mut fish);
        assert_eq!(fish[0], 1);
        assert_eq!(fish[1], 1);
        assert_eq!(fish[2], 2);
        assert_eq!(fish[3], 1);
        assert_eq!(fish[4], 0);
        assert_eq!(fish[5], 0);
        assert_eq!(fish[6], 0);
        assert_eq!(fish[7], 0);
        assert_eq!(fish[8], 0);

        birth(&mut fish);
        assert_eq!(fish[0], 1);
        assert_eq!(fish[1], 2);
        assert_eq!(fish[2], 1);
        assert_eq!(fish[3], 0);
        assert_eq!(fish[4], 0);
        assert_eq!(fish[5], 0);
        assert_eq!(fish[6], 1);
        assert_eq!(fish[7], 0);
        assert_eq!(fish[8], 1);
    }

    #[test]
    fn test_earth_rotation() {
        let mut fish = read_fish_population(TEST_INPUT);
        earth_rotation(&mut fish, 5);
        assert_eq!(count(&fish), 10);
        earth_rotation(&mut fish, 10 - 5);
        assert_eq!(count(&fish), 12);
        earth_rotation(&mut fish, 14 - 10);
        assert_eq!(count(&fish), 20);
        earth_rotation(&mut fish, 18 - 14);
        assert_eq!(count(&fish), 26);
        earth_rotation(&mut fish, 80 - 18);
        assert_eq!(count(&fish), 5934);
    }

    #[test]
    fn test_earth_rotation_256days() {
        let mut fish = read_fish_population(TEST_INPUT);
        earth_rotation(&mut fish, 256);
        assert_eq!(count(&fish), 26984457539);
    }
}
