use std::fmt::Display;

use crate::AoCSolution;
use crate::parsing::input_lines;
use multimap::MultiMap;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

#[derive(Debug)]
pub struct ParseData {
    rules: MultiMap<u32, u32>,
    jobs: Vec<PrintJob>,
}
#[derive(Debug)]
struct PrintJob {
    pages: Vec<u32>,
}

impl PrintJob {
    fn is_valid(&self, rules: &MultiMap<u32, u32>) -> bool {
        self.pages.iter().enumerate().all(|(i, page)| {
            rules
                .get_vec(page)
                .is_none_or(|page_after| !self.pages[..i].iter().any(|p| page_after.contains(p)))
        })
    }

    fn middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    fn correct_page_order(&self, rules: &MultiMap<u32, u32>) -> PrintJob {
        let mut reordered = Vec::with_capacity(self.pages.len());

        for &page in &self.pages {
            let pos = rules
                .get_vec(&page)
                .and_then(|page_after| reordered.iter().position(|p| page_after.contains(p)))
                .unwrap_or(reordered.len());
            reordered.insert(pos, page);
        }

        PrintJob { pages: reordered }
    }
}

fn page_rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("|"), u32).parse(input)
}

fn parse_job(input: &str) -> IResult<&str, PrintJob> {
    separated_list1(tag(","), u32)
        .map(|pages| PrintJob { pages })
        .parse(input)
}

#[derive(Debug, Default)]
pub struct Solution {}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let (rules, jobs) = input
            .split_once("\n\n")
            .expect("Missing blank line between rules and jobs");

        ParseData {
            rules: input_lines(rules)
                .map(|line| page_rule(line.trim()).expect("Failed to parse page rule").1)
                .collect(),
            jobs: input_lines(jobs)
                .map(|line| parse_job(line.trim()).expect("Could not parse job").1)
                .collect(),
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.jobs
            .iter()
            .filter(|job| job.is_valid(&data.rules))
            .map(|job| job.middle_page())
            .sum::<u32>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.jobs
            .iter()
            .filter(|job| !job.is_valid(&data.rules))
            .map(|job| job.correct_page_order(&data.rules).middle_page())
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "143")
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "123")
    }
}
