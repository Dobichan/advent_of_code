use multimap::MultiMap;
use std::{collections::HashSet, fmt::Display};

use crate::{AoCSolution, grid::Grid, point::Point};

#[derive(Debug, Default)]
pub struct Solution {}

pub struct ParseData {
    antennas: MultiMap<char, Point>,
    grid: Grid,
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let grid: Grid = input.parse().expect("Invalid grid");
        let antennas = grid
            .iter()
            .filter(|&(_, ch)| ch != '.')
            .map(|(p, ch)| (ch, p))
            .collect();
        ParseData { antennas, grid }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut nodes = HashSet::new();

        for (_, points) in data.antennas.iter_all() {
            for (i, &a) in points.iter().enumerate() {
                for &b in &points[i + 1..] {
                    for r in [a * 2 - b, b * 2 - a] {
                        if data.grid.in_bounds(r) {
                            nodes.insert(r);
                        }
                    }
                }
            }
        }

        nodes.len()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut nodes = HashSet::new();
        for (_, points) in data.antennas.iter_all() {
            for (i, &a) in points.iter().enumerate() {
                for &b in &points[i + 1..] {
                    let delta = b - a;
                    for (mut p, step) in [(a, -delta), (b, delta)] {
                        while data.grid.in_bounds(p) {
                            nodes.insert(p);
                            p += step;
                        }
                    }
                }
            }
        }

        nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "14");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "34");
    }
}
