use std::collections::HashMap;

use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 2;
pub struct Solution {}

#[derive(Debug, PartialEq)]
pub struct ProductIdRange {
    lower: u64,
    upper: u64,
    start_prefix: u64,
}

impl ProductIdRange {
    fn new(lower: &str, upper: &str) -> Self {
        let low: u64 = lower.parse().unwrap();
        let high: u64 = upper.parse().unwrap();
        let start_num_digits = lower.len();
        let prefix: u64;

        if start_num_digits % 2 == 0 {
            prefix = lower[0..start_num_digits / 2].parse().unwrap();
        } else {
            prefix = 10u64.pow(start_num_digits as u32 / 2);
        }

        ProductIdRange {
            lower: low,
            upper: high,
            start_prefix: prefix,
        }
    }

    fn get_illegal_ids(&self) -> Vec<u64> {
        let mut test_num = self.start_prefix;
        let mut ret = Vec::with_capacity(100);
        loop {
            let illegal = create_illegal_number(test_num);
            if illegal > self.upper {
                break;
            }
            if self.number_in_range(illegal) {
                ret.push(illegal);
            }
            test_num += 1;
        }
        ret
    }

    fn get_illegal_ids2(&self) -> Vec<u64> {
        let start_num_digits = self.lower.ilog10() + 1;
        let end_num_digits = self.upper.ilog10() + 1;
        let mut ids = HashMap::new();

        for i in start_num_digits..=end_num_digits {
            let max = create_illegal_number2(9, (i / 2 + 1) as i32);
            for digit in 1..max {
                let num_digits = digit.ilog10() + 1;
                if i % num_digits != 0 {
                    continue;
                }
                let test_num = create_illegal_number2(digit, i as i32);
                if self.number_in_range(test_num) {
                    ids.insert(test_num, 1);
                }
            }
        }
        let mut ret = Vec::with_capacity(ids.keys().count());
        for key in ids.keys() {
            ret.push(*key);
        }
        ret.sort();
        ret
    }

    fn number_in_range(&self, num: u64) -> bool {
        if num >= self.lower && num <= self.upper {
            return true;
        }
        false
    }
}

fn parse(input: &str) -> Vec<ProductIdRange> {
    let mut ret = Vec::with_capacity(30);
    for part in input.trim().split(',') {
        let nums: Vec<_> = part.split('-').collect();
        let prod = ProductIdRange::new(nums[0], nums[1]);
        ret.push(prod);
    }
    ret
}

fn create_illegal_number(prefix: u64) -> u64 {
    let num_digits = prefix.ilog10() + 1;

    10u64.pow(num_digits) * prefix + prefix
}

