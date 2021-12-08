#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

type Row = u8;
type Col = u8;
type Seat = u16;

fn main() {
    let input = get_input(INPUT_FILE);

    let mut seats = input
        .lines()
        .map(|line| {
            let bnr = parse_boarding_nr(line.trim());
            bnr.2 as Seat
        })
        .collect::<Vec<Seat>>();
    seats.sort_unstable();
    seats.reverse();
    let seats = seats;

    let max_seat = seats[0];

    println!("Highest seat ID {}", max_seat);

    for i in 0..seats.len() {
        if seats[i] - seats[i + 1] > 1 {
            let my_seat = seats[i] - 1;
            println!("My seat ID {}", my_seat);
            break;
        }
    }
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

fn parse_boarding_nr(value: &str) -> (Row, Col, Seat) {
    let mut row_start = 0u8;
    let mut row_end = 127u8;
    let mut col_start = 0u8;
    let mut col_end = 7u8;
    let mut row = &row_end;
    let mut col = &col_end;

    let chars = value.chars().collect::<Vec<char>>();

    for n in 0..7usize {
        match chars[n] {
            'F' => {
                row_end = (row_end - row_start - 1) / 2 + row_start;
                row = &row_end;
            }
            'B' => {
                row_start = (row_end - row_start + 1) / 2 + row_start;
                row = &row_start;
            }
            _ => {}
        }
    }

    for n in 7..10usize {
        match chars[n] {
            'L' => {
                col_end = (col_end - col_start - 1) / 2 + col_start;
                col = &col_end;
            }
            'R' => {
                col_start = (col_end - col_start + 1) / 2 + col_start;
                col = &col_start;
            }
            _ => {}
        }
    }

    (*row, *col, *row as u16 * 8 + *col as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TEST_INPUT: &str = r"";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_parse_boarding_nr() {
        let bnr = "BFFFBBFRRR";
        let parsed = parse_boarding_nr(bnr);
        assert_eq!(parsed, (70, 7, 567));

        let bnr = "FFFBBBFRRR";
        let parsed = parse_boarding_nr(bnr);
        assert_eq!(parsed, (14, 7, 119));

        let bnr = "BBFFBBFRLL";
        let parsed = parse_boarding_nr(bnr);
        assert_eq!(parsed, (102, 4, 820));
    }
}
