#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{collections::HashSet, fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

// unique 1, 4, 7, 8
// 1 => len 2
// 4 => len 4
// 7 => len 3
// 8 => len 7
// 2, 3, 5 => len 5
// 0, 6, 9 => len 6

const SEGMENTS_LEN: &[u8] = &[6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

//  0000
// 1    2
// 1    2
//  3333    SEGMENTS and segment_map index numbers
// 4    5
// 4    5
//  6666
const SEGMENTS: &[&[u8]] = &[
    &[0, 1, 2, 4, 5, 6],    // 0
    &[2, 5],                // 1
    &[0, 2, 3, 4, 6],       // 2
    &[0, 2, 3, 5, 6],       // 3
    &[1, 2, 3, 5],          // 4
    &[0, 1, 3, 5, 6],       // 5
    &[0, 1, 3, 4, 5, 6],    // 6
    &[0, 2, 5],             // 7
    &[0, 1, 2, 3, 4, 5, 6], // 8
    &[0, 1, 2, 3, 5, 6],    // 9
];

fn main() {
    let input = get_input(INPUT_FILE);
    let count = count_output_digits(&input, &[1, 4, 7, 8]);
    println!("Digits 1, 4, 7, 8 appearance {}", count);

    let output = mapped_output(&input);
    let sum = output
        .iter()
        .map(|out| {
            *out as usize
        })
        .sum::<usize>();

    println!("Part 2 output sum result: {}", sum);
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

fn count_output_digits(input: &str, digits: &[u8]) -> usize {
    input
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|&out| {
                    let digit_len = digits
                        .iter()
                        .map(|digit| SEGMENTS_LEN[*digit as usize])
                        .collect::<Vec<u8>>();

                    digit_len.contains(&(out.len() as u8))
                })
                .count()
        })
        .sum::<usize>()
}

// sample 1
//     &['a', 'b', 'c', 'e', 'f', 'g'],      // 0
//     &['c', 'f'],                          // 1
//     &['a', 'c', 'd', 'e', 'g'],           // 2
//     &['a', 'c', 'd', 'f', 'g'],           // 3
//     &['b', 'c', 'd', 'f'],                // 4
//     &['a', 'b', 'd', 'f', 'g'],           // 5
//     &['a', 'b', 'd', 'e', 'f', 'g'],      // 6
//     &['a', 'c', 'f'],                     // 7
//     &['a', 'b', 'c', 'd', 'e', 'f', 'g'], // 8
//     &['a', 'b', 'c', 'd', 'f', 'g'],      // 9

// sample 2
//     &['a', 'b', 'c', 'd', 'e', 'g'],      // 0
//     &['a', 'b'],                          // 1
//     &['a', 'c', 'd', 'f', 'g'],           // 2
//     &['a', 'b', 'c', 'd', 'f'],           // 3
//     &['a', 'b', 'e', 'f'],                // 4
//     &['b', 'c', 'd', 'e', 'f'],           // 5
//     &['b', 'c', 'd', 'e', 'f', 'g'],      // 6
//     &['a', 'b', 'd'],                     // 7
//     &['a', 'b', 'c', 'd', 'e', 'f', 'g'], // 8
//     &['a', 'b', 'c', 'd', 'e', 'f'],      // 9

