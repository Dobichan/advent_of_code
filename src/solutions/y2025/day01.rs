use std::{fmt::Display, str::FromStr};

use crate::{AoCSolution, parsing::input_lines};

#[derive(Debug)]
pub enum Direction {
    Left(i32),
    Right(i32),
}

#[derive(Debug, Default)]
pub struct Solution {}

fn rotate(pos: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Left(steps) => {
            let start = if pos == 0 { 100 } else { pos };
            let new_pos = start - steps;
            let hits = if new_pos <= 0 {
                1 + (-new_pos) / 100
            } else {
                0
            };
            (new_pos.rem_euclid(100), hits)
        }
        Direction::Right(steps) => {
            let new_pos = pos + steps;
            (new_pos.rem_euclid(100), new_pos.div_euclid(100))
        }
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<Direction>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|rotation| {
                let steps: i32 =
                    FromStr::from_str(&rotation[1..]).expect("Could not parse number {rotation}");
                match rotation.chars().next().expect("Line too short") {
                    'L' => Direction::Left(steps),
                    'R' => Direction::Right(steps),
                    _ => panic!("Illegal input: {rotation}"),
                }
            })
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut pos = 50;
        let mut at_zero = 0;
        for movement in data {
            let (new_pos, _) = rotate(pos, movement);
            if new_pos == 0 {
                at_zero += 1;
            }
            pos = new_pos;
        }
        at_zero
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut pos = 50;
        let mut passed_zero = 0;
        for movement in data {
            let (new_pos, hits) = rotate(pos, movement);
            passed_zero += hits;
            pos = new_pos
        }
        passed_zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "6");
    }
}
