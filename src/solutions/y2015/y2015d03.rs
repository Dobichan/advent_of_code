use crate::AoCSolution;
use std::collections::HashMap;

const YEAR: u16 = 2015;
const DAY: u8 = 3;
pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
        let mut pos = (0, 0);
        houses.insert(pos, 1);

        for dir in input.chars() {
            pos = match dir {
                '^' => (pos.0, pos.1 - 1),
                '>' => (pos.0 + 1, pos.1),
                'v' => (pos.0, pos.1 + 1),
                '<' => (pos.0 - 1, pos.1),
                _ => panic!("Illegal character in input!!!!"),
            };
            if !houses.contains_key(&pos) {
                houses.insert(pos, 1);
            }
        }
        houses.len().to_string()
    }
    fn part2(&mut self, input: &str) -> String {
        let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
        let mut santa_pos = (0, 0);
        let mut robo_santa_pos = (0, 0);
        houses.insert(santa_pos, 1);

        for (i, dir) in input.chars().enumerate() {
            let new_pos = match dir {
                '^' => {
                    if i % 2 == 0 {
                        (santa_pos.0, santa_pos.1 - 1)
                    } else {
                        (robo_santa_pos.0, robo_santa_pos.1 - 1)
                    }
                }
                '>' => {
                    if i % 2 == 0 {
                        (santa_pos.0 + 1, santa_pos.1)
                    } else {
                        (robo_santa_pos.0 + 1, robo_santa_pos.1)
                    }
                }
                'v' => {
                    if i % 2 == 0 {
                        (santa_pos.0, santa_pos.1 + 1)
                    } else {
                        (robo_santa_pos.0, robo_santa_pos.1 + 1)
                    }
                }
                '<' => {
                    if i % 2 == 0 {
                        (santa_pos.0 - 1, santa_pos.1)
                    } else {
                        (robo_santa_pos.0 - 1, robo_santa_pos.1)
                    }
                }
                _ => panic!("Illegal character in input!!!!"),
            };
            if !houses.contains_key(&new_pos) {
                houses.insert(new_pos, 1);
            }
            if i % 2 == 0 {
                santa_pos = new_pos;
            } else {
                robo_santa_pos = new_pos;
            }
        }
        houses.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut sol = Solution {};

        const INPUT1: &str = ">";
        assert_eq!(sol.part1(INPUT1), "2");

        const INPUT2: &str = "^>v<";
        assert_eq!(sol.part1(INPUT2), "4");

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(sol.part1(INPUT3), "2");
    }

    #[test]
    fn test_part2() {
        let mut sol = Solution {};

        const INPUT1: &str = "^v";
        assert_eq!(sol.part2(INPUT1), "3");

        const INPUT2: &str = "^>v<";
        assert_eq!(sol.part2(INPUT2), "3");

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(sol.part2(INPUT3), "11");
    }
}
