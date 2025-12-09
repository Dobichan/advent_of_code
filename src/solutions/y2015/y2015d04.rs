use crate::AoCSolution;
use md5::Digest;
use md5::Md5;

const YEAR: u16 = 2015;
const DAY: u8 = 4;
pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let mut done = false;
        let mut append = 0;

        while !done {
            let check = format!("{}{}", input, append);
            let md5 = format!("{:x}", Md5::digest(check.as_bytes()));
            if md5.starts_with("00000") {
                done = true;
            } else {
                append += 1;
            }
        }

        append.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut done = false;
        let mut append = 0;

        while !done {
            let check = format!("{}{}", input, append);
            let md5 = format!("{:x}", Md5::digest(check.as_bytes()));
            if md5.starts_with("000000") {
                done = true;
            } else {
                append += 1;
            }
        }

        append.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut sol = Solution {};
        const INPUT1: &str = "abcdef";
        assert_eq!(sol.part1(INPUT1), "609043");
    }

    #[test]
    fn test_part2() {}
}
