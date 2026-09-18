use std::fmt::Display;

use crate::AoCSolution;

#[derive(Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = Vec<i64>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                other => panic!("Unexpected character {other:?}"),
            })
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter().sum::<i64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .scan(0, |floor, step| {
                *floor += step;
                Some(*floor)
            })
            .position(|floor| floor == -1)
            .map_or(-1, |i| i as i64 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT1A: &str = r"(())";
        const INPUT1B: &str = r"()()";
        const INPUT2A: &str = r"(((";
        const INPUT2B: &str = r"(()(()(";
        const INPUT3: &str = r"))(((((";
        const INPUT4A: &str = r"())";
        const INPUT4B: &str = r"))(";
        const INPUT5A: &str = r")))";
        const INPUT5B: &str = r")())())";

        let sol = Solution::default();

        assert_eq!(sol.solve1(INPUT1A), "0");
        assert_eq!(sol.solve1(INPUT1B), "0");
        assert_eq!(sol.solve1(INPUT2A), "3");
        assert_eq!(sol.solve1(INPUT2B), "3");
        assert_eq!(sol.solve1(INPUT3), "3");
        assert_eq!(sol.solve1(INPUT4A), "-1");
        assert_eq!(sol.solve1(INPUT4B), "-1");
        assert_eq!(sol.solve1(INPUT5A), "-3");
        assert_eq!(sol.solve1(INPUT5B), "-3");
    }

    #[test]
    fn test_part2() {
        const INPUT1A: &str = r")";
        const INPUT1B: &str = r"()())";

        let sol = Solution::default();

        assert_eq!(sol.solve2(INPUT1A), "1");
        assert_eq!(sol.solve2(INPUT1B), "5");
    }
}
