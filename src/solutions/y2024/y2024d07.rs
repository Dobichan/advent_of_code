use crate::AoCSolution;
use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{i64, multispace0, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

const YEAR: u16 = 2024;
const DAY: u8 = 7;

#[derive(Debug)]
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

#[derive(Debug)]
struct Equation {
    expected_sum: i64,
    numbers: Vec<i64>,
    operators: Vec<Operator>,
}

impl Equation {
    fn new(expected_sum: i64, numbers: Vec<i64>) -> Self {
        let numbers_length = numbers.len();
        let mut ret = Equation {
            expected_sum,
            numbers,
            operators: Vec::with_capacity(numbers_length),
        };

        for _i in 0..numbers_length - 1 {
            ret.operators.push(Operator::Add);
        }

        ret
    }

    fn permute_operations(&mut self, use_concat: bool) -> bool {
        for operator in self.operators.iter_mut() {
            match operator {
                Operator::Add => {
                    *operator = Operator::Mul;
                    return true;
                }
                Operator::Mul => {
                    if use_concat {
                        *operator = Operator::Concat;
                        return true;
                    } else {
                        *operator = Operator::Add;
                    }
                }
                Operator::Concat => *operator = Operator::Add,
            }
        }
        false
    }

    fn calculate(&self) -> i64 {
        let mut ret = self.numbers[0];
        for (i, num) in self.numbers.iter().enumerate() {
            if i > 0 {
                ret = self.operators[i - 1].calculate(ret, *num);
            }
        }
        ret
    }

    fn is_valid(&self) -> bool {
        self.calculate() == self.expected_sum
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

pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let mut answer = 0;
        for line in input.lines() {
            let mut equation = parse_line(line.trim());
            while !equation.is_valid() {
                if !equation.permute_operations(false) {
                    break;
                }
            }

            if equation.is_valid() {
                answer += equation.expected_sum;
            }
        }
        answer.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut answer = 0;
        for line in input.lines() {
            let mut equation = parse_line(line.trim());
            while !equation.is_valid() {
                if !equation.permute_operations(true) {
                    break;
                }
            }

            if equation.is_valid() {
                answer += equation.expected_sum;
            }
        }
        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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

        let mut sol = Solution {};

        assert_eq!(sol.part1(EXAMPLE_INPUT.trim()), "3749")
    }

    #[test]
    fn test_part2() {
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

        let mut sol = Solution {};

        assert_eq!(sol.part2(EXAMPLE_INPUT.trim()), "11387")
    }
}
