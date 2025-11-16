use crate::AoCSolution;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar};
use nom::combinator::value;
use nom::multi::{many_till, many1};
use nom::sequence::{delimited, separated_pair};

const YEAR: u16 = 2024;
const DAY: u8 = 3;

#[derive(Debug, Clone)]
enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, numbers) = delimited(
        tag("("),
        separated_pair(complete::i64, tag(","), complete::i64),
        tag(")"),
    )
    .parse(input)?;

    Ok((input, Instruction::Mul(numbers.0, numbers.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
        mul_instruction,
    ))
    .parse(input)
}

pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let (_, instructions) =
            many1(many_till(anychar, mul_instruction).map(|(_skip, instruction)| instruction))
                .parse(input)
                .unwrap();

        instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (_, instructions) =
            many1(many_till(anychar, instruction).map(|(_skip, instruction)| instruction))
                .parse(input)
                .unwrap();

        instructions
            .iter()
            .fold(
                (true, 0),
                |(process, total), instruction| match instruction {
                    Instruction::Do => (true, total),
                    Instruction::Dont => (false, total),
                    Instruction::Mul(a, b) => {
                        if process {
                            (process, total + a * b)
                        } else {
                            (process, total)
                        }
                    }
                },
            )
            .1
            .to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "#;

        let sol = Solution {};
        let answer = sol.part1(EXAMPLE_INPUT.trim());
        assert_eq!(answer, "161");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "#;

        let sol = Solution {};
        let answer = sol.part2(EXAMPLE_INPUT.trim());
        assert_eq!(answer, "48");
    }
}
