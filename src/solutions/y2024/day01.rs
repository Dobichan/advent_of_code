use std::fmt::Display;

use crate::{
    AoCSolution,
    parsing::{input_lines, numbers_to_pair},
};

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct ParsedData {
    list1: Vec<i64>,
    list2: Vec<i64>,
}

impl AoCSolution for Solution {
    type Parsed = ParsedData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let (mut list1, mut list2): (Vec<i64>, Vec<i64>) =
            input_lines(input).map(numbers_to_pair).unzip();

        list1.sort_unstable();
        list2.sort_unstable();

        ParsedData { list1, list2 }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.list1
            .iter()
            .zip(&data.list2)
            .map(|(a, b)| (a - b).abs())
            .sum::<i64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut i = 0;
        let mut j = 0;
        let mut total = 0;

        let len1 = data.list1.len();
        let len2 = data.list2.len();

        while i < len1 && j < len2 {
            if data.list1[i] < data.list2[j] {
                i += 1
            } else if data.list1[i] > data.list2[j] {
                j += 1;
            } else {
                let value = data.list1[i];
                let mut count1 = 0;
                let mut count2 = 0;
                while i < len1 && data.list1[i] == value {
                    count1 += 1;
                    i += 1;
                }
                while j < len2 && data.list2[j] == value {
                    count2 += 1;
                    j += 1;
                }
                total += value * count1 * count2;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        let sol = Solution::default();

        let data = sol.parse(EXAMPLE_INPUT);
        assert_eq!(data.list1, [1, 2, 3, 3, 3, 4]);
        assert_eq!(data.list2, [3, 3, 3, 4, 5, 9]);

        assert_eq!(sol.solve1(EXAMPLE_INPUT), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "31");
    }
}
