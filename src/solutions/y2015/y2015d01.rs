use crate::AoCSolution;

const YEAR: u16 = 2015;
const DAY: u8 = 1;

pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .chars()
            .map(|c| if c == '(' { 1 } else { -1 })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut lvl = 0;
        for (i, c) in input.chars().enumerate() {
            if c == '(' {
                lvl += 1;
            } else {
                lvl -= 1;
            }
            if lvl == -1 {
                return (i + 1).to_string();
            }
        }
        "-1".to_string()
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

        let mut sol = Solution {};

        assert_eq!(sol.part1(INPUT1A), "0");
        assert_eq!(sol.part1(INPUT1B), "0");
        assert_eq!(sol.part1(INPUT2A), "3");
        assert_eq!(sol.part1(INPUT2B), "3");
        assert_eq!(sol.part1(INPUT3), "3");
        assert_eq!(sol.part1(INPUT4A), "-1");
        assert_eq!(sol.part1(INPUT4B), "-1");
        assert_eq!(sol.part1(INPUT5A), "-3");
        assert_eq!(sol.part1(INPUT5B), "-3");
    }

    #[test]
    fn test_part2() {
        const INPUT1A: &str = r")";
        const INPUT1B: &str = r"()())";

        let mut sol = Solution {};

        assert_eq!(sol.part2(INPUT1A), "1");
        assert_eq!(sol.part2(INPUT1B), "5");
    }
}
