use crate::AoCSolution;
use fancy_regex::Regex;

const YEAR: u16 = 2015;
const DAY: u8 = 5;
pub struct Solution {
    vowels_check: Regex,
    repeat_character: Regex,
    illegal_group: Regex,
    repeating_pair: Regex,
    spaced_double_letter: Regex,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            vowels_check: Regex::new("([aeiou])").unwrap(),
            repeat_character: Regex::new("(.)\\1").unwrap(),
            illegal_group: Regex::new("(ab|cd|pq|xy)").unwrap(),
            repeating_pair: Regex::new("(.)(.).*(\\1\\2)").unwrap(),
            spaced_double_letter: Regex::new("(.).(\\1)").unwrap(),
        }
    }
    fn is_nice_part1(&self, line: &str) -> bool {
        let vowels: Vec<_> = self.vowels_check.captures_iter(line.trim()).collect();
        if vowels.len() < 3 {
            return false;
        }
        let repeated_chars: Vec<_> = self.repeat_character.captures_iter(line.trim()).collect();
        if repeated_chars.len() < 1 {
            return false;
        }
        let illegal_groups: Vec<_> = self.illegal_group.captures_iter(line.trim()).collect();
        if illegal_groups.len() > 0 {
            return false;
        }

        true
    }

    fn is_nice_part2(&self, line: &str) -> bool {
        let repeating_pairs: Vec<_> = self.repeating_pair.captures_iter(line.trim()).collect();
        if repeating_pairs.len() < 1 {
            return false;
        }

        let double_letters: Vec<_> = self
            .spaced_double_letter
            .captures_iter(line.trim())
            .collect();
        if double_letters.len() < 1 {
            return false;
        }
        true
    }
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        input
            .trim()
            .lines()
            .filter(|l| self.is_nice_part1(l))
            .collect::<Vec<_>>()
            .len()
            .to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        input
            .trim()
            .lines()
            .filter(|l| self.is_nice_part2(l))
            .collect::<Vec<_>>()
            .len()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut sol = Solution::new();

        assert!(sol.is_nice_part1("ugknbfddgicrmopn"));
        assert!(sol.is_nice_part1("aaa"));
        assert!(!sol.is_nice_part1("jchzalrnumimnmhp"));
        assert!(!sol.is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!sol.is_nice_part1("dvszwmarrgswjxmb"));

        const EXAMPLE_INPUT: &str = r#"
            ugknbfddgicrmopn
            aaa
            jchzalrnumimnmhp
            haegwjzuvuyypxyu
            dvszwmarrgswjxmb
            "#;
        assert_eq!(sol.part1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        let mut sol = Solution::new();

        assert!(sol.is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(sol.is_nice_part2("xxyxx"));
        assert!(!sol.is_nice_part2("uurcxstgmygtbstg"));
        assert!(!sol.is_nice_part2("ieodomkazucvgmuy"));

        const EXAMPLE_INPUT: &str = r#"
            qjhvhtzxzqqjkmpb
            xxyxx
            uurcxstgmygtbstg
            ieodomkazucvgmuy
            "#;

        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "2");
    }
}
