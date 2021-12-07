#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

type Coord = (usize, usize);

fn main() {
    let input = get_input(INPUT_FILE);

    let input_vec = input_vec(&input);
    let tree_count = tree_encounter_count(&input_vec, 3, 1);

    println!("Encountered trees: {}", tree_count);

    let mut tree_count = vec![tree_encounter_count(&input_vec, 1, 1)];
    tree_count.push(tree_encounter_count(&input_vec, 3, 1));
    tree_count.push(tree_encounter_count(&input_vec, 5, 1));
    tree_count.push(tree_encounter_count(&input_vec, 7, 1));
    tree_count.push(tree_encounter_count(&input_vec, 1, 2));

    let tree_count_prod = tree_count.iter().product::<usize>();

    println!("Encountered trees product result: {}", tree_count_prod);
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
fn input_vec(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}

fn position(input_vec: &Vec<&str>, right: usize, down: usize) -> Coord {
    let width = input_vec[0].len();
    let pos_right = if right < width {
        right
    } else {
        right - right / width * width
    };

    let height = input_vec.len();
    let pos_down = if down < height { down } else { height };

    (pos_right, pos_down)
}

#[inline(always)]
fn is_tree(input_vec: &Vec<&str>, pos_right: usize, pos_down: usize) -> bool {
    input_vec[pos_down].chars().nth(pos_right) == Some('#')
}

fn tree_encounter_count(input_vec: &Vec<&str>, right: usize, down: usize) -> usize {
    let height = input_vec.len();
    let mut pos = (0, 0);
    let mut tree_count = 0;

    loop {
        pos = position(input_vec, pos.0 + right, pos.1 + down);
        if pos.1 >= height {
            break;
        }
        if is_tree(input_vec, pos.0, pos.1) {
            tree_count += 1;
        }
    }

    tree_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_input_vec() {
        let input_vec = input_vec(TEST_INPUT);
        assert_eq!(input_vec.len(), 11);
        assert_eq!(input_vec[0].len(), 11);
    }

    #[test]
    fn test_position() {
        let input_vec = input_vec(TEST_INPUT);
        let pos = position(&input_vec, 0, 0);
        assert_eq!(pos, (0, 0));
        let pos = position(&input_vec, 1, 0);
        assert_eq!(pos, (1, 0));
        let pos = position(&input_vec, 5, 3);
        assert_eq!(pos, (5, 3));
        let pos = position(&input_vec, 10, 0);
        assert_eq!(pos, (10, 0));
        let pos = position(&input_vec, 0, 10);
        assert_eq!(pos, (0, 10));
        let pos = position(&input_vec, 10, 10);
        assert_eq!(pos, (10, 10));
        let pos = position(&input_vec, 11, 10);
        assert_eq!(pos, (0, 10));
        let pos = position(&input_vec, 15, 0);
        assert_eq!(pos, (4, 0));
        let pos = position(&input_vec, 39, 0);
        assert_eq!(pos, (6, 0));
        let pos = position(&input_vec, 25, 12);
        assert_eq!(pos, (3, 11));
    }

    #[test]
    fn test_is_tree() {
        let input_vec = input_vec(TEST_INPUT);
        assert!(!is_tree(&&input_vec, 1, 0));
        assert!(is_tree(&&input_vec, 2, 0));
        assert!(is_tree(&&input_vec, 3, 0));
        assert!(is_tree(&&input_vec, 1, 10));
        assert!(!is_tree(&&input_vec, 2, 10));
    }

    #[test]
    fn test_tree_encounter_count() {
        let input_vec = input_vec(TEST_INPUT);
        let tree_count = tree_encounter_count(&input_vec, 3, 1);
        assert_eq!(tree_count, 7);
    }

    #[test]
    fn test_tree_encounter_count2() {
        let input_vec = input_vec(TEST_INPUT);
        let tree_count1 = tree_encounter_count(&input_vec, 1, 1);
        assert_eq!(tree_count1, 2);

        let tree_count2 = tree_encounter_count(&input_vec, 3, 1);
        assert_eq!(tree_count2, 7);

        let tree_count3 = tree_encounter_count(&input_vec, 5, 1);
        assert_eq!(tree_count3, 3);

        let tree_count4 = tree_encounter_count(&input_vec, 7, 1);
        assert_eq!(tree_count4, 4);

        let tree_count5 = tree_encounter_count(&input_vec, 1, 2);
        assert_eq!(tree_count5, 2);

        assert_eq!(
            tree_count1 * tree_count2 * tree_count3 * tree_count4 * tree_count5,
            336
        );
    }
}
