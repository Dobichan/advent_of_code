use advent_of_code::parsing::*;
use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{i64, multispace0, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

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
            Operator::Concat => (a.to_string() + b.to_string().as_str()).parse().unwrap(),
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

fn part1(input: &str) -> i64 {
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
    answer
}

fn part2(input: &str) -> i64 {
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
    answer
}

fn main() {
    const YEAR: u16 = 2024;
    const DAY: u8 = 7;

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

        assert_eq!(part1(EXAMPLE_INPUT.trim()), 3749)
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

        assert_eq!(part2(EXAMPLE_INPUT.trim()), 11387)
    }
}
