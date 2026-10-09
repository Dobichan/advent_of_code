use std::fmt::Display;

use crate::{AoCSolution, parsing::input_lines};

#[derive(Default)]
pub struct Solution {}

const KEYS: usize = 19 * 19 * 19 * 19; // each diff is in -9..=9, => 19 values

fn key_index(a: i8, b: i8, c: i8, d: i8) -> usize {
    let a = (a + 9) as usize;
    let b = (b + 9) as usize;
    let c = (c + 9) as usize;
    let d = (d + 9) as usize;
    ((a * 16 + b) * 19 + c) * 19 + d
}

fn pseudorandom(num: u32) -> u32 {
    let step1 = num << 6;

    let mut result = (num ^ step1) & 0xffffff;

    let step2 = result >> 5;
    result = (result ^ step2) & 0xffffff;

    let step3 = result << 11;
    result = (result ^ step3) & 0xffffff;

    result
}

fn get_num_keymap(num: u32, length: usize, map: &mut [u32]) {
    let mut numbers: Vec<i8> = Vec::with_capacity(length + 1);

    let mut temp_num = num;
    numbers.push((num % 10) as i8);
    for _i in 0..length {
        temp_num = pseudorandom(temp_num);
        numbers.push((temp_num % 10) as i8);
    }

    let mut seen = vec![false; KEYS];

    for i in 4..=length {
        let a = numbers[i - 3] - numbers[i - 4];
        let b = numbers[i - 2] - numbers[i - 3];
        let c = numbers[i - 1] - numbers[i - 2];
        let d = numbers[i] - numbers[i - 1];

        let key = key_index(a, b, c, d);
        if !seen[key] {
            seen[key] = true;
            map[key] += numbers[i] as u32;
        }
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<u32>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|n| n.parse::<u32>().unwrap())
            .collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|&n| {
                let mut val = n;
                for _i in 0..2000 {
                    val = pseudorandom(val);
                }
                val as u64
            })
            .sum::<u64>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        let mut map = vec![0u32; KEYS];

        for &n in data {
            get_num_keymap(n, 2000, &mut map);
        }
        *map.iter().max().unwrap() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r"
123
";

    const EXAMPLE_INPUT_2: &str = r"
1
10
100
2024
";

    #[test]
    fn test_part1_1() {
        let mut nums = Vec::new();
        nums.push(EXAMPLE_INPUT_1.trim().parse::<u32>().unwrap());
        for _ in 0..10 {
            nums.push(pseudorandom(*nums.last().unwrap()));
        }
        assert_eq!(
            nums,
            [
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254
            ]
        );
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT_2), "37327623");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT_2), "24");
    }
}
