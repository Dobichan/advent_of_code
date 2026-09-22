use std::fmt::Display;

use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::{AoCSolution, iterators::GosperIterator};

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug)]
pub struct Machine {
    indicators: u32,
    buttons: Vec<Vec<usize>>,
    buttons_bitmask: Vec<u32>,
    joltage_requirements: Vec<Joltage>,
}

#[derive(Debug)]
pub struct Joltage {
    level: isize,
    contributing_buttons: Vec<usize>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let elements: Vec<_> = input.trim().split(' ').collect();
        let indicators = elements[0];
        let joltage = elements[elements.len() - 1];

        let buttons: Vec<Vec<usize>> = elements[1..elements.len() - 1] // Only process buttons
            .iter()
            .map(|btn| {
                btn[1..btn.len() - 1] // Skip '(' and ')' - first and last
                    .split(',')
                    .map(|idx| idx.parse().unwrap())
                    .collect()
            })
            .collect();

        let buttons_bitmask: Vec<u32> = buttons
            .iter()
            .map(|btn| btn.iter().map(|&idx| 1u32 << idx).sum())
            .collect();

        let joltages: Vec<Joltage> = joltage[1..joltage.len() - 1] // skip '{' and '}' - first and last
            .split(',')
            .enumerate()
            .map(|(i, jo)| Joltage {
                level: jo.parse::<isize>().unwrap(),
                contributing_buttons: buttons_bitmask
                    .iter()
                    .positions(|mask| mask & 1u32 << i != 0)
                    .collect(),
            })
            .collect();

        Self {
            indicators: indicators[1..indicators.len() - 1]
                .chars()
                .enumerate()
                .map(|(i, c)| if c == '#' { 1u32 << i } else { 0 })
                .sum(),
            buttons_bitmask,
            buttons,
            joltage_requirements: joltages,
        }
    }

    fn get_min_btn_presses_leds(&self) -> u32 {
        let num_buttons = self.buttons_bitmask.len();
        let index_numbers = GosperIterator::new((1u32 << num_buttons) - 1);

        for button_mask in index_numbers {
            let mut res = 0;
            for i in 0..num_buttons {
                if (button_mask & 1u32 << i) != 0 {
                    res ^= self.buttons_bitmask[i];
                }
            }
            if res == self.indicators {
                return button_mask.count_ones();
            }
        }

        panic!("Solution not found! (Should not happen)");
    }

    fn get_min_button_presses_joltage(&self) -> isize {
        let mut residual: Vec<isize> = self.joltage_requirements.iter().map(|j| j.level).collect();
        let mut presses = vec![None; self.buttons.len()];

        self.min_presses(&mut residual, &mut presses, 0, isize::MAX)
            .expect("Machine has no valid button combination to reach desired joltage levels")
    }

    fn min_presses(
        &self,
        residual: &mut [isize],        // remaining battery level
        presses: &mut [Option<isize>], // Number of presses per button
        spent: isize,                  // passes committed on this branch
        mut bound: isize,              // only look at totals below this
    ) -> Option<isize> {
        let max_res = *residual.iter().max().unwrap();
        let sum_res: isize = residual.iter().sum();
        let widest = self.buttons.iter().map(Vec::len).max().unwrap() as isize;
        let need = max_res.max((sum_res + widest - 1) / widest);
        if spent + need >= bound {
            return None;
        }

        let mut pick: Option<(usize, usize, Vec<usize>)> = None;
        for (c, j) in self.joltage_requirements.iter().enumerate() {
            let free: Vec<usize> = j
                .contributing_buttons
                .iter()
                .copied()
                .filter(|&b| presses[b].is_none())
                .collect();

            match (free.is_empty(), residual[c]) {
                (true, 0) => continue,    // satisfied
                (true, _) => return None, // fixed buttons miss the target
                (false, r) => {
                    let key = if r == 0 { 0 } else { free.len() };
                    if pick.as_ref().is_none_or(|(k, ..)| key < *k) {
                        pick = Some((key, c, free));
                    }
                }
            }
        }

        let Some((_, counter, free)) = pick else {
            return Some(spent);
        };

        let target = residual[counter];
        let button = free[0];
        let affected = &self.buttons[button];

        // Last free button for this counter: its value is forced
        let range = if free.len() == 1 {
            target..=target
        } else {
            0..=target
        };
        let mut best = None;

        for n in range {
            if affected.iter().any(|&c| residual[c] < n) {
                break; // would overshoot a counter this button feeds
            }

            presses[button] = Some(n);
            affected.iter().for_each(|&c| residual[c] -= n);

            if let Some(total) = self.min_presses(residual, presses, spent + n, bound) {
                bound = total; // later branches must beat this
                best = Some(total);
            }

            affected.iter().for_each(|&c| residual[c] += n);
            presses[button] = None;
        }

        best
    }
}

impl AoCSolution for Solution {
    type Parsed = Vec<Machine>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input.trim().lines().map(Machine::new).collect()
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        data.iter()
            .map(|m| m.get_min_btn_presses_leds())
            .sum::<u32>()
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        data.par_iter()
            .map(|m| {
                let a = m.get_min_button_presses_joltage();
                println!("presses: {a}");
                a
            })
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