fn mapped_output(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| {
            let mut values = line
                .split('|')
                .map(|values| values.trim().split(' '))
                .flatten()
                .collect::<Vec<&str>>();

            // output digits
            let digits_out = &[
                values[values.len() - 4],
                values[values.len() - 3],
                values[values.len() - 2],
                values[values.len() - 1],
            ];

            // sort with longest first for faster excluding
            values.sort_unstable_by(|&a, &b| b.len().cmp(&a.len()));
            let values = values;

            let mut digits: [Vec<char>; 10] = Default::default();
            digits[8].extend_from_slice(&['a', 'b', 'c', 'd', 'e', 'f', 'g']); // always all possibilities
            let mut digits_count = 1u8;

            // possible values because of value length with 1, 4, 7 at the beginning
            let digits_indexes = [
                vec![1, 4, 7usize],
                values
                    .iter()
                    .map(|&value| {
                        SEGMENTS_LEN
                            .iter()
                            .enumerate()
                            .filter_map(|(digit_idx, &length)| {
                                if ![1, 4, 7].contains(&digit_idx) && length == value.len() as u8 {
                                    Some(digit_idx)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<usize>>()
                    })
                    .flatten()
                    .collect::<Vec<usize>>(),
            ]
            .concat();

            // container for currently all possible mappings
            //  0000
            // 1    2
            // 1    2
            //  3333    SEGMENTS and segment_map index numbers
            // 4    5
            // 4    5
            //  6666
            let mut segment_map: [HashSet<char>; 7] = Default::default();

            // try to find single length values for unique mappings
            digits_indexes.iter().for_each(|&digit_idx| {
                if segment_map.iter().flatten().collect::<Vec<&char>>().len() == 7 {
                    return; // segment_map finished
                }

                let length = &SEGMENTS_LEN[digit_idx];
                values
                    .iter()
                    .filter(|&&value| value.len() as u8 == *length)
                    .for_each(|&value| {
                        if segment_map.iter().flatten().collect::<Vec<&char>>().len() == 7 {
                            return; // segment_map finished
                        }

                        digits[digit_idx] = value.chars().collect();
                        digits[digit_idx].sort_unstable();
                        digits_count += 1;

                        // collect possible segments for updating
                        let update_segments = SEGMENTS[digit_idx]
                            .iter()
                            .filter(|&segmap_idx| segment_map[*segmap_idx as usize].is_empty())
                            .collect::<Vec<&u8>>();

                        // collect possible segment chars for mapping
                        let segmap_chars = update_segments
                            .iter()
                            .map(|_| {
                                digits[digit_idx]
                                    .iter()
                                    .filter(|&c| {
                                        segment_map
                                            .iter()
                                            .flatten()
                                            .find(|&map_c| *map_c == *c)
                                            .is_none()
                                    })
                                    .collect::<Vec<&char>>()
                            })
                            .collect::<Vec<Vec<&char>>>();

                        // populate segment_map
                        segmap_chars.iter().for_each(|c_vec| {
                            c_vec.into_iter().for_each(|&c| {
                                // update segments with char values not already found in other segments
                                update_segments.iter().for_each(|&segmap_idx| {
                                    segment_map[*segmap_idx as usize].insert(*c);
                                })
                            });
                        });

                        // find missing segment chars in digits[n]
                        let mut segment_chars_miss = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
                        segment_chars_miss.retain(|map_idx| !digits[digit_idx].contains(map_idx));
                        let segment_chars_miss = segment_chars_miss;

                        // remove missing segment chars from missing segments of digits[n]
                        let mut segments_miss = SEGMENTS[8].to_vec();
                        segments_miss
                            .retain(|segmap_idx| !SEGMENTS[digit_idx].contains(segmap_idx));

                        segments_miss.iter().for_each(|segmap_idx| {
                            let cur_segment_map = &mut segment_map[*segmap_idx as usize];
                            if cur_segment_map.len() > 1 {
                                let multi_count = cur_segment_map
                                    .iter()
                                    .filter(|&mcheck_c| segment_chars_miss.contains(mcheck_c))
                                    .count()
                                    as u8;

                                // check unique possibility
                                if multi_count == 1 {
                                    cur_segment_map
                                        .retain(|map_c| segment_chars_miss.contains(map_c));
                                }
                            }

                            // remove found segment chars from other segments
                            let mut check_indexes = vec![segmap_idx.clone()];
                            while check_indexes.len() > 0 {
                                check_indexes.clone().iter().for_each(|&check_idx| {
                                    check_indexes.remove(0);
                                    if segment_map[check_idx as usize].len() == 1 {
                                        let remove_c = segment_map[check_idx as usize]
                                            .iter()
                                            .next()
                                            .unwrap()
                                            .clone();
                                        (0..=6u8).for_each(|check_idx2| {
                                            if check_idx2 != check_idx {
                                                if segment_map[check_idx2 as usize]
                                                    .remove(&remove_c)
                                                    && segment_map[check_idx2 as usize].len() == 1
                                                {
                                                    check_indexes.push(check_idx2);
                                                }
                                            }
                                        })
                                    }
                                });
                            }
                        });
                    });
            });

            let digits = digits_out
                .iter()
                .map(|&out_str| {
                    let out = out_str.chars().collect::<Vec<char>>();

                    let mut segment_cmp = out
                        .iter()
                        .map(|out_c| {
                            segment_map
                                .iter()
                                .position(|c| c.get(out_c).is_some())
                                .unwrap() as u8
                        })
                        .collect::<Vec<u8>>();

                    segment_cmp.sort_unstable();
                    let segment_cmp = segment_cmp;

                    let digit = SEGMENTS
                        .iter()
                        .position(|&segments| segments == segment_cmp)
                        .expect(&format!("{:?} => {:?}", out, segment_cmp))
                        as u8;

                    digit
                })
                .collect::<Vec<u8>>();

            digits[0] as u16 * 1000
                + digits[1] as u16 * 100
                + digits[2] as u16 * 10
                + digits[3] as u16
        })
        .collect::<Vec<u16>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
                               be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                               edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                               fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                               fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                               aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                               fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                               dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                               bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                               egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                               gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const TEST_OUTPUT: &[u16] = &[
        5353, 8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315,
    ];

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_count_output_digits() {
        // ignore first entry
        let input = TEST_INPUT.split_once(char::is_control).unwrap().1;

        let count = count_output_digits(input, &[1, 4, 7, 8]);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_mapped_output() {
        let output = mapped_output(TEST_INPUT);

        let mut n = 0;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 1;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 2;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 3;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 4;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 5;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 6;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 7;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 8;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 9;
        assert_eq!(output[n], TEST_OUTPUT[n]);
        n = 10;
        assert_eq!(output[n], TEST_OUTPUT[n]);

        assert_eq!(
            output[1..].iter().map(|out| *out as usize).sum::<usize>(),
            61229
        );
    }
}
