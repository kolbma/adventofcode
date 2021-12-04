#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";
const HORIZ_SIZE: usize = 5;
const VERT_SIZE: usize = 5;
const BOARD_SIZE: usize = HORIZ_SIZE * VERT_SIZE;
const BOARDS_MAX: usize = 100;

#[derive(Debug, PartialEq)]
enum Direction {
    Row,
    Col,
}

type BingoNumbers = Vec<String>;
type Boards = Vec<String>;

#[inline(always)]
fn is_row_checked(boards: &Vec<bool>, board: usize, nr: usize) -> bool {
    let board_start = board * BOARD_SIZE;

    let row_start = nr * HORIZ_SIZE + board_start;
    let row_end = row_start + HORIZ_SIZE;

    for check in boards[row_start..row_end].iter() {
        if !check {
            return false;
        }
    }

    return true;
}

#[inline(always)]
fn is_col_checked(boards: &Vec<bool>, board: usize, nr: usize) -> bool {
    let board_start = board * BOARD_SIZE;

    let col_start = nr + board_start;

    for i in 0..VERT_SIZE {
        let idx = col_start + i * HORIZ_SIZE;
        let check = boards[idx];
        if !check {
            return false;
        }
    }

    return true;
}

#[inline(always)]
fn fill_boards() -> (BingoNumbers, Boards, usize) {
    // read text file
    let mut file = File::open(INPUT_FILE).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = contents.lines();
    // first line has called bingo numbers
    let numbers = lines.next().unwrap();
    // split line by comma to get ordered list of bingo numbers
    let numbers = numbers.split(",").map(|s| s.to_string()).collect();

    let mut board_count = -1i8;
    let mut boards = vec![String::new(); BOARD_SIZE * BOARDS_MAX];
    let mut boards_idx = 0usize;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            board_count += 1;
            continue;
        }

        line.split_whitespace().for_each(|s| {
            boards[boards_idx] = s.to_owned();
            boards_idx += 1;
        });
    }

    (numbers, boards, board_count as usize)
}

#[inline(always)]
fn find_bingo(
    boards: &Boards,
    board_count: usize,
    numbers: &BingoNumbers,
) -> Result<(usize, usize, Direction, u32), String> {
    let mut boards_check = vec![false; BOARD_SIZE * BOARDS_MAX];

    for number in numbers {
        for n in 0..boards.len() {
            if number.eq(&boards[n]) {
                boards_check[n] = true;
            }
        }

        for board_nr in 0..board_count {
            // TODO: this works because HORIZ_SIZE == VERT_SIZE
            for row_nr in 0..HORIZ_SIZE {
                let col_nr = row_nr;
                if is_row_checked(&boards_check, board_nr, row_nr) {
                    let res = calc_result(boards, board_nr, &boards_check, &number.to_owned());
                    return Ok((board_nr, row_nr, Direction::Row, res));
                } else if is_col_checked(&boards_check, board_nr, col_nr) {
                    let res = calc_result(boards, board_nr, &boards_check, &number.to_owned());
                    return Ok((board_nr, col_nr, Direction::Col, res));
                }
            }
        }
    }

    Err("no bingo found".into())
}

