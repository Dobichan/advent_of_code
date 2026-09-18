use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::{IResult, Parser};
use std::fmt::Display;

use crate::AoCSolution;
use crate::parsing::input_lines;

#[derive(Copy, Clone, Debug, Default)]
pub struct Solution {}

fn get_dimensions(input: &str) -> IResult<&str, (u32, u32, u32)> {
    (u32, tag("x"), u32, tag("x"), u32)
        .map(|(l, _, w, _, h)| (l, w, h))
        .parse(input)
}

impl AoCSolution for Solution {
    type Parsed = Vec<(u32, u32, u32)>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|line| get_dimensions(line).expect("bad dimension line").1)
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|(l, w, h)| {
                let sides = [l * w, w * h, h * l];
                let smallest = sides.iter().min().unwrap();
                2 * sides.iter().sum::<u32>() + smallest
            })
            .sum::<u32>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|(l, w, h)| {
                let max = l.max(w).max(h);
                let sum_two_smallest = l + w + h - max;
                let extra = l * w * h;
                2 * sum_two_smallest + extra
            })
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"2x3x4";
    const INPUT2: &str = r"1x1x10";

    #[test]
    fn test_part1() {
        let sol = Solution::default();
        assert_eq!(sol.solve1(INPUT1), "58");

        assert_eq!(sol.solve1(INPUT2), "43");
    }

    #[test]
    fn test_part2() {
        let sol = Solution::default();
        assert_eq!(sol.solve2(INPUT1), "34");

        assert_eq!(sol.solve2(INPUT2), "14");
    }
}
