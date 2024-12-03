use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

extern crate nom;

#[derive(Debug)]
pub enum Instruction {
    Mul(i32, i32),
}
fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn part1(input: &str) -> i32 {
    let (_rest, instr) =
        many1(many_till(anychar, instruction).map(|(_discard, instr)| instr))(input).unwrap();
    let mut res = 0;
    for ins in instr {
        res += match ins {
            Instruction::Mul(a, b) => a * b,
        }
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let ret = part1(dummy);

        println!("{:?}", ret);

        assert_eq!(ret, 161);
    }
}
