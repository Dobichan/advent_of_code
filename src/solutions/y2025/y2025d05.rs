use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 5;
pub struct Solution {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdRange {
    from: u64,
    to: u64,
}

impl IdRange {
    fn new(from: &str, to: &str) -> IdRange {
        let from: u64 = from.parse().expect("illegal string received");
        let to: u64 = to.parse().expect("illegal string received");

        IdRange { from, to }
    }

    fn in_range(&self, id: u64) -> bool {
        if id >= self.from && id <= self.to {
            return true;
        }
        false
    }
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let ranges_and_ids: Vec<_> = input.trim().split("\n\n").collect();
        let ranges: Vec<_> = ranges_and_ids[0]
            .lines()
            .map(|range| {
                let temp: Vec<&str> = range.split('-').collect();
                IdRange::new(temp[0], temp[1])
            })
            .collect();
        let available_ids: Vec<_> = ranges_and_ids[1]
            .lines()
            .map(|id| id.parse::<u64>().expect("Illegal ID detected"))
            .collect();
        let mut ret = 0;

        for id in available_ids {
            for range in &ranges {
                if range.in_range(id) {
                    ret += 1;
                    break;
                }
            }
        }

        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges_and_ids: Vec<_> = input.trim().split("\n\n").collect();
        let mut ranges: Vec<_> = ranges_and_ids[0]
            .lines()
            .map(|range| {
                let temp: Vec<&str> = range.split('-').collect();
                IdRange::new(temp[0], temp[1])
            })
            .collect();
        ranges.sort();

        let mut ret: u64 = 0;
        let mut moving_start = 0;

        for range in ranges {
            let mut temp_start = range.from;

            if moving_start >= range.from {
                temp_start = moving_start + 1;
            }

            if temp_start <= range.to {
                ret += range.to - temp_start + 1;
            }
            moving_start = moving_start.max(range.to);
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "3-5\n\
                    10-14\n\
                    16-20\n\
                    12-18\n\
                    \n\
                    1\n\
                    5\n\
                    8\n\
                    11\n\
                    17\n\
                    32";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "3");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "3-5\n\
                    10-14\n\
                    16-20\n\
                    12-18\n\
                    \n\
                    1\n\
                    5\n\
                    8\n\
                    11\n\
                    17\n\
                    32";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "14");
    }
}
