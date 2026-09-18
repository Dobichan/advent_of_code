use std::{collections::HashSet, fmt::Display};

use crate::{AoCSolution, grid::Grid, point::Point};

#[derive(Debug, Default)]
pub struct Solution {}

fn reachable_peaks(grid: &Grid<u8>, p: Point) -> Vec<Point> {
    let elevation = grid[p];
    if elevation == 9 {
        return vec![p];
    }

    let mut peaks = Vec::new();

    for delta in Point::CARDINAL {
        let next = p + delta;
        if grid.get(next) == Some(elevation + 1) {
            peaks.extend(reachable_peaks(grid, next));
        }
    }
    peaks
}

impl AoCSolution for Solution {
    type Parsed = Grid<u8>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let grid: Grid<char> = input.parse().expect("Illegal grid");
        grid.map(|ch| ch.to_digit(10).expect("Not a valid digit") as u8)
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut sum = 0;
        for (p, _) in data.iter().filter(|&(_, elevation)| elevation == 0) {
            sum += reachable_peaks(data, p)
                .into_iter()
                .collect::<HashSet<_>>()
                .len();
        }
        sum
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut sum = 0;

        for (p, _) in data.iter().filter(|&(_, elevation)| elevation == 0) {
            sum += reachable_peaks(data, p).len();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
9990999
9991999
9992999
6543456
7111117
8111118
9111119
";

    #[test]
    fn test_parse() {
        assert_eq!(
            Solution::default().parse(EXAMPLE_INPUT),
            Grid::try_from(vec![
                vec![9, 9, 9, 0, 9, 9, 9],
                vec![9, 9, 9, 1, 9, 9, 9],
                vec![9, 9, 9, 2, 9, 9, 9],
                vec![6, 5, 4, 3, 4, 5, 6],
                vec![7, 1, 1, 1, 1, 1, 7],
                vec![8, 1, 1, 1, 1, 1, 8],
                vec![9, 1, 1, 1, 1, 1, 9]
            ])
            .expect("Should be a grid")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "2");
    }
}
