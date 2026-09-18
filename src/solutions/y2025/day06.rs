use std::fmt::Display;

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug, Copy, Clone)]
enum MathOperation {
    Add,
    Mul,
}

impl MathOperation {
    fn calc(&self, num1: u64, num2: u64) -> u64 {
        match self {
            MathOperation::Add => num1 + num2,
            MathOperation::Mul => num1 * num2,
        }
    }
}

#[derive(Debug)]
pub struct ParseData {
    // Part 1 view
    numbers_row: Vec<Vec<u64>>,

    // Part 2 view
    bytes_row: Vec<Vec<u8>>,

    // How to handle the numbers
    operators: Vec<MathOperation>,
}

#[derive(Debug)]
pub struct MathProblem {
    numbers: Vec<u64>,
    operation: MathOperation,
}

impl MathProblem {
    fn new() -> Self {
        Self {
            numbers: Vec::with_capacity(10),
            operation: MathOperation::Add,
        }
    }
    fn calculate(&self) -> u64 {
        let mut ret = self.numbers[0];
        for i in 1..self.numbers.len() {
            ret = self.operation.calc(ret, self.numbers[i]);
        }
        ret
    }
}

fn column_number(rows: &[Vec<u8>], col: usize) -> Option<u64> {
    let mut digits = rows
        .iter()
        .map(|row| row[col])
        .filter(u8::is_ascii_digit)
        .map(|d| u64::from(d - b'0'));
    let first = digits.next()?;
    Some(digits.fold(first, |acc, d| acc * 10 + d))
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut lines = input.trim_end().lines();
        let operator_line = lines.next_back().expect("Input is empty");

        let operators = operator_line
            .split_ascii_whitespace()
            .map(|op| match op {
                "+" => MathOperation::Add,
                "*" => MathOperation::Mul,
                other => panic!("Illegal operator {other:?}"),
            })
            .collect();

        let mut numbers_row: Vec<Vec<u64>> = Vec::new();
        let mut bytes_row: Vec<Vec<u8>> = Vec::new();

        for line in lines {
            numbers_row.push(
                line.split_ascii_whitespace()
                    .map(|n| n.parse().expect("Not a number"))
                    .collect(),
            );
            bytes_row.push(line.as_bytes().to_vec());
        }
        dbg!(ParseData {
            numbers_row,
            bytes_row,
            operators,
        })
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut ret: u64 = 0;
        let mut problems = Vec::with_capacity(data.operators.len());

        for (i, &operation) in data.operators.iter().enumerate() {
            problems.push(MathProblem {
                numbers: data.numbers_row.iter().map(|row| row[i]).collect(),
                operation,
            });
        }

        for prob in problems {
            ret += prob.calculate();
        }
        ret
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut ret: u64 = 0;

        // let inputs: Vec<_> = data.trim().lines().collect();
        let mut problems = Vec::with_capacity(data.operators.len());
        let width = data.bytes_row.first().map_or(0, Vec::len);

        let mut problem = MathProblem::new();
        let mut next_operator = data.operators.iter();

        for col in 0..width {
            if let Some(val) = column_number(&data.bytes_row, col) {
                problem.numbers.push(val);
            } else {
                problem.operation = *next_operator.next().expect("No more operators left");
                problems.push(problem);
                problem = MathProblem::new();
            }
        }
        if !problem.numbers.is_empty() {
            problem.operation = *next_operator.next().unwrap();
            problems.push(problem);
        }

        for prob in problems {
            ret += prob.calculate();
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "3263827");
    }
}
