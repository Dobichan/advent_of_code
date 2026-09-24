use std::{collections::HashMap, fmt::Display};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct Machine {
    part1_led_pattern: usize,         // Affected LEDs for part 1
    toggle_masks: Vec<usize>, // Resulting battery/led affected for each button combo precomputed
    buttons: Vec<Vec<usize>>, // Battery indexes each button affects
    joltage_requirements: Vec<isize>, // Required jotage level of all the batteries
}

impl Machine {
    fn new(input: &str) -> Self {
        let elements: Vec<_> = input.trim().split(' ').collect();
        let indicators = elements[0];
        let joltage = elements[elements.len() - 1];

        let part1_led_pattern = mask_from_bools(
            indicators[1..indicators.len() - 1]
                .chars()
                .map(|c| c == '#'),
        );

        let buttons: Vec<Vec<usize>> = elements[1..elements.len() - 1] // Only process buttons
            .iter()
            .map(|btn| {
                btn[1..btn.len() - 1] // Skip '(' and ')' - first and last
                    .split(',')
                    .map(|idx| idx.parse().unwrap())
                    .collect()
            })
            .collect();

        let buttons_bitmask: Vec<usize> = buttons
            .iter()
            .map(|btn| btn.iter().map(|&idx| 1usize << idx).sum())
            .collect();

        let num_buttons = buttons_bitmask.len();

        let toggle_masks = (0..1usize << num_buttons)
            .map(|btn_mask| {
                (0..num_buttons)
                    .map(|i| {
                        if has_bit(btn_mask, i) {
                            buttons_bitmask[i]
                        } else {
                            0
                        }
                    })
                    .fold(0, |acc, btn_mask| acc ^ btn_mask)
            })
            .collect();

        let joltages: Vec<isize> = joltage[1..joltage.len() - 1] // skip '{' and '}' - first and last
            .split(',')
            .map(|jo| jo.parse::<isize>().unwrap())
            .collect();

        Self {
            part1_led_pattern,
            toggle_masks,
            buttons,
            joltage_requirements: joltages,
        }
    }

    fn build_joltage_adjusts(&self) -> Vec<Vec<isize>> {
        let num_buttons = self.buttons.len();
        let num_batteries = self.joltage_requirements.len();
        let none_buttons = vec![];

        (0..1usize << num_buttons)
            .map(|btn_mask| {
                (0..num_buttons)
                    .map(|i| {
                        if has_bit(btn_mask, i) {
                            &self.buttons[i]
                        } else {
                            &none_buttons
                        }
                    })
                    .fold(vec![0; num_batteries], |mut acc, btn_mask| {
                        for &i in btn_mask {
                            acc[i] += 1
                        }
                        acc
                    })
            })
            .collect()
    }

    /// Returns a vec of bitmasks that indicates which button combinations
    /// results in the desired pattern
    fn get_button_combos(&self, pattern: usize) -> Vec<usize> {
        self.toggle_masks
            .iter()
            .enumerate()
            .filter(|(_, combo_result)| **combo_result == pattern)
            .map(|(i, _)| i)
            .collect()
    }

    fn get_min_btn_presses_leds(&self) -> usize {
        self.get_button_combos(self.part1_led_pattern)
            .iter()
            .map(|btns| btns.count_ones() as usize)
            .min()
            .unwrap()
    }

    fn get_min_button_presses_joltage(&self) -> isize {
        let joltage_adjusts = self.build_joltage_adjusts();
        let mut cache = HashMap::new();

        self.min_buttons_joltage(&self.joltage_requirements, &joltage_adjusts, &mut cache)
            .expect("Machine has no valid button combination to reach desired joltage levels")
    }

    fn min_buttons_joltage(
        &self,
        levels: &[isize],
        joltage_adjusts: &[Vec<isize>],
        cache: &mut HashMap<Vec<isize>, Option<isize>>,
    ) -> Option<isize> {
        if levels.iter().all(|&l| l == 0) {
            return Some(0);
        }

        if let Some(&cached) = cache.get(levels) {
            return cached;
        }

        let mut best: Option<isize> = None;

        let mask = parity_mask(levels);
        let valid_combinations = self.get_button_combos(mask);

        for combo in valid_combinations {
            let adjusts = &joltage_adjusts[combo];

            if adjusts
                .iter()
                .zip(levels)
                .any(|(adjust, level)| adjust > level)
            {
                continue; // It adjusts more than level and overshoots
            }

            let remaining_joltages: Vec<isize> = levels
                .iter()
                .zip(adjusts)
                .map(|(level, adjust)| (level - adjust) / 2)
                .collect();

            if let Some(remaining_presses) =
                self.min_buttons_joltage(&remaining_joltages, joltage_adjusts, cache)
            {
                let total = combo.count_ones() as isize + 2 * remaining_presses;
                best = Some(best.map_or(total, |b| b.min(total)));
            }
        }
        cache.insert(levels.to_vec(), best);

        best
    }
}

fn has_bit(mask: usize, i: usize) -> bool {
    1usize << i & mask != 0
}

fn mask_from_bools(bits: impl IntoIterator<Item = bool>) -> usize {
    bits.into_iter()
        .enumerate()
        .filter(|&(_, bit)| bit)
        .fold(0, |mask, (i, _)| mask | 1 << i)
}

fn parity_mask(levels: &[isize]) -> usize {
    levels
        .iter()
        .enumerate()
        .filter(|&(_, &level)| level & 1 != 0)
        .fold(0, |mask, (i, _)| mask | 1 << i)
}

impl AoCSolution for Solution {
    type Parsed = Vec<Machine>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.trim().lines().map(Machine::new).collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|m| m.get_min_btn_presses_leds())
            .sum::<usize>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.par_iter()
            .map(|m| m.get_min_button_presses_joltage())
            .sum::<isize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "33");
    }
}
