#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";
const FOLD_HORIZONTAL: char = 'x';
const FOLD_VERTICAL: char = 'y';

fn main() {
    let input = get_input(INPUT_FILE);

    let data = parse_data(&input);
    let mut dots = data.0;
    let folds = data.1;

    let fold_sample = folds[0];

    fold(&mut dots, fold_sample);

    println!("Dots after 1st fold: {}", count_dots(&dots));

    (1..folds.len()).for_each(|idx| {
        fold(&mut dots, folds[idx]);
    });

    println!("\nCode is...\n");

    dots.iter().for_each(|dots_line| {
        dots_line.iter().for_each(|is_dot| {
            if *is_dot {
                print!("X");
            } else {
                print!(" ");
            }
        });
        println!();
    });
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

fn parse_data(input: &str) -> (Vec<Vec<bool>>, Vec<(char, u16)>) {
    let mut folds = Vec::new();
    let mut dots = Vec::new();

    input.lines().for_each(|line| {
        let line = line.trim();

        if line.is_empty() {
            return;
        }

        if line.starts_with('f') {
            // parse fold
            let mut fold = line.splitn(3, ' ').last().unwrap().splitn(2, '=');
            folds.push((
                fold.next().unwrap().chars().next().unwrap(),
                fold.next().unwrap().parse::<u16>().unwrap(),
            ));
        } else {
            // parse dots
            let mut dot = line.splitn(2, ',');
            let x = dot.next().unwrap().parse::<usize>().unwrap();
            let y = dot.next().unwrap().parse::<usize>().unwrap();

            while dots.len() <= y {
                dots.push(Vec::new());
            }
            while dots[y].len() <= x {
                dots[y].push(false);
            }

            // mark dot
            dots[y][x] = true;
        }
    });

    (dots, folds)
}

fn fold(dots: &mut Vec<Vec<bool>>, fold: (char, u16)) {
    let fold_dir = fold.0;
    let fold_pos = fold.1 as usize;

    if fold_dir == FOLD_VERTICAL {
        ((fold_pos + 1)..dots.len()).for_each(|y| {
            (0..dots[y].len()).for_each(|x| {
                if dots[y][x] {
                    let y = fold_pos - (y - fold_pos);
                    while dots[y].len() <= x {
                        dots[y].push(false);
                    }
                    dots[y][x] = true;
                }
            });
        });
        dots.truncate(fold_pos);
    } else if fold_dir == FOLD_HORIZONTAL {
        (0..dots.len()).for_each(|y| {
            let dots_line = &mut dots[y];
            if dots_line.len() > fold_pos {
                (fold_pos..dots_line.len()).for_each(|x| {
                    if dots_line[x] {
                        let x = fold_pos - (x - fold_pos);
                        dots_line[x] = true;
                    }
                });
            }
            dots_line.truncate(fold_pos);
        });
    }
}

#[inline(always)]
fn count_dots(dots: &Vec<Vec<bool>>) -> usize {
    dots.iter()
        .map(|dots_line| dots_line.iter().filter(|&is_dot| *is_dot).count())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"6,10
                               0,14
                               9,10
                               0,3
                               10,4
                               4,11
                               6,0
                               6,12
                               4,1
                               0,13
                               10,12
                               3,4
                               3,0
                               8,4
                               1,10
                               2,14
                               8,10
                               9,0
                               
                               fold along y=7
                               fold along x=5
                              ";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_parse_data() {
        let input = TEST_INPUT;

        let data = parse_data(input);
        let dots = data.0;

        assert!(dots[0][3]);
        assert!(dots[1][4]);
        assert!(dots[12][10]);
        assert!(dots[14][0]);
        assert_eq!(dots[2].len(), 0);
        assert_eq!(dots[11].len(), 5);
    }

    #[test]
    fn test_fold() {
        let input = TEST_INPUT;

        let data = parse_data(input);
        let mut dots = data.0;
        let folds = data.1;

        let fold_sample = folds[0];

        fold(&mut dots, fold_sample);

        assert!(dots[0][3]);
        assert!(dots[1][4]);
        if fold_sample.0 == FOLD_VERTICAL {
            assert_eq!(dots.len(), fold_sample.1 as usize);
        } else if fold_sample.0 == FOLD_HORIZONTAL {
            assert!(dots[0].len() <= fold_sample.1 as usize);
        }

        assert_eq!(dots[2].len(), 11);
        assert_eq!(dots[3].len(), 5);
        assert!(dots[3][0]);
        assert!(!dots[3][3]);
        assert!(dots[3][4]);

        let fold_sample = folds[1];

        fold(&mut dots, fold_sample);

        if fold_sample.0 == FOLD_VERTICAL {
            assert_eq!(dots.len(), fold_sample.1 as usize);
        } else if fold_sample.0 == FOLD_HORIZONTAL {
            assert!(dots[0].len() == fold_sample.1 as usize);
        }

        (0..5).for_each(|x| assert!(dots[0][x]));
        (0..5).for_each(|x| assert!(dots[4][x]));
    }

    #[test]
    fn test_count_dots() {
        let input = TEST_INPUT;

        let data = parse_data(input);
        let mut dots = data.0;
        let folds = data.1;

        let fold_sample = folds[0];

        fold(&mut dots, fold_sample);
        assert_eq!(count_dots(&dots), 17);
    }
}
