use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};
use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{i64, multispace0, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn calculate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Concat => a * 10_i64.pow(b.ilog10() + 1) + b,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Equation {
    expected_sum: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn new(expected_sum: i64, numbers: Vec<i64>) -> Self {
        Equation {
            expected_sum,
            numbers,
        }
    }

    fn is_solvable(&self, use_concat: bool) -> bool {
        self.search(self.numbers[0], &self.numbers[1..], use_concat)
    }

    fn search(&self, acc: i64, rest: &[i64], use_concat: bool) -> bool {
        let Some((&next, rest)) = rest.split_first() else {
            return acc == self.expected_sum;
        };
        if acc > self.expected_sum {
            return false;
        }
        self.search(Operator::Add.calculate(acc, next), rest, use_concat)
            || self.search(Operator::Mul.calculate(acc, next), rest, use_concat)
            || (use_concat && self.search(Operator::Concat.calculate(acc, next), rest, use_concat))
    }
}

fn get_numbers(nums: &str) -> IResult<&str, Vec<i64>> {
    terminated(separated_list1(multispace1, i64), multispace0).parse(nums)
}

fn parse_line(line: &str) -> Equation {
    let (_, (sum, numbers)) = separated_pair(i64, tag(": "), get_numbers)
        .parse(line)
        .unwrap();

    Equation::new(sum, numbers)
}

#[derive(Debug, Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = Vec<Equation>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input).map(|l| parse_line(l.trim())).collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .filter(|eq| eq.is_solvable(false))
            .map(|eq| eq.expected_sum)
            .sum::<i64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .filter(|eq| eq.is_solvable(true))
            .map(|eq| eq.expected_sum)
            .sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "3749")
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "11387")
    }
}
