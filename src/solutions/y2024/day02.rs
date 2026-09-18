use std::fmt::Display;

use crate::{
    AoCSolution,
    parsing::{input_lines, line_numbers_to_vec},
};

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct Report {
    numbers: Vec<i64>,
}

#[derive(Debug)]
pub struct ParseData {
    reports: Vec<Report>,
}

impl Report {
    fn diffs(&self) -> impl Iterator<Item = i64> + '_ {
        self.numbers.windows(2).map(|w| w[1] - w[0])
    }

    fn is_safe(&self) -> bool {
        self.diffs().all(|diff| (1..=3).contains(&diff))
            || self.diffs().all(|diff| (-3..=-1).contains(&diff))
    }

    fn is_safe_with_dampener(&self) -> bool {
        self.is_safe()
            || (0..self.numbers.len()).any(|number_to_skip| {
                let pruned: Vec<_> = self
                    .numbers
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != number_to_skip)
                    .map(|(_, &n)| n)
                    .collect();
                Report { numbers: pruned }.is_safe()
            })
    }
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        ParseData {
            reports: input_lines(input)
                .map(|line| Report {
                    numbers: line_numbers_to_vec(line),
                })
                .collect(),
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.reports.iter().filter(|r| r.is_safe()).count()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.reports
            .iter()
            .filter(|r| r.is_safe_with_dampener())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "4");
    }
}
