use std::{collections::BTreeSet, fmt::Display};

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::u64, multi::separated_list1,
};

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn repeated_ids(from: u64, to: u64, blok_len: u32, times: u32) -> Vec<u64> {
    let unit = 10u64.pow(blok_len);
    let multiplier = (0..times).fold(0, |acc, _| acc * unit + 1);

    let smallest_block = unit / 10;
    let largest_block = unit - 1;

    let first = smallest_block.max(from.div_ceil(multiplier));
    let last = largest_block.min(to / multiplier);

    (first..=last).map(|block| block * multiplier).collect()
}

fn id_range(input: &str) -> IResult<&str, (u64, u64)> {
    (u64, tag("-"), u64)
        .map(|(from, _, to)| (from, to))
        .parse(input)
}

fn doubled_ids(from: u64, to: u64) -> Vec<u64> {
    let mut ids = Vec::new();
    for block_len in 1..=digits(to) / 2 {
        ids.extend(repeated_ids(from, to, block_len, 2));
    }

    ids
}

fn any_repeated_ids(from: u64, to: u64) -> BTreeSet<u64> {
    let mut ids = BTreeSet::new();
    for total_len in digits(from)..=digits(to) {
        for block_len in 1..total_len {
            if total_len % block_len == 0 {
                ids.extend(repeated_ids(from, to, block_len, total_len / block_len));
            }
        }
    }
    ids
}

impl AoCSolution for Solution {
    type Parsed = Vec<(u64, u64)>;

    fn parse(&self, input: &str) -> Self::Parsed {
        separated_list1(tag(","), id_range)
            .parse(input.trim())
            .expect("Malformed input")
            .1
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|&(from, to)| doubled_ids(from, to).iter().sum::<u64>())
            .sum::<u64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|&(from, to)| any_repeated_ids(from, to).iter().sum::<u64>())
            .sum::<u64>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test_parsing() {
        let sol = Solution::default();
        assert_eq!(sol.parse("11-22"), [(11, 22)]);
        assert_eq!(sol.parse("11-22,95-115"), [(11, 22), (95, 115)]);
        assert_eq!(
            sol.parse("11-22,95-115,123-156\n"),
            [(11, 22), (95, 115), (123, 156)]
        );
    }
    #[test]
    fn test_repeated_ids() {
        assert_eq!(
            repeated_ids(1, 99, 1, 2),
            [11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
        assert_eq!(repeated_ids(12121212, 12121212, 2, 4), [12121212]);
        assert_eq!(repeated_ids(824824824, 824824824, 3, 3), [824824824]);
        assert_eq!(repeated_ids(1000, 1009, 2, 2), []);
    }

    #[test]
    fn test_doubled_ids() {
        assert_eq!(doubled_ids(11, 22), [11, 22]);
        assert_eq!(doubled_ids(95, 115), [99]);
        assert_eq!(doubled_ids(998, 1012), [1010]);
        assert_eq!(doubled_ids(1188511880, 1188511890), [1188511885]);
        assert_eq!(doubled_ids(222220, 222224), [222222]);
        assert_eq!(doubled_ids(446443, 446449), [446446]);
        assert_eq!(doubled_ids(38593856, 38593862), [38593859]);
    }

    #[test]
    fn test_any_repeated_ids() {
        let ids = |from, to| any_repeated_ids(from, to).into_iter().collect::<Vec<_>>();
        assert_eq!(ids(11, 22), [11, 22]);
        assert_eq!(ids(95, 115), [99, 111]);
        assert_eq!(ids(998, 1012), [999, 1010]);
        assert_eq!(ids(1188511880, 1188511890), [1188511885]);
        assert_eq!(ids(222220, 222224), [222222]);
        assert_eq!(ids(1698522, 1698528), []);
        assert_eq!(ids(38593856, 38593862), [38593859]);
        assert_eq!(ids(565653, 565659), [565656]);
        assert_eq!(ids(824824821, 824824827), [824824824]);
        assert_eq!(ids(2121212118, 2121212124), [2121212121]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "4174379265");
    }
}
