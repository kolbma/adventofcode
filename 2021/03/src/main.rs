#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

type BinType = u16;
const BIN_TYPE_BITS: usize = 16;

#[derive(Clone, Copy)]
struct Bin {
    data: BinType,
    data_len: usize,
}

impl Into<BinType> for Bin {
    fn into(self) -> BinType {
        self.data
    }
}

impl Into<BinType> for &Bin {
    fn into(self) -> BinType {
        self.data
    }
}

impl From<&str> for Bin {
    fn from(s: &str) -> Self {
        let mut data: BinType = 0;
        let mut pos = s.len();
        s.chars().for_each(|c| {
            pos -= 1;

            match c {
                '0' => data |= 0 << pos,
                '1' => data |= 1 << pos,
                _ => {}
            };
        });

        Self {
            data,
            data_len: s.len(),
        }
    }
}

struct BinContainer {
    data: Vec<Bin>,
    data_len: usize,
}

impl BinContainer {
    fn new(input: &str) -> Self {
        let mut data = Vec::new();
        input
            .lines()
            .for_each(|line| data.push(Bin::from(line.trim())));
        let data_len = if data.is_empty() { 0 } else { data[0].data_len };
        Self { data, data_len }
    }

    fn _count_zero_or_one(data: &Vec<&Bin>) -> [BinType; 2 * BIN_TYPE_BITS] {
        let mut count = [0; 2 * BIN_TYPE_BITS];

        data.iter().for_each(|&b| {
            let n: BinType = b.into();
            for i in 0..BIN_TYPE_BITS {
                let zero_or_one = (n >> i) & 1;
                let count_idx = i * 2;
                match zero_or_one {
                    0 => count[count_idx] += 1,
                    1 => count[count_idx + 1] += 1,
                    _ => {}
                }
            }
        });

        count
    }

    fn gamma(self: &Self) -> BinType {
        // most common values wins
        let count = BinContainer::_count_zero_or_one(&self.data.iter().collect());

        let mut res: BinType = 0;

        // works with fixed size data
        for i in 0..self.data_len {
            let count_idx = i * 2;
            if count[count_idx] < count[count_idx + 1] {
                res += 1 << i;
            }
        }

        res
    }

    fn epsilon(self: &Self) -> BinType {
        // least common values win
        let count = BinContainer::_count_zero_or_one(&self.data.iter().collect());

        let mut res = 0;

        // works with fixed size data
        for i in 0..self.data_len {
            let count_idx = i * 2;
            if count[count_idx] > count[count_idx + 1] {
                res += 1 << i;
            }
        }

        res
    }

    fn power_consumption(self: &Self) -> u64 {
        (self.epsilon() as u64)
            .checked_mul(self.gamma() as u64)
            .unwrap()
    }

    fn o2(self: &Self) -> BinType {
        // most common values wins
        let mut data: Vec<&Bin> = self.data.iter().collect();

        // works with fixed size data
        for i in (0..self.data_len).rev() {
            let count = BinContainer::_count_zero_or_one(&data);
            let count_idx = i * 2;
            if count[count_idx] > count[count_idx + 1] {
                // filter available zeroes
                data = data
                    .into_iter()
                    .filter(|&b| {
                        let n: BinType = b.into();
                        let zero_or_one = (n >> i) & 1;
                        zero_or_one == 0
                    })
                    .collect();
            } else {
                // filter available ones
                data = data
                    .into_iter()
                    .filter(|&b| {
                        let n: BinType = b.into();
                        let zero_or_one = (n >> i) & 1;
                        zero_or_one == 1
                    })
                    .collect();
            }
            if data.len() == 1 {
                break;
            }
        }

        data[0].into()
    }

    fn co2(self: &Self) -> BinType {
        // least common values wins
        let mut data: Vec<&Bin> = self.data.iter().collect();

        // works with fixed size data
        for i in (0..self.data_len).rev() {
            let count = BinContainer::_count_zero_or_one(&data);
            let count_idx = i * 2;
            if count[count_idx] <= count[count_idx + 1] {
                // filter available zeroes
                data = data
                    .into_iter()
                    .filter(|&b| {
                        let n: BinType = b.into();
                        let zero_or_one = (n >> i) & 1;
                        zero_or_one == 0
                    })
                    .collect();
            } else {
                // filter available ones
                data = data
                    .into_iter()
                    .filter(|&b| {
                        let n: BinType = b.into();
                        let zero_or_one = (n >> i) & 1;
                        zero_or_one == 1
                    })
                    .collect();
            }
            if data.len() == 1 {
                break;
            }
        }

        data[0].into()
    }

    fn life_support_rating(self: &Self) -> u64 {
        (self.o2() as u64).checked_mul(self.co2() as u64).unwrap()
    }
}

fn main() {
    let input = get_input(INPUT_FILE);

    let container = BinContainer::new(&input);
    let res = container.power_consumption();

    println!("Power consumption {}", res);

    let res = container.life_support_rating();

    println!("Life support rating {}", res);
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

    const TEST_INPUT: &str = r"00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_bin() {
        let s = "00001";
        let b = Bin::from(s);
        let n: BinType = b.into();
        assert_eq!(n, 1);

        let s = "00010";
        let b = Bin::from(s);
        let n: BinType = b.into();
        assert_eq!(n, 2);

        let s = "00011";
        let b = Bin::from(s);
        let n: BinType = b.into();
        assert_eq!(n, 3);

        let s = "10011";
        let b = &Bin::from(s);
        let n: BinType = b.into();
        assert_eq!(n, 19);
    }

    #[test]
    fn test_gamma() {
        let container = BinContainer::new(TEST_INPUT);
        let g = container.gamma();
        assert_eq!(g, 22);
    }

    #[test]
    fn test_epsilon() {
        let container = BinContainer::new(TEST_INPUT);
        let g = container.epsilon();
        assert_eq!(g, 9);
    }

    #[test]
    fn test_power_consumption() {
        let container = BinContainer::new(TEST_INPUT);
        let res = container.power_consumption();
        assert_eq!(res, 198);
    }

    #[test]
    fn test_o2() {
        let container = BinContainer::new(TEST_INPUT);
        let o2 = container.o2();
        assert_eq!(o2, 23)
    }

    #[test]
    fn test_co2() {
        let container = BinContainer::new(TEST_INPUT);
        let co2 = container.co2();
        assert_eq!(co2, 10)
    }

    #[test]
    fn test_life_support_rating() {
        let container = BinContainer::new(TEST_INPUT);
        let res = container.life_support_rating();
        assert_eq!(res, 230);
    }
}
