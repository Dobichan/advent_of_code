use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Debug, Default)]
pub struct Solution {}

pub fn get_max_jolt(bank: &[u8], position: usize) -> u64 {
    let mut max = 0;
    let mut max_index = 0;

    let available_chars = &bank[0..bank.len() - (position - 1)];

    for (i, &ch) in available_chars.iter().enumerate() {
        let digit = ch as u64;
        if digit > max {
            max = digit;
            max_index = i;
        }
    }
    if position > 1 {
        let remaining_chars = &bank[max_index + 1..bank.len()];
        max = 10u64.pow(position as u32 - 1) * max + get_max_jolt(remaining_chars, position - 1);
    }
    max
}

impl AoCSolution for Solution {
    type Parsed = Vec<Vec<u8>>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter().map(|bank| get_max_jolt(bank, 2)).sum::<u64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut ret = 0;
        for bank in data {
            ret += get_max_jolt(bank, 12);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
987654321111111
811111111111119
234234234234278
818181911112111
";

    fn digits(s: &str) -> Vec<u8> {
        s.bytes().map(|b| b - b'0').collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "357");
    }

    #[test]
    fn test_get_max_jots() {
        assert_eq!(get_max_jolt(&digits("19"), 1), 9);
        assert_eq!(get_max_jolt(&digits("91"), 1), 9);
        assert_eq!(get_max_jolt(&digits("7"), 1), 7);
        assert_eq!(get_max_jolt(&digits("119"), 2), 19);
        assert_eq!(get_max_jolt(&digits("987654321111111"), 12), 987654321111);
        assert_eq!(get_max_jolt(&digits("811111111111119"), 12), 811111111119);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "3121910778619");
    }
}