fn create_illegal_number2(prefix: u64, num_digits: i32) -> u64 {
    let prefix_digits = prefix.ilog10() as i32 + 1;

    if prefix_digits > num_digits / 2 {
        return 0;
    }

    let mut current_digits = num_digits - prefix_digits;
    let mut ret = 0;
    while current_digits > 0 {
        ret += 10u64.pow(current_digits as u32) * prefix;
        current_digits -= prefix_digits;
    }
    ret += prefix;
    ret
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let products = parse(input);
        let mut ret: u64 = 0;

        for product in products {
            let temp_sum: u64 = product.get_illegal_ids().iter().sum();
            ret += temp_sum;
        }

        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let products = parse(input);
        let mut ret: u64 = 0;

        for product in products {
            let temp_sum: u64 = product.get_illegal_ids2().iter().sum();
            ret += temp_sum;
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_one_range() {
        const INPUT: &str = "11-22";

        assert_eq!(
            parse(&INPUT),
            [ProductIdRange {
                lower: 11,
                upper: 22,
                start_prefix: 1,
            }]
        );
    }

    #[test]
    fn test_parsing_two_ranges() {
        const INPUT: &str = "11-22,95-115";

        assert_eq!(
            parse(&INPUT),
            [
                ProductIdRange {
                    lower: 11,
                    upper: 22,
                    start_prefix: 1
                },
                ProductIdRange {
                    lower: 95,
                    upper: 115,
                    start_prefix: 9,
                }
            ]
        );
    }

    #[test]
    fn test_parsing_three_ranges() {
        const INPUT: &str = "11-22,95-115,123-156";

        assert_eq!(
            parse(&INPUT),
            [
                ProductIdRange {
                    lower: 11,
                    upper: 22,
                    start_prefix: 1
                },
                ProductIdRange {
                    lower: 95,
                    upper: 115,
                    start_prefix: 9,
                },
                ProductIdRange {
                    lower: 123,
                    upper: 156,
                    start_prefix: 10,
                }
            ]
        );
    }

    #[test]
    fn test_create_illegal_number() {
        let num1 = create_illegal_number(1u64);
        assert_eq!(num1, 11);

        let num2 = create_illegal_number(123u64);
        assert_eq!(num2, 123123);

        let num3 = create_illegal_number(11885u64);
        assert_eq!(num3, 1188511885);
    }

    #[test]
    fn test_number_in_product_range() {
        let num1 = create_illegal_number(123);
        let num2 = create_illegal_number(131);
        let test_range1 = ProductIdRange::new("120000", "130000");

        assert!(test_range1.number_in_range(num1));
        assert!(!test_range1.number_in_range(num2));
    }

    #[test]
    fn test_get_illegal_nums() {
        let test_range = ProductIdRange::new("11", "22");
        assert_eq!(test_range.get_illegal_ids(), [11, 22]);

        let test_range = ProductIdRange::new("95", "115");
        assert_eq!(test_range.get_illegal_ids(), [99]);

        let test_range = ProductIdRange::new("998", "1012");
        assert_eq!(test_range.get_illegal_ids(), [1010]);

        let test_range = ProductIdRange::new("1188511880", "1188511890");
        assert_eq!(test_range.get_illegal_ids(), [1188511885]);

        let test_range1 = ProductIdRange::new("222220", "222224");
        assert_eq!(test_range1.get_illegal_ids(), [222222]);

        let test_range = ProductIdRange::new("446443", "446449");
        assert_eq!(test_range.get_illegal_ids(), [446446]);

        let test_range = ProductIdRange::new("38593856", "38593862");
        assert_eq!(test_range.get_illegal_ids(), [38593859]);
    }

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,\
                                     222220-222224,1698522-1698528,446443-446449,\
                                     38593856-38593862,565653-565659,824824821-824824827,\
                                     2121212118-2121212124";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "1227775554");
    }

    #[test]
    fn test_creat_illegals_type2() {
        let test_num = create_illegal_number2(12, 8);
        assert_eq!(test_num, 12121212);

        let test_num = create_illegal_number2(9, 2);
        assert_eq!(test_num, 99);

        let test_num = create_illegal_number2(824, 9);
        assert_eq!(test_num, 824824824);
    }

    #[test]
    fn test_illegal_range_type2() {
        let test_range = ProductIdRange::new("11", "22");
        assert_eq!(test_range.get_illegal_ids2(), [11, 22]);

        let test_range = ProductIdRange::new("95", "115");
        assert_eq!(test_range.get_illegal_ids2(), [99, 111]);

        let test_range = ProductIdRange::new("998", "1012");
        assert_eq!(test_range.get_illegal_ids2(), [999, 1010]);

        let test_range = ProductIdRange::new("1188511880", "1188511890");
        assert_eq!(test_range.get_illegal_ids2(), [1188511885]);

        let test_range = ProductIdRange::new("222220", "222224");
        assert_eq!(test_range.get_illegal_ids2(), [222222]);

        let test_range = ProductIdRange::new("1698522", "1698528");
        assert_eq!(test_range.get_illegal_ids2(), []);

        let test_range = ProductIdRange::new("38593856", "38593862");
        assert_eq!(test_range.get_illegal_ids2(), [38593859]);

        let test_range = ProductIdRange::new("565653", "565659");
        assert_eq!(test_range.get_illegal_ids2(), [565656]);

        let test_range = ProductIdRange::new("824824821", "824824827");
        assert_eq!(test_range.get_illegal_ids2(), [824824824]);

        let test_range = ProductIdRange::new("2121212118", "2121212124");
        assert_eq!(test_range.get_illegal_ids2(), [2121212121]);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,\
                                     222220-222224,1698522-1698528,446443-446449,\
                                     38593856-38593862,565653-565659,824824821-824824827,\
                                     2121212118-2121212124";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "4174379265");
    }
}
