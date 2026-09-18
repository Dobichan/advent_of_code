use fancy_regex::Regex;
use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Debug)]
pub struct Solution {
    rules: Rules,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            rules: Rules::new(),
        }
    }
}

#[derive(Debug)]
pub struct Rules {
    vowels_check: Regex,
    repeat_character: Regex,
    illegal_group: Regex,
    repeating_pair: Regex,
    spaced_double_letter: Regex,
}

impl Rules {
    fn new() -> Self {
        Self {
            vowels_check: Regex::new(r"([aeiou])").unwrap(),
            repeat_character: Regex::new(r"(.)\1").unwrap(),
            illegal_group: Regex::new(r"(ab|cd|pq|xy)").unwrap(),
            repeating_pair: Regex::new(r"(.)(.).*(\1\2)").unwrap(),
            spaced_double_letter: Regex::new(r"(.).(\1)").unwrap(),
        }
    }

    fn is_nice_part1(&self, line: &str) -> bool {
        self.vowels_check.find_iter(line).count() >= 3
            && self.repeat_character.is_match(line).unwrap()
            && !self.illegal_group.is_match(line).unwrap()
    }

    fn is_nice_part2(&self, line: &str) -> bool {
        self.repeating_pair.is_match(line).unwrap()
            && self.spaced_double_letter.is_match(line).unwrap()
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<String>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input).map(|l| l.trim().to_string()).collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter().filter(|l| self.rules.is_nice_part1(l)).count()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter().filter(|l| self.rules.is_nice_part2(l)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let r = Rules::new();

        assert!(r.is_nice_part1("ugknbfddgicrmopn"));
        assert!(r.is_nice_part1("aaa"));
        assert!(!r.is_nice_part1("jchzalrnumimnmhp"));
        assert!(!r.is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!r.is_nice_part1("dvszwmarrgswjxmb"));

        const EXAMPLE_INPUT: &str = r"
ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb
";

        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        let r = Rules::new();
        assert!(r.is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(r.is_nice_part2("xxyxx"));
        assert!(!r.is_nice_part2("uurcxstgmygtbstg"));
        assert!(!r.is_nice_part2("ieodomkazucvgmuy"));

        const EXAMPLE_INPUT: &str = r"
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
";

        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "2");
    }
}
