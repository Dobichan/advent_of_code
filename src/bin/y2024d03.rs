use advent_of_code::parsing::*;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, anychar};
use nom::combinator::value;
use nom::multi::{many_till, many1};
use nom::sequence::{delimited, separated_pair};

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

fn part1(input: &str) -> i64 {
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
        .sum()
}

fn part2(input: &str) -> i64 {
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
}

fn main() {
    const YEAR: u16 = 2024;
    const DAY: u8 = 3;

    let input = read_input(YEAR, DAY);

    let start = std::time::Instant::now();
    let answer1 = part1(&input);
    let end = std::time::Instant::now();

    println!("");
    println!("Answer part 1: {answer1}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");

    let start = std::time::Instant::now();
    let answer2 = part2(&input);
    let end = std::time::Instant::now();

    println!("Answer part 2: {answer2}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "#;

        let answer = part1(EXAMPLE_INPUT.trim());
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "#;

        let answer = part2(EXAMPLE_INPUT.trim());
        assert_eq!(answer, 48);
    }
}
