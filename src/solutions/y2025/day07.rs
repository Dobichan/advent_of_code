use std::fmt::Display;

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct Manifold {
    pub start: usize,
    pub width: usize,
    pub rows: Vec<Vec<bool>>,
}

impl AoCSolution for Solution {
    type Parsed = Manifold;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut lines = input.trim().lines();
        let first = lines.next().expect("Empty input");
        let start = first.find('S').expect("No S on first line");
        let width = first.len();

        let rows = lines
            .filter(|l| l.contains('^'))
            .map(|line| line.chars().map(|ch| ch == '^').collect())
            .collect();

        Manifold { start, width, rows }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut beams = vec![false; data.width];
        beams[data.start] = true;
        let mut splits = 0;

        for row in &data.rows {
            let mut this_row_beams = vec![false; data.width];

            for col in (0..data.width).filter(|&c| beams[c]) {
                if row[col] {
                    splits += 1;
                    if col > 0 {
                        this_row_beams[col - 1] = true;
                    }
                    if col + 1 < data.width {
                        this_row_beams[col + 1] = true;
                    }
                } else {
                    this_row_beams[col] = true;
                }
            }
            beams = this_row_beams;
        }
        splits
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut counts = vec![0u64; data.width];
        counts[data.start] = 1;

        for row in &data.rows {
            let mut this_row_beams = vec![0u64; data.width];

            for (col, &n) in counts.iter().enumerate().filter(|&(_, &n)| n > 0) {
                if row[col] {
                    if col > 0 {
                        this_row_beams[col - 1] += n;
                    }
                    if col + 1 < data.width {
                        this_row_beams[col + 1] += n;
                    }
                } else {
                    this_row_beams[col] += n;
                }
            }
            counts = this_row_beams;
        }
        counts.iter().sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "40");
    }
}
