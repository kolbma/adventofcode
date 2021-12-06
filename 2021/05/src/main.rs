#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read, slice::Iter};

const INPUT_FILE: &str = "./data/input";

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        let c: Vec<u16> = s.split(',').map(|s| s.parse::<u16>().unwrap()).collect();
        Self { x: c[0], y: c[1] }
    }
}

struct Venture {
    pub start: Coordinate,
    pub end: Coordinate,
    pub direction: Direction,
}

impl Venture {
    fn new(start: &Coordinate, end: &Coordinate, diagonal: bool) -> Self {
        let direction = if start.x == end.x {
            Direction::Vertical
        } else if start.y == end.y {
            Direction::Horizontal
        } else if !diagonal {
            Direction::Ignore
        } else {
            let diff_x = if start.x >= end.x {
                start.x - end.x
            } else {
                end.x - start.x
            };
            let diff_y = if start.y >= end.y {
                start.y - end.y
            } else {
                end.y - start.y
            };

            if diff_y == diff_x {
                Direction::Diagonal
            } else {
                println!("warning: ignoring {:?} -> {:?}", start, end);
                Direction::Ignore
            }
            // panic!("non binary venture coordinates");
        };

        Self {
            start: *start,
            end: *end,
            direction,
        }
    }
}

struct VentureField {
    pub coordinate: Coordinate,
    pub status: u8,
}

impl VentureField {
    fn new(coordinate: &Coordinate) -> Self {
        Self {
            coordinate: *coordinate,
            status: 1,
        }
    }
}

struct VentureHandler {
    fields: Vec<VentureField>,
}

impl VentureHandler {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }

    fn push(self: &mut Self, coordinate: &Coordinate) {
        // TODO: this is not thread safe (but we don't use threads)
        let mut exists = false;
        for field in &mut self.fields {
            if field.coordinate == *coordinate {
                field.status += 1;
                exists = true;
                break;
            }
        }

        if !exists {
            self.fields.push(VentureField::new(coordinate));
        }
    }

    #[allow(dead_code)]
    fn status(self: &Self, coordinate: &Coordinate) -> u8 {
        for field in &self.fields {
            if field.coordinate == *coordinate {
                return field.status;
            }
        }
        return 0;
    }

    #[inline(always)]
    fn iter(self: &Self) -> Iter<VentureField> {
        self.fields.iter()
    }
}

fn main() {
    let input = get_input(INPUT_FILE);
    let mut handler = VentureHandler::new();

    calc_ventures(&mut handler, &input, false);
    let count = count_venture_points(&handler, 2);

    println!("Minimum {} overlapping venture points: {}", 2, count);

    let mut handler = VentureHandler::new();

    calc_ventures(&mut handler, &input, true);
    let count = count_venture_points(&handler, 2);

    println!(
        "Minimum {} overlapping venture points with diagonals: {}",
        2, count
    );
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

fn calc_ventures(handler: &mut VentureHandler, input: &str, diagonal: bool) {
    // let mut handler = VentureHandler::new();

    let lines = input.lines();
    lines.for_each(|line| {
        let coords: Vec<&str> = line.trim().split(" -> ").collect();
        let venture = Venture::new(
            &Coordinate::from(coords[0]),
            &Coordinate::from(coords[1]),
            diagonal,
        );

        match venture.direction {
            Direction::Horizontal => {
                let (start, end) = if venture.start.x <= venture.end.x {
                    (venture.start.x, venture.end.x)
                } else {
                    (venture.end.x, venture.start.x)
                };

                for x in start..=end {
                    handler.push(&Coordinate {
                        x,
                        y: venture.start.y,
                    });
                }
            }
            Direction::Vertical => {
                let (start, end) = if venture.start.y <= venture.end.y {
                    (venture.start.y, venture.end.y)
                } else {
                    (venture.end.y, venture.start.y)
                };

                for y in start..=end {
                    handler.push(&Coordinate {
                        x: venture.start.x,
                        y,
                    });
                }
            }
            Direction::Diagonal => {
                // start with smallest x-coordinate
                let (start, end) = if venture.start.x <= venture.end.x {
                    (venture.start, venture.end)
                } else {
                    (venture.end, venture.start)
                };

                let step = if start.y <= end.y { 1i8 } else { -1i8 };

                let mut y = start.y;
                for x in start.x..=end.x {
                    handler.push(&Coordinate { x, y });
                    y = y.wrapping_add(step as u16);
                }
            }
            Direction::Ignore => {}
        }
    });
}

#[inline(always)]
fn count_venture_points(handler: &VentureHandler, min_power: u8) -> usize {
    handler.iter().filter(|f| f.status >= min_power).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_calc_ventures() {
        let mut handler = VentureHandler::new();
        calc_ventures(&mut handler, TEST_INPUT, false);
        let status = handler.status(&Coordinate { x: 1, y: 4 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 2, y: 4 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 3, y: 4 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 4, y: 4 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 8, y: 0 });
        assert_eq!(status, 0);
        let status = handler.status(&Coordinate { x: 7, y: 4 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 0, y: 9 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 7, y: 9 });
        assert_eq!(status, 0);
    }

    #[test]
    fn test_count_venture_points() {
        let mut handler = VentureHandler::new();
        calc_ventures(&mut handler, TEST_INPUT, false);
        let count = count_venture_points(&handler, 2);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_calc_ventures_diagonal() {
        let mut handler = VentureHandler::new();
        calc_ventures(&mut handler, TEST_INPUT, true);
        let status = handler.status(&Coordinate { x: 1, y: 4 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 2, y: 4 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 3, y: 4 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 4, y: 4 });
        assert_eq!(status, 3);
        let status = handler.status(&Coordinate { x: 8, y: 0 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 7, y: 4 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 0, y: 9 });
        assert_eq!(status, 2);
        let status = handler.status(&Coordinate { x: 7, y: 9 });
        assert_eq!(status, 0);
        let status = handler.status(&Coordinate { x: 0, y: 8 });
        assert_eq!(status, 1);
        let status = handler.status(&Coordinate { x: 1, y: 8 });
        assert_eq!(status, 0);
        let status = handler.status(&Coordinate { x: 8, y: 8 });
        assert_eq!(status, 1);
    }

    #[test]
    fn test_count_venture_points_diagonal() {
        let mut handler = VentureHandler::new();
        calc_ventures(&mut handler, TEST_INPUT, true);
        let count = count_venture_points(&handler, 2);
        assert_eq!(count, 12);
    }
}
