use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}
fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn part2(input: &str) -> i32 {
    let (_rest, instr) =
        many1(many_till(anychar, instruction).map(|(_discard, instr)| instr))(input).unwrap();

    let mut res = 0;
    let mut add = true;
    for ins in instr {
        res += match ins {
            Instruction::Mul(a, b) => {
                if add {
                    a * b
                } else {
                    0
                }
            }
            Instruction::Dont => {
                add = false;
                0
            }
            Instruction::Do => {
                add = true;
                0
            }
        }
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let dummy = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let ret = part2(dummy);

        println!("{:?}", ret);

        assert_eq!(ret, 48);
    }
}
