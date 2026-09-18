use md5::Digest;
use md5::Md5;
use std::fmt::Display;

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

fn first_with_leading_zeros(key: &str, num_zeros: usize) -> u64 {
    (0u64..)
        .find(|n| {
            let hash = Md5::digest(format!("{key}{n}"));

            let front: &[u8] = &hash[..num_zeros / 2];

            // Each hash byte gives 2 digits in hex. If odd number of zeros, make sure the additional byte starts with 0, i.e. < 0x10
            if num_zeros % 2 == 1 && hash[num_zeros / 2] >= 0x10 {
                return false;
            }
            front.iter().all(|&d| d == 0)
        })
        .expect("No matching hash")
}

impl AoCSolution for Solution {
    type Parsed = String;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.trim().to_string()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        first_with_leading_zeros(data, 5)
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        first_with_leading_zeros(data, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sol = Solution::default();

        const INPUT1: &str = "abcdef";
        assert_eq!(sol.solve1(INPUT1), "609043");

        // This runs slow
        // const INPUT2: &str = "pqrstuv";
        // assert_eq!(sol.solve1(INPUT2), "1048970");
    }
}
