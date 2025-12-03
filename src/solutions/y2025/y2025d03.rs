use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 3;
pub struct Solution {}

pub fn get_max_jolt(bank: &str, position: usize) -> i64 {
    let mut max = 0;
    let mut max_index = 0;

    let available_chars = &bank[0..bank.len() - (position - 1)];

    for (i, ch) in available_chars.chars().enumerate() {
        let digit: i64 = (ch as u8 - '0' as u8) as i64;
        if digit > max {
            max = digit;
            max_index = i;
        }
    }
    if position > 1 {
        let remaining_chars = &bank[max_index + 1..bank.len()];
        max = 10i64.pow(position as u32 - 1) * max + get_max_jolt(remaining_chars, position - 1);
    }
    max
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut ret = 0;

        for bank in input.trim().lines() {
            ret += get_max_jolt(bank, 2);
        }
        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut ret = 0;
        for bank in input.trim().lines() {
            ret += get_max_jolt(bank, 12);
        }
        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "987654321111111\n\
                    811111111111119\n\
                    234234234234278\n\
                    818181911112111";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "357");
    }

    #[test]
    fn test_get_max_jots() {
        let input = "19";
        let res = get_max_jolt(input, 1);
        assert_eq!(res, 9);

        let input = "91";
        let res = get_max_jolt(input, 1);
        assert_eq!(res, 9);

        let input = "7";
        let res = get_max_jolt(input, 1);
        assert_eq!(res, 7);

        let input = "119";
        let res = get_max_jolt(input, 2);
        assert_eq!(res, 19);

        let input = "987654321111111";
        let res = get_max_jolt(input, 12);
        assert_eq!(res, 987654321111);

        let input = "811111111111119";
        let res = get_max_jolt(input, 12);
        assert_eq!(res, 811111111119);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "987654321111111\n\
                    811111111111119\n\
                    234234234234278\n\
                    818181911112111";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "3121910778619");
    }
}
