#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{cell::RefCell, collections::HashSet, fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

const END: &str = "end";
const START: &str = "start";

thread_local! {
    static VISITED_CAVES: RefCell<HashSet<String>>  = RefCell::new(HashSet::new());
}

fn main() {
    let input = get_input(INPUT_FILE);

    let paths = path_traverse(&input, START, 1);
    println!("Number of paths: {}", paths.len());

    let paths = path_traverse(&input, START, 2);
    println!("Part 2 number of paths: {}", paths.len());
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

fn path_traverse(input: &str, lookup_src: &str, small_cave_visits: usize) -> Vec<String> {
    let mut paths = Vec::<String>::new();

    // part 2 variant with counting small caves - slows it down
    let mut small_caves_set = HashSet::<&str>::new();
    let mut small_cave = None;
    let mut cur_small_cave_visits = 0;
    cur_small_cave_visits += lookup_src
        .split(',')
        .filter(|&cave| {
            if cave != START && cave.to_lowercase() == cave {
                if let Some(small_cave) = small_cave {
                    cave == small_cave || !small_caves_set.insert(cave)
                } else if !small_caves_set.insert(cave) {
                    small_cave.replace(cave);
                    cur_small_cave_visits += 1; // add the last one
                    true // count more than 1 visits
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count();

    let is_allowed = cur_small_cave_visits <= small_cave_visits;

    if is_allowed {
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }

                let s = line.split_once('-').unwrap();
                let src = s.0;
                let dst = s.1;

                if dst == START || src == END {
                    Some(vec![[dst, src]])
                } else if src != START && dst != END {
                    Some(vec![[src, dst], [dst, src]])
                } else {
                    Some(vec![[src, dst]])
                }
            })
            .flatten()
            .for_each(|slice| {
                let src = slice[0];
                let dst = slice[1];

                let cur_cave_src = lookup_src.split(',').last().unwrap();

                // the old part one variant
                // let is_allowed = dst.to_lowercase() != dst
                //     || !(lookup_src.ends_with(&(",".to_string() + dst))
                //         || lookup_src.contains(&(",".to_string() + dst + ",")));

                if cur_cave_src != END && src == cur_cave_src && is_allowed {
                    if dst == END {
                        paths.push(lookup_src.to_string() + "," + END);
                    } else {
                        let dst_lookup = lookup_src.to_string() + "," + dst;
                        paths.extend(path_traverse(input, &dst_lookup, small_cave_visits));
                    }
                }
            });
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: &[&str] = &[
        r"start-A
          start-b
          A-c
          A-b
          b-d
          A-end
          b-end
         ",
        r"dc-end
          HN-start
          start-kj
          dc-start
          dc-HN
          LN-dc
          HN-end
          kj-sa
          kj-HN
          kj-dc
         ",
        r"fs-end
          he-DX
          fs-he
          start-DX
          pj-DX
          end-zg
          zg-sl
          zg-pj
          pj-he
          RW-he
          fs-DX
          pj-RW
          zg-RW
          start-pj
          he-WI
          zg-he
          pj-fs
          start-RW
         ",
    ];
    const RESULTS: &[&str] = &[
        r"start,A,b,A,c,A,end
          start,A,b,A,end
          start,A,b,end
          start,A,c,A,b,A,end
          start,A,c,A,b,end
          start,A,c,A,end
          start,A,end
          start,b,A,c,A,end
          start,b,A,end
          start,b,end
         ",
        r"start,HN,dc,HN,end
          start,HN,dc,HN,kj,HN,end
          start,HN,dc,end
          start,HN,dc,kj,HN,end
          start,HN,end
          start,HN,kj,HN,dc,HN,end
          start,HN,kj,HN,dc,end
          start,HN,kj,HN,end
          start,HN,kj,dc,HN,end
          start,HN,kj,dc,end
          start,dc,HN,end
          start,dc,HN,kj,HN,end
          start,dc,end
          start,dc,kj,HN,end
          start,kj,HN,dc,HN,end
          start,kj,HN,dc,end
          start,kj,HN,end
          start,kj,dc,HN,end
          start,kj,dc,end
         ",
        r"226",
    ];

    const RESULTS_2: &[&str] = &[
        r"start,A,b,A,b,A,c,A,end
          start,A,b,A,b,A,end
          start,A,b,A,b,end
          start,A,b,A,c,A,b,A,end
          start,A,b,A,c,A,b,end
          start,A,b,A,c,A,c,A,end
          start,A,b,A,c,A,end
          start,A,b,A,end
          start,A,b,d,b,A,c,A,end
          start,A,b,d,b,A,end
          start,A,b,d,b,end
          start,A,b,end
          start,A,c,A,b,A,b,A,end
          start,A,c,A,b,A,b,end
          start,A,c,A,b,A,c,A,end
          start,A,c,A,b,A,end
          start,A,c,A,b,d,b,A,end
          start,A,c,A,b,d,b,end
          start,A,c,A,b,end
          start,A,c,A,c,A,b,A,end
          start,A,c,A,c,A,b,end
          start,A,c,A,c,A,end
          start,A,c,A,end
          start,A,end
          start,b,A,b,A,c,A,end
          start,b,A,b,A,end
          start,b,A,b,end
          start,b,A,c,A,b,A,end
          start,b,A,c,A,b,end
          start,b,A,c,A,c,A,end
          start,b,A,c,A,end
          start,b,A,end
          start,b,d,b,A,c,A,end
          start,b,d,b,A,end
          start,b,d,b,end
          start,b,end
        ",
        "103",
        "3509",
    ];

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_path_traverse() {
        (0..=2usize).for_each(|n| {
            let input = SAMPLES[n];
            let result = RESULTS[n];

            let result = if let Ok(result_len) = result.parse::<usize>() {
                let mut v = Vec::new();
                v.resize(result_len, "");
                v
            } else {
                result
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() {
                            None
                        } else {
                            Some(line)
                        }
                    })
                    .collect::<Vec<&str>>()
            };

            let paths = path_traverse(input, START, 1);
            assert_eq!(paths.len(), result.len());
        });
    }

    #[test]
    fn test_path_traverse_2() {
        (0..=2usize).for_each(|n| {
            let input = SAMPLES[n];
            let result = RESULTS_2[n];

            let result = if let Ok(result_len) = result.parse::<usize>() {
                let mut v = Vec::new();
                v.resize(result_len, "");
                v
            } else {
                result
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() {
                            None
                        } else {
                            Some(line)
                        }
                    })
                    .collect::<Vec<&str>>()
            };

            let paths = path_traverse(input, START, 2);
            assert_eq!(paths.len(), result.len());
        });
    }
}
