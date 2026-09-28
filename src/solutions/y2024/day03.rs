use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::combinator::value;
use nom::{IResult, Parser};
use std::fmt::Display;

use crate::AoCSolution;

#[derive(Debug, Clone)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    (tag("mul("), u32, tag(","), u32, tag(")"))
        .map(|(_, x, _, y, _)| Instruction::Mul(x, y))
        .parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
        mul_instruction,
    ))
    .parse(input)
}

#[derive(Debug, Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = Vec<Instruction>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut instructions = Vec::new();
        let mut rest = input;

        while let Some(pos) = rest.find(['m', 'd']) {
            rest = &rest[pos..];

            match instruction(rest) {
                Ok((remaining, instruction)) => {
                    instructions.push(instruction);
                    rest = remaining
                }
                Err(_) => rest = &rest[1..],
            }
        }

        instructions
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .filter_map(|instruction| match instruction {
                Instruction::Mul(a, b) => Some(a * b),
                _ => None,
            })
            .sum::<u32>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .fold(
                (true, 0),
                |(process, total), instruction| match instruction {
                    Instruction::Do => (true, total),
                    Instruction::Dont => (false, total),
                    Instruction::Mul(a, b) if process => (process, total + a * b),
                    Instruction::Mul(_, _) => (process, total),
                },
            )
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "161");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "48");
    }
}
