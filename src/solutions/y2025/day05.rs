use std::fmt::Display;

use nom::{IResult, Parser, bytes::complete::tag, character::complete::u64};

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct ParseData {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

fn id_range(entry: &str) -> IResult<&str, (u64, u64)> {
    (u64, tag("-"), u64).map(|(a, _, b)| (a, b)).parse(entry)
}

fn is_fresh(ingredient: u64, ranges: &[(u64, u64)]) -> bool {
    ranges
        .iter()
        .any(|range| (range.0..=range.1).contains(&ingredient))
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let (ranges, ingrs) = input.trim().split_once("\n\n").expect("Malformed input.");

        ParseData {
            ranges: ranges
                .trim()
                .lines()
                .map(|range| id_range(range).expect("Illegal range").1)
                .collect(),
            ingredients: ingrs
                .trim()
                .lines()
                .map(|i| i.parse().expect("Could not convert to digit"))
                .collect(),
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.ingredients
            .iter()
            .filter(|&&ingredient| is_fresh(ingredient, &data.ranges))
            .count()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut ranges = data.ranges.clone();
        ranges.sort_unstable();

        let mut total: u64 = 0;
        let mut covered_to: Option<u64> = None;

        for &(from, to) in &ranges {
            let start = match covered_to {
                Some(end) if end >= from => end + 1,
                _ => from,
            };

            if start <= to {
                total += to - start + 1;
                covered_to = Some(to)
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "14");
    }
}
