use crate::{AoCSolution, parsing::numbers_to_pair};

const YEAR: u16 = 2024;
const DAY: u8 = 01;
pub struct Solution {}

fn input_to_vectors(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    lines.iter().map(|line| numbers_to_pair(line)).unzip()
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let (mut list1, mut list2) = input_to_vectors(&crate::parsing::input_to_vectors(input));

        list1.sort();
        list2.sort();

        list1
            .iter()
            .zip(list2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut list1, mut list2) = input_to_vectors(&crate::parsing::input_to_vectors(input));

        list1.sort();
        list2.sort();

        let mut sum = 0;
        for &num in &list1 {
            let count = list2.iter().filter(|&&x| x == num).count() as i64;
            sum += num * count;
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#;

        let (firsts, seconds) = input_to_vectors(&crate::parsing::input_to_vectors(EXAMPLE_INPUT));

        assert_eq!(firsts, [3, 4, 2, 1, 3, 3]);
        assert_eq!(seconds, [4, 3, 5, 3, 9, 3]);

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "11");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#;

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "31");
    }
}
