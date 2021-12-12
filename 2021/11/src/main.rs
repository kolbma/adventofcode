#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{cell::RefCell, collections::HashSet, fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";
const LCOUNT: usize = 10usize;
const LLENGTH: usize = 10usize;

thread_local! {
    static OCTOPUS_LEVELS: RefCell<[u8; LCOUNT*LLENGTH]> = RefCell::new([0; LCOUNT*LLENGTH]);
}

fn main() {
    let input = get_input(INPUT_FILE);

    detect_energy(&input);

    let mut flashes = 0u16;
    (0..100).for_each(|_| {
        flashes += energy_step();
    });

    println!("Number of flashes after 100 steps: {}", flashes);

    let steps = bright_flash_step(&input);

    println!("Brigh flash occurs after steps: {}", steps);
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

fn detect_energy(input: &str) {
    let mut idx = 0usize;
    input.lines().for_each(|line| {
        line.trim().chars().for_each(|c| {
            OCTOPUS_LEVELS.with(|levels| levels.borrow_mut()[idx] = (c as u8) - 48);
            idx += 1;
        });
    });
}

fn energy_step() -> u16 {
    let mut flash_indexes = HashSet::new();

    OCTOPUS_LEVELS.with(|levels| {
        {
            levels.borrow_mut().iter_mut().for_each(|level| *level += 1);
        }

        let mut is_changed = true;

        while is_changed {
            is_changed = false;
            levels
                .clone()
                .borrow()
                .iter()
                .enumerate()
                .for_each(|(idx, level)| {
                    if *level > 9 && flash_indexes.insert(idx) {
                        let mut y = 0usize;
                        let x = if idx < LLENGTH {
                            idx
                        } else {
                            let modulo = idx % LLENGTH;
                            y = (idx - modulo) / LLENGTH;
                            modulo
                        };
                        let y = y;

                        let mut neighbours = Vec::new();

                        if x > 0 {
                            neighbours.push(y * LLENGTH + x - 1);
                        }
                        if x < LLENGTH - 1 {
                            neighbours.push(y * LLENGTH + x + 1);
                        }
                        if y > 0 {
                            neighbours.push((y - 1) * LLENGTH + x);
                            if x > 0 {
                                neighbours.push((y - 1) * LLENGTH + x - 1);
                            }
                            if x < LLENGTH - 1 {
                                neighbours.push((y - 1) * LLENGTH + x + 1);
                            }
                        }
                        if y < LCOUNT - 1 {
                            neighbours.push((y + 1) * LLENGTH + x);
                            if x > 0 {
                                neighbours.push((y + 1) * LLENGTH + x - 1);
                            }
                            if x < LLENGTH - 1 {
                                neighbours.push((y + 1) * LLENGTH + x + 1);
                            }
                        }

                        if neighbours.len() > 0 {
                            is_changed = true;
                            neighbours.iter().for_each(|neighbour| {
                                levels.borrow_mut()[*neighbour] += 1;
                            });
                        }
                    }
                });
        }

        levels.borrow_mut().iter_mut().for_each(|level| {
            if *level > 9 {
                *level = 0;
            }
        });
    });

    flash_indexes.len() as u16
}

fn bright_flash_step(input: &str) -> u16 {
    detect_energy(input);

    let mut step = 0u16;
    loop {
        step = step.checked_add(1).unwrap();
        let flashes = energy_step();
        if flashes == (LLENGTH * LCOUNT) as u16 {
            break;
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"5483143223
                               2745854711
                               5264556173
                               6141336146
                               6357385478
                               4167524645
                               2176841721
                               6882881134
                               4846848554
                               5283751526
                              ";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_detect_energy() {
        let input = TEST_INPUT;

        detect_energy(input);

        OCTOPUS_LEVELS.with(|levels| {
            let levels = levels.borrow();
            assert_eq!(levels[0], 5);
            assert_eq!(levels[9], 3);
            assert_eq!(levels[10], 2);
            assert_eq!(levels[23], 4);
            assert_eq!(levels[49], 8);
            assert_eq!(levels[99], 6);
        });
    }

    #[test]
    fn test_energy_step() {
        let input = TEST_INPUT;

        detect_energy(input);

        let mut flashes = 0u16;
        flashes += energy_step();
        assert_eq!(flashes, 0);

        flashes += energy_step();
        assert_eq!(flashes, 35);

        (0..8).for_each(|_| {
            flashes += energy_step();
        });
        assert_eq!(flashes, 204);

        (0..90).for_each(|_| {
            flashes += energy_step();
        });
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_bright_flash_step() {
        let input = TEST_INPUT;

        let steps = bright_flash_step(input);
        assert_eq!(steps, 195);
    }
}
