#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

const TAGS_OPEN: &[char] = &['(', '[', '{', '<'];
const TAGS_CLOSE: &[char] = &[')', ']', '}', '>'];
const TAG_SCORE: &[u16] = &[3, 57, 1197, 25137];
const TAG_AUTO_SCORE: &[u16] = &[1, 2, 3, 4];

fn main() {
    let input = get_input(INPUT_FILE);

    let error_scores = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let res = validate_delimiter(line);
            if res.is_err() {
                Some(res.unwrap_err().0)
            } else {
                None
            }
        })
        .collect::<Vec<u16>>();

    println!("error score: {}", sum_scores(&error_scores));

    let mut autocomplete_scores = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if let Some(scores) = autocomplete_delimiter(line) {
                Some(calc_autocomplete_score(&scores))
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    autocomplete_scores.sort_unstable();
    let score = autocomplete_scores[(autocomplete_scores.len() - 1) >> 1];

    println!("autocomplete score: {}", score);
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

fn validate_delimiter(s: &str) -> Result<&str, (u16, String)> {
    if s.is_empty() {
        Ok(s)
    } else {
        let mut tags = Vec::<u8>::new();
        let mut error_tag: Option<(char, u8)> = None;
        if s.chars().any(|c| {
            let is_invalid = if let Some(open_tag_idx) =
                TAGS_OPEN.iter().position(|open_c| *open_c == c)
            {
                tags.push(open_tag_idx as u8);
                false
            } else {
                if TAGS_CLOSE.contains(&c) {
                    let idx = TAGS_CLOSE.iter().position(|close_c| *close_c == c).unwrap() as u8;
                    if tags.last().is_none() || *tags.last().unwrap() != idx {
                        error_tag.replace((c, idx));
                        true
                    } else {
                        tags.pop();
                        false
                    }
                } else {
                    false
                }
            };
            is_invalid
        }) {
            let score = TAG_SCORE[error_tag.unwrap().1 as usize];
            Err((
                score,
                format!(
                    "delimiter {} has no matching open tag",
                    error_tag.unwrap().0
                ),
            ))
        } else {
            Ok(s)
        }
    }
}

#[inline(always)]
fn sum_scores(scores: &Vec<u16>) -> u32 {
    scores.iter().map(|score| *score as u32).sum::<u32>()
}

fn autocomplete_delimiter(s: &str) -> Option<Vec<u8>> {
    if s.is_empty() {
        None
    } else {
        let mut tags = Vec::<u8>::new();
        let mut error_tag: Option<(char, u8)> = None;
        if s.chars().any(|c| {
            let is_invalid = if let Some(open_tag_idx) =
                TAGS_OPEN.iter().position(|open_c| *open_c == c)
            {
                tags.push(open_tag_idx as u8);
                false
            } else {
                if TAGS_CLOSE.contains(&c) {
                    let idx = TAGS_CLOSE.iter().position(|close_c| *close_c == c).unwrap() as u8;
                    if tags.last().is_none() || *tags.last().unwrap() != idx {
                        error_tag.replace((c, idx));
                        true
                    } else {
                        tags.pop();
                        false
                    }
                } else {
                    false
                }
            };
            is_invalid
        }) {
            None
        } else {
            tags.reverse();
            Some(tags)
        }
    }
}

#[inline(always)]
fn calc_autocomplete_score(tags: &Vec<u8>) -> u64 {
    let mut score = 0u64;

    tags.iter().for_each(|tag| {
        score = score.checked_mul(5).unwrap();
        score = score
            .checked_add(TAG_AUTO_SCORE[*tag as usize] as u64)
            .unwrap();
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
                               [(()[<>])]({[<{<<[]>>(
                               {([(<{}[<>[]}>{[]{[(<()>
                               (((({<>}<{<{<>}{[]{[]{}
                               [[<[([]))<([[{}[[()]]]
                               [{[{({}]{}}([{[{{{}}([]
                               {<[[]]>}<{[{[{[]{()[[[]
                               [<(<(<(<{}))><([]([]()
                               <{([([[(<>()){}]>(<<{{
                               <{([{{}}[<[[[<>{}]]]>[]]
                              ";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_validate_delimiter() {
        let input = r"()
                           []
                           ([])
                           {()()()}
                           <([{}])>
                           [<>({}){}[([])<>]]
                           (((((((((())))))))))
                          ";
        input.lines().for_each(|line| {
            let line = line.trim();
            let res = validate_delimiter(line);
            assert!(res.is_ok(), "error: '{}'", res.unwrap_err().0);
        });

        let input = r"(]
                           {()()()>
                           (((()))}
                           <([]){()}[{}])
                          ";
        input.lines().for_each(|line| {
            let line = line.trim();
            let res = validate_delimiter(line);
            assert!(
                line.is_empty() || res.is_err(),
                "should be error: '{}'",
                res.unwrap()
            );
        });

        let input = TEST_INPUT;
        let error_scores = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let res = validate_delimiter(line);
                if res.is_err() {
                    Some(res.unwrap_err().0)
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>();

        assert_eq!(error_scores.len(), 5);
        assert_eq!(sum_scores(&error_scores), 26397);
    }

    #[test]
    fn test_autocomplete_score() {
        let line = r"<{([{{}}[<[[[<>{}]]]>[]]"; // completes with ])}>

        let score = if let Some(scores) = autocomplete_delimiter(line) {
            calc_autocomplete_score(&scores)
        } else {
            0
        };
        assert_eq!(score, 294);
    }

    #[test]
    fn test_autocomplete_delimiter() {
        let input = TEST_INPUT;
        let mut autocomplete_scores = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if let Some(scores) = autocomplete_delimiter(line) {
                    Some(calc_autocomplete_score(&scores))
                } else {
                    None
                }
            })
            .collect::<Vec<u64>>();

        autocomplete_scores.sort_unstable();
        let score = autocomplete_scores[(autocomplete_scores.len() - 1) >> 1];

        assert_eq!(score, 288957);
    }
}
