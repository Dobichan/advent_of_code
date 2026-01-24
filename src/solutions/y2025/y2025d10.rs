use crate::{AoCSolution, iterators::GosperIterator};

const YEAR: u16 = 2025;
const DAY: u8 = 10;

pub struct Solution {}

#[derive(Debug)]
struct Machine {
    indicators: u32,
    buttons: Vec<u32>,
    _joltage_requirements: Vec<u16>,
}

impl Machine {
    fn get_min_btn_presses_leds(&self) -> u32 {
        let num_buttons = self.buttons.len();
        let index_numbers = GosperIterator::new((1u32 << num_buttons) - 1);
        // println!("\n{:?}", self);

        for button_mask in index_numbers {
            // println!("{:08b}", button_mask);
            let mut res = 0;
            for i in 0..num_buttons {
                // println!("{i}");
                if (button_mask & 1u32 << i) != 0 {
                    res ^= self.buttons[i];
                }
            }
            if res == self.indicators {
                return button_mask.count_ones();
            }
        }

        0
    }
}

fn parse(line: &str) -> Machine {
    let elements: Vec<_> = line.split(' ').collect();
    let indicators = elements[0];
    let joltage = elements[elements.len() - 1];

    Machine {
        indicators: indicators[1..indicators.len() - 1]
            .chars()
            .enumerate()
            .map(|(i, c)| if c == '#' { 1u32 << i } else { 0 })
            .sum(),
        buttons: elements[1..elements.len() - 1] // Skip first and last - indicators and joltage
            .iter()
            .map(|btn| {
                btn[1..btn.len() - 1] // Skip '(' and ')' - first and last
                    .split(',')
                    .map(|idx| {
                        let i: u8 = idx.parse().unwrap();
                        1u32 << i
                    })
                    .sum()
            })
            .collect(),
        _joltage_requirements: joltage[1..joltage.len() - 1] // skip '{' and '}' - first and last
            .split(',')
            .map(|jo| jo.parse::<u16>().unwrap())
            .collect(),
    }
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str, dryrun: bool) -> String {
        if dryrun {
            return 1234567890.to_string();
        }

        // let test: Vec<_> = input.trim().lines().map(|line| parse(line)).collect();
        // for t in test {
        //     println!("{:?}", t);
        // }
        let lines: Vec<_> = input.trim().lines().collect();

        lines
            .iter()
            .map(|line| parse(line))
            .map(|machine| machine.get_min_btn_presses_leds())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&mut self, _input: &str, dryrun: bool) -> String {
        if dryrun {
            return 1234567890.to_string();
        }
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
                [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
                [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let mut sol = Solution {};
        let answer = sol.part1(EXAMPLE_INPUT, false);

        assert_eq!(answer, "7");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            "#;

        let mut sol = Solution {};
        let answer = sol.part2(EXAMPLE_INPUT, false);

        assert_eq!(answer, "");
    }
}
