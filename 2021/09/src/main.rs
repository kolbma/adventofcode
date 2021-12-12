#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{collections::HashMap, fs::File, io::Read, sync::atomic, sync::atomic::AtomicUsize};

const INPUT_FILE: &str = "./data/input";

type PointsLow = Vec<Point>;
type MapNiner = HashMap<usize, HashMap<usize, Point>>;

thread_local! {
    static LINE_LENGTH: AtomicUsize = AtomicUsize::new(0);
    static LINE_COUNT: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
    pub value: u8,
}

impl Point {
    fn new(x: usize, y: usize, value: u8) -> Self {
        Self { x, y, value }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        // ignore value
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let input = get_input(INPUT_FILE);

    let points = point_locations(&input);
    let map_yx_niners = points.1;
    let points = points.0;

    let risk_sum = calc_risk_sum(&points);
    println!("Risk sum: {}", risk_sum);

    let mut bsizes = basin_sizes(&points, &map_yx_niners);
    bsizes.sort_unstable();
    bsizes.reverse();

    let basin_res = bsizes[0..3].iter().product::<usize>();
    println!("Three largest basins size calculation: {}", basin_res);
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

fn point_locations<'a>(input: &'a str) -> (PointsLow, MapNiner) {
    let mut points = PointsLow::new();
    let mut map_yx_niners = MapNiner::new();
    let buf = &mut [0u8, 9, 9];
    let mut line_ref: [Vec<u8>; 2] = Default::default();
    let mut x = 0usize;
    let mut y = 0usize;

    fn cleanup_line(points: &mut PointsLow, line_ref: &[Vec<u8>; 2], y: usize) {
        let del_points = line_ref[1]
            .iter()
            .enumerate()
            .filter_map(|(x, val)| {
                if *val < line_ref[0][x] {
                    Some((x, y - 1))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();

        let mut del_points_idx = points
            .iter()
            .enumerate()
            .filter_map(|(del_pos_idx, p)| {
                if del_points.contains(&(p.x, p.y)) {
                    Some(del_pos_idx)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        del_points_idx.reverse();

        del_points_idx.iter().for_each(|idx| {
            points.remove(*idx);
        });
    }

    fn fill_map_niners(map: &mut MapNiner, key: usize, child_key: usize, point: &Point) {
        if let Some(child_map) = map.get_mut(&key) {
            child_map.insert(child_key, *point);
        } else {
            let mut child_map = HashMap::new();
            child_map.insert(child_key, *point);
            map.insert(key, child_map);
        }
    }

    input.bytes().for_each(|b| {
        if b >= 48 && b <= 57 {
            let value = b - 48;

            line_ref[1].push(value);
            buf[2] = value;
            if buf[0] > buf[1]
                && buf[1] < buf[2]
                && (line_ref[0].len() < x - 1 || line_ref[0][x - 1] > buf[1])
            {
                points.push(Point::new(x - 1, y, buf[1]));
            }
            if buf[2] == 9 {
                let point = Point::new(x, y, buf[2]);
                fill_map_niners(&mut map_yx_niners, y, x, &point);
            }
            buf[0] = buf[1];
            buf[1] = buf[2];

            x += 1;
        } else if b == 10 {
            LINE_LENGTH.with(|line_length| line_length.store(x, atomic::Ordering::SeqCst));
            if buf[0] > buf[1] && (line_ref[0].len() < x - 1 || line_ref[0][x - 1] > buf[1]) {
                points.push(Point::new(x - 1, y, buf[1]));
            }

            if y > 0 {
                cleanup_line(&mut points, &line_ref, y);
            }

            x = 0;
            y += 1;
            LINE_COUNT.with(|line_count| line_count.fetch_add(1, atomic::Ordering::SeqCst));
            buf[0] = 0;
            buf[1] = 9;
            line_ref = [line_ref[1].clone(), Vec::new()];
        }
    });

    cleanup_line(&mut points, &line_ref, y);
    if x > 0 {
        LINE_COUNT.with(|line_count| line_count.fetch_add(1, atomic::Ordering::SeqCst));
    }

    (points, map_yx_niners)
}

#[inline(always)]
fn calc_risk_sum(points: &PointsLow) -> u16 {
    points.iter().map(|p| p.value as u16 + 1).sum::<u16>()
}

fn basin_sizes(points: &PointsLow, map_yx_niners: &MapNiner) -> Vec<usize> {
    let mut basin_size = Vec::<usize>::new();

    fn walk_neighbours(
        basin_points: &mut Vec<Point>,
        x: usize,
        y: usize,
        map_yx_niners: &MapNiner,
    ) {
        let line_count = LINE_COUNT.with(|line_count| line_count.load(atomic::Ordering::SeqCst));
        let line_length =
            LINE_LENGTH.with(|line_length| line_length.load(atomic::Ordering::SeqCst));

        let is_niner = if let Some(niners_in_line) = map_yx_niners.get(&y) {
            if niners_in_line.get(&x).is_some() {
                // found niner
                true
            } else {
                false
            }
        } else {
            false
        };

        if !is_niner {
            let bp = Point::new(x, y, 0);

            if !basin_points.contains(&bp) {
                // check neighbours
                let mut neighbours = Vec::<Point>::new();
                if x > 0 {
                    neighbours.push(Point::new(x - 1, y, 0));
                }
                if x < line_length - 1 {
                    neighbours.push(Point::new(x + 1, y, 0));
                }
                if y > 0 {
                    neighbours.push(Point::new(x, y - 1, 0));
                }
                if y < line_count - 1 {
                    neighbours.push(Point::new(x, y + 1, 0));
                }

                let neighbours = neighbours
                    .iter()
                    .filter(|&neighbour| !basin_points.contains(neighbour))
                    .collect::<Vec<&Point>>();

                basin_points.push(bp); // push after neighbours check

                neighbours.iter().for_each(|p| {
                    walk_neighbours(basin_points, p.x, p.y, map_yx_niners);
                });
            }
        }
    }

    points.iter().for_each(|p| {
        let mut basin_points = Vec::<Point>::new();
        walk_neighbours(&mut basin_points, p.x, p.y, map_yx_niners);
        basin_size.push(basin_points.len());
    });

    basin_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"2199943210
                               3987894921
                               9856789892
                               8767896789
                               9899965678";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_low_locations() {
        let input = TEST_INPUT;

        let points = point_locations(input);
        let points = points.0;

        // println!("{:#?}", points);

        assert_eq!(points[0], Point::new(1, 0, 1));
        assert_eq!(points[1], Point::new(9, 0, 0));
        assert_eq!(points[2], Point::new(2, 2, 5));
        assert_eq!(points[3], Point::new(6, 4, 5));
    }

    #[test]
    fn test_niners() {
        let input = TEST_INPUT;

        let points = point_locations(input);
        let map_yx_niners = points.1;
        // let _points = points.0;

        assert_eq!(map_yx_niners.len(), 5);
        assert_eq!(map_yx_niners.get(&0).unwrap().len(), 3);
        assert_eq!(map_yx_niners.get(&1).unwrap().len(), 3);
        assert_eq!(map_yx_niners.get(&2).unwrap().len(), 3);
        assert_eq!(map_yx_niners.get(&3).unwrap().len(), 2);
        assert_eq!(map_yx_niners.get(&4).unwrap().len(), 4);
    }

    #[test]
    fn test_niners_input() {
        let input = get_input(INPUT_FILE);

        let points = point_locations(&input);
        let map_yx_niners = points.1;

        assert_eq!(map_yx_niners.len(), 100);
        assert_eq!(map_yx_niners.get(&0).unwrap().len(), 24);
        assert_eq!(map_yx_niners.get(&10).unwrap().len(), 27);

        assert_eq!(
            map_yx_niners
                .iter()
                .map(|(_k, v)| { v.len() })
                .sum::<usize>(),
            2815
        );
    }

    #[test]
    fn test_basin_sizes() {
        let input = TEST_INPUT;

        let points = point_locations(input);
        let map_yx_niners = points.1;
        let points = points.0;

        let mut bsizes = basin_sizes(&points, &map_yx_niners);
        bsizes.sort_unstable();
        bsizes.reverse();

        let basin_res = bsizes[0..3].iter().product::<usize>();

        assert_eq!(basin_res, 1134)
    }

    #[test]
    fn test_basin_sizes_input() {
        let input = get_input(INPUT_FILE);

        let points = point_locations(&input);
        let map_yx_niners = points.1;
        let points = points.0;

        let mut bsizes = basin_sizes(&points, &map_yx_niners);
        bsizes.sort_unstable();
        bsizes.reverse();

        let basin_res = bsizes[0..3].iter().product::<usize>();

        assert_eq!(basin_res, 931200);
    }
}
