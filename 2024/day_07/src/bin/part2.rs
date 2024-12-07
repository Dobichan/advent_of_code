use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha0},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Mul,
    Concatenate,
}

impl Operator {
    fn calc(&self, a: &u64, b: &u32) -> u64 {
        match self {
            Operator::Add => *a + *b as u64,
            Operator::Mul => *a * *b as u64,
            Operator::Concatenate => {
                let con = a.to_string() + b.to_string().as_str();
                con.parse::<u64>().unwrap()
            }
        }
    }
}

#[derive(Debug)]
pub struct Equation {
    sum: u64,
    numbers: Vec<u32>,
    operators: Vec<Operator>,
}

impl Equation {
    fn calculate(&self) -> u64 {
        // Seed the result
        let mut s: u64 = self.numbers[0] as u64;

        for (i, (a, b)) in self.numbers.iter().tuple_windows().enumerate() {
            s = self.operators[i].calc(&s, b);
            // println!("{s}");
        }

        s
    }

    fn is_valid(&self) -> bool {
        self.calculate() == self.sum
    }

    fn permute_operations(&mut self) {
        for i in 0..usize::pow(3, self.operators.len() as u32) {
            // println!("Permute: {:?}", self);
            if self.is_valid() {
                println!("Valid: {:?}", self);
                break;
            }
            let mut overflowing = true;
            for op in self.operators.iter_mut() {
                if overflowing {
                    *op = match op {
                        Operator::Add => {
                            overflowing = false;
                            Operator::Mul
                        }
                        Operator::Mul => {
                            overflowing = false;
                            Operator::Concatenate
                        }
                        Operator::Concatenate => {
                            overflowing = true;
                            Operator::Add
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_line(line: &str) -> IResult<&str, Equation> {
    let (rest, s) = delimited(alpha0, complete::u64, tag(": "))(line)?;
    let (_, nums) = separated_list0(tag(" "), complete::u32)(rest)?;

    let mut ops: Vec<Operator> = Vec::with_capacity(nums.len() - 1);
    for _ in 0..nums.len() - 1 {
        ops.push(Operator::Add);
    }

    Ok((
        "",
        Equation {
            sum: s,
            numbers: nums,
            operators: ops,
        },
    ))
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut ret = vec![];
    for line in input.lines() {
        ret.push(parse_line(line).unwrap().1);
    }
    ret
}

fn part2(input: &str) -> u64 {
    let mut data = parse_input(input);

    // println!("{:?}", data);

    data.iter_mut()
        .map(|e| {
            e.permute_operations();
            e
        })
        .filter(|e| e.is_valid())
        .map(|e| e.sum)
        .sum::<u64>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part2(dummy), 11387);
    }

    #[test]
    fn test_permute1() {
        let line = "190: 10 19";

        let mut equation = parse_line(line).unwrap().1;

        equation.permute_operations();

        println!("{:?}", equation);
        assert_eq!(equation.is_valid(), true);
    }

    #[test]
    fn test_permute2() {
        let line = "7290: 6 8 6 15";

        let mut equation = parse_line(line).unwrap().1;

        equation.permute_operations();

        println!("{:?}", equation);
        assert_eq!(equation.is_valid(), false);
    }

    #[test]
    fn test_permute3() {
        let line = "3267: 81 40 27";

        let mut equation = parse_line(line).unwrap().1;

        equation.permute_operations();

        println!("{:?}", equation);
        assert_eq!(equation.is_valid(), true);
    }

    #[test]
    fn test_permute4() {
        let line = "7290: 6 8 6 15";

        let mut equation = parse_line(line).unwrap().1;

        equation.permute_operations();

        println!("{:?}", equation);
        assert_eq!(equation.is_valid(), true);
    }

    #[test]
    fn test_parse_line() {
        let line = "190: 10 19";

        let equation = parse_line(line).unwrap();

        assert_eq!(equation.1.sum, 190);
        assert_eq!(equation.1.numbers, [10, 19]);
        assert_eq!(equation.1.operators, [Operator::Add]);
    }
}
