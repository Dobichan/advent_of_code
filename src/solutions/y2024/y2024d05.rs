use crate::AoCSolution;
use multimap::MultiMap;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

const YEAR: u16 = 2024;
const DAY: u8 = 5;

#[derive(Debug)]
struct PrintJob {
    pages: Vec<i64>,
}

impl PrintJob {
    fn is_valid(&self, rules: &MultiMap<i64, i64>) -> bool {
        for i in 0..self.pages.len() {
            if let Some(this_page_rule) = rules.get_vec(&self.pages[i]) {
                let pages_before: &[i64] = &self.pages.as_slice()[0..i];
                if pages_before
                    .iter()
                    .filter(|p| {
                        if this_page_rule.contains(p) {
                            true
                        } else {
                            false
                        }
                    })
                    .count()
                    > 0
                {
                    return false;
                }
            }
        }
        true
    }

    fn get_middle_page(&self) -> i64 {
        self.pages[self.pages.len() / 2]
    }

    fn get_correct_page_order(&self, rules: &MultiMap<i64, i64>) -> PrintJob {
        let mut reordered = Vec::with_capacity(self.pages.len());

        for page in self.pages.iter() {
            let mut inserted = false;

            if let Some(this_page_rules) = rules.get_vec(&page) {
                for (i, new_page) in reordered.iter().enumerate() {
                    if this_page_rules.contains(new_page) {
                        reordered.insert(i, *page);
                        inserted = true;
                        break;
                    }
                }
            }
            if !inserted {
                reordered.push(*page);
            }
        }

        PrintJob { pages: reordered }
    }
}

fn parse_job(input: &str) -> IResult<&str, PrintJob> {
    let (input, list) = separated_list1(tag(","), i64).parse(input)?;
    Ok((input, PrintJob { pages: list }))
}

fn page_rule(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(i64, tag("|"), i64).parse(input)
}

fn parse(input: &str) -> (MultiMap<i64, i64>, Vec<PrintJob>) {
    let (rules, jobs) = input.split_once("\n\n").unwrap();

    let mut map = MultiMap::new();

    for rule in rules.trim().lines() {
        let r = page_rule(rule.trim()).expect("Failed to parse page rule");
        map.insert(r.1.0, r.1.1);
    }

    let mut all_jobs = Vec::new();
    for job in jobs.trim().lines() {
        let j = parse_job(job.trim()).expect("Could not parse job");
        all_jobs.push(j.1);
    }
    (map, all_jobs)
}

pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let (rules, jobs) = parse(input);

        let mut ret = 0;
        for job in jobs {
            if job.is_valid(&rules) {
                ret += job.get_middle_page();
            }
        }

        ret.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let (rules, jobs) = parse(input);

        let mut ret = 0;
        for job in jobs {
            if !job.is_valid(&rules) {
                let shuffled = job.get_correct_page_order(&rules);

                ret += shuffled.get_middle_page();
            }
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        97,13,75,29,47";

        let mut sol = Solution {};
        assert_eq!(sol.part1(EXAMPLE_INPUT), "143")
    }

    #[test]
    fn test_part2() {
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
        97,13,75,29,47";

        let mut sol = Solution {};
        assert_eq!(sol.part2(EXAMPLE_INPUT), "123")
    }
}
