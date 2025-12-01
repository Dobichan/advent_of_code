use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 1;
pub struct Solution {}

fn rotate(pos: i32, instruction: &str) -> (i32, i32) {
    let mut data = instruction.chars();
    let dir = data.next().unwrap();
    let steps: i32 = data
        .as_str()
        .trim()
        .parse::<i32>()
        .expect("Failed to parse steps");
    let mut passed_0 = 0;
    let mut new_pos = match dir {
        'L' => {
            if pos == 0 {
                pos + 100 - steps
            } else {
                pos - steps
            }
        }
        'R' => pos + steps,
        _ => panic!("illegal input: {instruction}"),
    };
    while new_pos < 0 {
        new_pos += 100;
        passed_0 += 1;
    }
    while new_pos > 100 {
        new_pos -= 100;
        passed_0 += 1;
    }
    (new_pos % 100, passed_0)
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut pos = 50;
        let mut ret = 0;
        for line in input.trim().lines() {
            (pos, _) = rotate(pos, line);
            if pos == 0 {
                ret += 1;
            }
        }
        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut pos = 50;
        let mut ret = 0;

        for line in input.trim().lines() {
            let (new_pos, passed_0) = rotate(pos, line);
            ret += passed_0;
            if new_pos == 0 {
                ret += 1;
            }
            pos = new_pos;
        }
        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "3");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "6");
    }
}
