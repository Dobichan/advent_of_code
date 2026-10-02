use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn get_win_cost(&self, scale: i64) -> i64 {
        let mut cost = 0;

        let a = (self.a.0, self.a.1);
        let b = (self.b.0, self.b.1);
        let prize = (self.prize.0 + scale, self.prize.1 + scale);

        // A*a.0 + B*b.0 = prize.0
        // A*a.1 + B*b.1 = prize.1

        // Solving gives
        // A = (b.0*prize.1 - b.1*prize.0) / (a.1*b.0 - a.0*b.1)
        // B = (a.1*prize.0 - a.0*prize.1) / (a.1*b.0 - a.0*b.1)

        // If A and B is whole number (no fractions) we win
        // return A * 3 + B for the token cost

        let divisor = a.1 * b.0 - a.0 * b.1;
        if divisor != 0 {
            let a1: f64 = (b.0 * prize.1 - b.1 * prize.0) as f64;
            let b1: f64 = (a.1 * prize.0 - a.0 * prize.1) as f64;

            let a = a1 / divisor as f64;
            let b = b1 / divisor as f64;

            if a.abs().fract() < f64::EPSILON && b.abs().fract() < f64::EPSILON {
                cost = (a * 3.0 + b) as i64;
            }
        }

        cost
    }
}

impl TryFrom<&[&str]> for ClawMachine {
    type Error = String;

    fn try_from(lines: &[&str]) -> Result<Self, Self::Error> {
        let [a, b, prize] = lines else {
            return Err(format!("expected 3 lines, got {:?}", lines));
        };

        Ok(Self {
            a: parse_xy(a)?,
            b: parse_xy(b)?,
            prize: parse_xy(prize)?,
        })
    }
}

fn parse_xy(line: &str) -> Result<(i64, i64), String> {
    let (x, y) = line
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|l| !l.is_empty())
        .map(|asc_digits| asc_digits.parse::<i64>().map_err(|e| e.to_string()))
        .collect_tuple()
        .ok_or_else(|| format!("expected exactly two numbers in {line}"))?;
    Ok((x?, y?))
}

impl AoCSolution for Solution {
    type Parsed = Vec<ClawMachine>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let lines: Vec<_> = input_lines(input).filter(|l| !l.is_empty()).collect();
        lines
            .chunks(3)
            .map(|line_group| ClawMachine::try_from(line_group).unwrap())
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter().map(|m| m.get_win_cost(0)).sum::<i64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|m| m.get_win_cost(10000000000000))
            .sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "480");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "875318608908");
    }
}
