use crate::{
    AoCSolution,
    parsing::{input_to_vectors, line_numbers_to_vec},
};

const YEAR: u16 = 2024;
const DAY: u8 = 02;
pub struct Solution {}

fn is_valid_pair(a: &i64, b: &i64, ascending: &bool) -> bool {
    let diff = b - a;
    if *ascending && diff > 0 && diff <= 3 {
        return true;
    }
    if !*ascending && diff < 0 && diff >= -3 {
        return true;
    }
    false
}

fn is_valid_sequence(numbers: &[i64], ascending: &bool) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    numbers
        .windows(2)
        .all(|w| is_valid_pair(&w[0], &w[1], ascending))
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut count = 0;

        for line in input_to_vectors(input) {
            let nums = line_numbers_to_vec(&line);
            let ascending = nums[1] > nums[0];

            if is_valid_sequence(&nums, &ascending) {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut count = 0;

        for line in input_to_vectors(input) {
            let nums = line_numbers_to_vec(&line);
            let length = nums.len();

            if is_valid_sequence(&nums, &(nums[1] > nums[0])) {
                count += 1;
                continue;
            }

            for i in 0..length {
                let mut pruned_list = nums.clone();
                pruned_list.remove(i);
                if is_valid_sequence(&pruned_list, &(pruned_list[1] > pruned_list[0])) {
                    count += 1;
                    break;
                }
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#;

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "2");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#;

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "4");
    }
}
