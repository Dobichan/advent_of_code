use std::{cell::RefCell, collections::HashMap, fmt::Display};

use crate::AoCSolution;

#[derive(Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct Data {
    numbers: Vec<u64>,
    cache: RefCell<HashMap<(usize, u64), u64>>,
}

fn blink(digit: u64, blinks_left: usize, cache: &mut HashMap<(usize, u64), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    } else if let Some(&count) = cache.get(&(blinks_left, digit)) {
        return count;
    } else if digit == 0 {
        let ret = blink(1, blinks_left - 1, cache);
        cache.insert((blinks_left, digit), ret);
        return ret;
    }

    let num_digits = digit.ilog10();
    if (num_digits + 1).is_multiple_of(2) {
        // even digits - split in half

        let orig_digit = digit;
        let a = digit / 10u64.pow(num_digits.div_ceil(2));
        let b = orig_digit - a * 10u64.pow(num_digits.div_ceil(2));
        let ret = blink(a, blinks_left - 1, cache) + blink(b, blinks_left - 1, cache);
        cache.insert((blinks_left, digit), ret);
        ret
    } else {
        let ret = blink(digit * 2024, blinks_left - 1, cache);
        cache.insert((blinks_left, digit), ret);
        ret
    }
}

impl AoCSolution for Solution {
    type Parsed = Data;

    fn parse(&self, input: &str) -> Self::Parsed {
        Data {
            numbers: input
                .trim()
                .split(' ')
                .map(|d| d.parse::<u64>().unwrap())
                .collect(),
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        let mut cache = data.cache.borrow_mut();
        data.numbers
            .iter()
            .map(|&d| blink(d, 25, &mut cache))
            .sum::<u64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut cache = data.cache.borrow_mut();
        data.numbers
            .iter()
            .map(|&d| blink(d, 75, &mut cache))
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
    125 17
";

    fn example_data() -> Data {
        Solution::default().parse(EXAMPLE_INPUT)
    }

    #[test]
    fn test_1_blink() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 1, &mut cache)).sum();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_2_blinks() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 2, &mut cache)).sum();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_3_blinks() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 3, &mut cache)).sum();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_4_blinks() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 4, &mut cache)).sum();
        assert_eq!(count, 9);
    }

    #[test]
    fn test_5_blinks() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 5, &mut cache)).sum();
        assert_eq!(count, 13);
    }

    #[test]
    fn test_6_blinks() {
        let input = example_data();
        let mut cache = input.cache.borrow_mut();

        let count: u64 = input.numbers.iter().map(|&d| blink(d, 6, &mut cache)).sum();
        assert_eq!(count, 22);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "55312");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "65601038650482");
    }
}
