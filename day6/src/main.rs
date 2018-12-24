use failure::Fallible;
use itertools::iproduct;
use std::collections::{BinaryHeap, HashSet};
use std::io::{self, BufRead, BufReader};
use std::ops::Range;

type Coords<'a> = &'a [(i16, i16)];
type CoordsBuf = Vec<(i16, i16)>;
type Bounds = (Range<i16>, Range<i16>);

fn read_coords<R: BufRead>(reader: R) -> Fallible<CoordsBuf> {
    reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut iter = line.split(", ");
            Ok((iter.next().unwrap().parse()?, iter.next().unwrap().parse()?))
        })
        .collect()
}

fn bounds(coords: Coords) -> Bounds {
    let x_min = coords.iter().map(|c| c.0).min().unwrap();
    let x_max = coords.iter().map(|c| c.0).max().unwrap();
    let y_min = coords.iter().map(|c| c.1).min().unwrap();
    let y_max = coords.iter().map(|c| c.1).max().unwrap();
    ((x_min..x_max + 1), (y_min..y_max + 1))
}

fn manhattan_distance(x1: i16, y1: i16, x2: i16, y2: i16) -> i16 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn nearest(coords: Coords, x2: i16, y2: i16) -> Option<usize> {
    let mut v = coords
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| (-manhattan_distance(*x1, *y1, x2, y2), i))
        .collect::<BinaryHeap<_>>();
    let (smallest, n) = v.pop().unwrap();
    let (next_smallest, _) = v.pop().unwrap();
    if smallest != next_smallest {
        Some(n)
    } else {
        None
    }
}

fn infinite(coords: Coords) -> HashSet<usize> {
    let (
        Range {
            start: x_start,
            end: x_end,
        },
        Range {
            start: y_start,
            end: y_end,
        },
    ) = bounds(coords);

    let top = (x_start - 1..x_end).map(|x| (x, y_start - 1));
    let right = (y_start - 1..y_end).map(|y| (x_end + 1, y));
    let bottom = (x_start..x_end + 1).map(|x| (x, y_end + 1));
    let left = (y_start..y_end + 1).map(|y| (x_start - 1, y));

    top.chain(right)
        .chain(bottom)
        .chain(left)
        .filter_map(|(x, y)| nearest(coords, x, y))
        .collect()
}

fn largest_area(coords: Coords) -> usize {
    let mut counts = vec![0; coords.len()];
    let (x_bounds, y_bounds) = bounds(coords);
    for (x, y) in iproduct!(x_bounds, y_bounds) {
        if let Some(n) = nearest(coords, x, y) {
            counts[n] += 1;
        }
    }

    let infinite = infinite(coords);
    counts
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| if infinite.contains(&i) { None } else { Some(n) })
        .max()
        .unwrap()
}

fn manhattan_max_area(coords: Coords, max: i16) -> usize {
    let (x_bounds, y_bounds) = bounds(coords);

    iproduct!(x_bounds, y_bounds)
        .filter(|(x1, y1)| {
            let sum: i16 = coords
                .iter()
                .map(|(x2, y2)| manhattan_distance(*x1, *y1, *x2, *y2))
                .sum();
            sum < max
        })
        .count()
}

fn main() -> Fallible<()> {
    let coords = read_coords(BufReader::new(io::stdin()))?;

    // Part 1
    println!("Part 1: {}", largest_area(&coords));

    // Part 2
    println!("Part 2: {}", manhattan_max_area(&coords, 10000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_COORDS: CoordsBuf = read_coords(BufReader::new(
            &b"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"[..]
        ))
        .unwrap();
    }

    #[test]
    fn test_read_coords() {
        assert_eq!(
            *TEST_COORDS,
            vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]
        );
    }

    #[test]
    fn test_bounds() {
        assert_eq!(bounds(&TEST_COORDS), ((1..9), (1..10)));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(5, 5, 5, 5), 0);
        assert_eq!(manhattan_distance(1, 1, 1, 6), 5);
        assert_eq!(manhattan_distance(1, 1, 5, 5), 8);
        assert_eq!(manhattan_distance(5, 5, 1, 1), 8);
        assert_eq!(manhattan_distance(5, 2, 5, 5), 3);
        assert_eq!(manhattan_distance(5, 2, 3, 4), 4);
    }

    #[test]
    fn test_nearest() {
        assert_eq!(nearest(&TEST_COORDS, 5, 5), Some(4));
        assert_eq!(nearest(&TEST_COORDS, 5, 2), Some(4));
        assert_eq!(nearest(&TEST_COORDS, 2, 5), None);
    }

    #[test]
    fn test_infinite() {
        assert_eq!(
            infinite(&TEST_COORDS),
            vec![0, 1, 2, 5].into_iter().collect()
        );
    }

    #[test]
    fn test_largest_area() {
        assert_eq!(largest_area(&TEST_COORDS), 17);
    }

    #[test]
    fn test_manhattan_max_area() {
        assert_eq!(manhattan_max_area(&TEST_COORDS, 32), 16);
    }
}
