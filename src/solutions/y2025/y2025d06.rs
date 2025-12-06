use nom::number;

use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 6;
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
impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut ret: u64 = 0;
        let mut problems = Vec::with_capacity(1000);
        let mut digit_rows = Vec::with_capacity(10);

        let inputs: Vec<_> = input.trim().lines().collect();

        for i in 0..inputs.len() - 1 {
            let digits: Vec<_> = inputs[i]
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(|d| d.parse::<u64>().unwrap())
                .collect();
            digit_rows.push(digits);
        }

        let operators: Vec<_> = inputs[inputs.len() - 1]
            .split(' ')
            .filter(|o| !o.is_empty())
            .map(|o| match o {
                "+" => MathOperation::Add,
                "*" => MathOperation::Mul,
                _ => panic!("illegal operator"),
            })
            .collect();

        for i in 0..operators.len() {
            problems.push(MathProblem {
                numbers: Vec::new(),
                operation: operators[i],
            });
            for j in 0..digit_rows.len() {
                problems[i].numbers.push(digit_rows[j][i]);
            }
        }

        for prob in problems {
            ret += prob.calculate();
        }
        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut ret: u64 = 0;

        let inputs: Vec<_> = input.trim().lines().collect();
        let mut problems = Vec::with_capacity(1000);

        let line_lenght = inputs[0].len();
        let digit_rows = inputs.len() - 1;

        // Convert each line to a Vec<char> for indexing
        let char_rows: Vec<Vec<char>> = inputs[..digit_rows]
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let operators: Vec<_> = inputs[inputs.len() - 1]
            .split(' ')
            .filter(|o| !o.is_empty())
            .map(|o| match o {
                "+" => MathOperation::Add,
                "*" => MathOperation::Mul,
                _ => panic!("illegal operator"),
            })
            .collect();

        let mut problem = MathProblem::new();
        let mut next_operator = operators.iter();
        for i in 0..line_lenght {
            let mut digits = String::new();

            for j in 0..digit_rows {
                digits.push(char_rows[j][i]);
            }
            if digits.trim().len() != 0 {
                problem.numbers.push(digits.trim().parse::<u64>().unwrap());
            } else {
                problem.operation = *next_operator.next().unwrap();
                problems.push(problem);
                problem = MathProblem::new();
            }
        }
        if problem.numbers.len() > 0 {
            problem.operation = *next_operator.next().unwrap();
            problems.push(problem);
        }

        for prob in problems {
            ret += prob.calculate();
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "123 328  51 64 \n\
                 45 64  387 23 \n\
                  6 98  215 314\n\
                *   +   *   +  ";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "4277556");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str =
            "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "3263827");
    }
}