#[inline(always)]
fn find_last_bingo(
    boards: &Boards,
    board_count: usize,
    numbers: &BingoNumbers,
) -> Result<(usize, usize, Direction, u32), String> {
    let mut boards_check = vec![false; BOARD_SIZE * BOARDS_MAX];
    let mut boards_finished = vec![false; BOARDS_MAX];
    let mut last_board = None;
    let mut last_number = String::new();

    for number in numbers {
        for n in 0..boards.len() {
            if number.eq(&boards[n]) {
                boards_check[n] = true;
            }
        }

        for board_nr in 0..board_count {
            if boards_finished[board_nr] {
                continue;
            }
            // TODO: this works because HORIZ_SIZE == VERT_SIZE
            for row_nr in 0..HORIZ_SIZE {
                let col_nr = row_nr;
                if is_row_checked(&boards_check, board_nr, row_nr) {
                    boards_finished[board_nr] = true;
                    last_board.replace((board_nr, row_nr, Direction::Row));
                    break;
                } else if is_col_checked(&boards_check, board_nr, col_nr) {
                    boards_finished[board_nr] = true;
                    last_board.replace((board_nr, col_nr, Direction::Col));
                    break;
                }
            }
        }

        let finished_count = boards_finished
            .iter()
            .map(|finished| if *finished { 1usize } else { 0usize })
            .sum::<usize>();
        if finished_count == board_count {
            last_number = number.to_owned();
            break;
        }
    }

    if let Some(board_data) = last_board {
        let (board_nr, row_nr, direction) = board_data;
        let res = calc_result(boards, board_nr, &boards_check, &last_number);
        return Ok((board_nr, row_nr, direction, res));
    }
    Err("no last bingo found".into())
}

#[inline(always)]
fn calc_result(boards: &Boards, board_nr: usize, boards_check: &Vec<bool>, bingo_nr: &str) -> u32 {
    let board_start = board_nr * BOARD_SIZE;
    let board_end = board_start + BOARD_SIZE;

    let mut nr_sum = 0u32;
    let mut idx = board_start;
    for check in boards_check[board_start..board_end].iter() {
        if !check {
            let nr = &boards[idx];
            nr_sum += nr.parse::<u32>().unwrap();
        }
        idx += 1;
    }
    nr_sum * bingo_nr.parse::<u32>().unwrap()
}

fn main() {
    let (numbers, boards, board_count) = fill_boards();

    let res = find_bingo(&boards, board_count, &numbers);

    let (board_nr, row_nr, direction, result) = res.unwrap();
    let direction = match direction {
        Direction::Row => "row",
        Direction::Col => "col",
    };

    println!("Board {}, {} {}", board_nr + 1, row_nr + 1, direction);
    println!("Result {}", result);

    let res = find_last_bingo(&boards, board_count, &numbers);

    let (board_nr, row_nr, direction, result) = res.unwrap();
    let direction = match direction {
        Direction::Row => "row",
        Direction::Col => "col",
    };

    println!("Last Board {}, {} {}", board_nr + 1, row_nr + 1, direction);
    println!("Result {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bingo() {
        let (_numbers, boards, board_count) = fill_boards();
        let numbers = vec![
            "66".to_string(),
            "78".to_string(),
            "7".to_string(),
            "45".to_string(),
            "92".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (0, 0, Direction::Row, 103408u32));

        let numbers = vec![
            "39".to_string(),
            "38".to_string(),
            "62".to_string(),
            "81".to_string(),
            "77".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (0, 1, Direction::Row, 85855u32));

        let numbers = vec![
            "47".to_string(),
            "66".to_string(),
            "71".to_string(),
            "17".to_string(),
            "69".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (1, 0, Direction::Col, 71829u32));

        let numbers = vec![
            "11".to_string(),
            "6".to_string(),
            "83".to_string(),
            "91".to_string(),
            "87".to_string(),
            "38".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (2, 4, Direction::Col, 42028u32));

        let numbers = vec![
            "11".to_string(),
            "6".to_string(),
            "83".to_string(),
            "91".to_string(),
            "87".to_string(),
            "38".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (2, 4, Direction::Col, 42028u32));

        let numbers = vec![
            "11".to_string(),
            "6".to_string(),
            "73".to_string(),
            "81".to_string(),
            "87".to_string(),
            "8".to_string(),
            "1".to_string(),
            "2".to_string(),
            "59".to_string(),
            "7".to_string(),
            "16".to_string(),
            "3".to_string(),
        ];
        let res = find_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (6, 2, Direction::Col, 14912u32));
    }

    #[test]
    fn test_find_last_bingo() {
        let (numbers, boards, board_count) = fill_boards();
        let res = find_last_bingo(&boards, board_count, &numbers);
        assert_eq!(res.unwrap(), (58, 2, Direction::Row, 7686u32));
    }
}
