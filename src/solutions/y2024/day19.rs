use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCSolution, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct ParseData {
    towels: Vec<String>,
    patterns: Vec<String>,
}

fn can_create<'a>(
    pattern: &'a str,
    towels: &Vec<String>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(&can) = cache.get(pattern) {
        return can;
    }

    let mut can = 0;
    for towel in towels {
        if let Some(remaining_pattern) = pattern.strip_prefix(towel.as_str())
            && can_create(remaining_pattern, towels, cache) == 1
        {
            can = 1;
            break;
        }
    }

    cache.insert(pattern, can);

    can
}

fn count_ways<'a>(
    pattern: &'a str,
    towels: &HashSet<&str>,
    max_len: usize,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(&known) = cache.get(pattern) {
        return known;
    }

    let mut total = 0;

    // Check start of pattern up to max len
    for len in 1..=max_len.min(pattern.len()) {
        // if there are any towel of length 'len' that matches pattern start, add it and continue.
        if towels.contains(&pattern[..len]) {
            total += count_ways(&pattern[len..], towels, max_len, cache);
        }
    }

    cache.insert(pattern, total);
    total
}

impl AoCSolution for Solution {
    type Parsed = ParseData;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut lines = input_lines(input);

        let towels = lines
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.trim().to_string())
            .collect();

        // Skip empty line
        let _ = lines.next();

        let patterns = lines.map(|t| t.to_string()).collect();

        Self::Parsed { towels, patterns }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut cache = HashMap::new();

        data.patterns
            .iter()
            // .inspect(|p| println!("{:?}", p))
            .map(|p| can_create(p, &data.towels, &mut cache))
            .sum::<usize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut cache = HashMap::new();
        // Make all towels into a HashSet for easy access.
        let towels: HashSet<&str> = data.towels.iter().map(|t| t.as_str()).collect();
        // Longest towel pattern
        let max_len = data.towels.iter().map(|t| t.len()).max().unwrap_or(0);

        data.patterns
            .iter()
            .map(|p| count_ways(p, &towels, max_len, &mut cache))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "16");
    }
}
