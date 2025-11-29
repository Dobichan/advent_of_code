use itertools::Itertools;

use crate::AoCSolution;

const YEAR: u16 = 2024;
const DAY: u8 = 10;
pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let grid = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();

        let mut sum = 0;

        for r in 0..rows {
            for c in 0..cols {
                sum += trailhead(&grid, r, c, true);
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();

        let mut sum = 0;

        for r in 0..rows {
            for c in 0..cols {
                sum += trailhead(&grid, r, c, false);
            }
        }
        sum.to_string()
    }
}

fn num_possible(grid: &Vec<Vec<char>>, num: char, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::with_capacity(10);
    let rows = grid.len();
    let cols = grid[0].len();

    if num != grid[r][c] {
        return ret;
    }
    // println!("Looking for {num} @ {r}x{c}");

    if num == '9' {
        // println!("9 @ {r}x{c}");
        ret.push((r, c));
    } else {
        // Check left for one bigger
        if c > 0 {
            let temp = num_possible(grid, (num as u8 + 1) as char, r, c - 1);
            for l in temp {
                ret.push(l);
            }
        }

        // Check above for one bigger
        if r > 0 {
            let temp = num_possible(grid, (num as u8 + 1) as char, r - 1, c);
            for l in temp {
                ret.push(l);
            }
        }

        // Check right for one bigger
        if c < cols - 1 {
            let temp = num_possible(grid, (num as u8 + 1) as char, r, c + 1);
            for l in temp {
                ret.push(l);
            }
        }

        // Check below for one bigger
        if r < rows - 1 {
            let temp = num_possible(grid, (num as u8 + 1) as char, r + 1, c);
            for l in temp {
                ret.push(l);
            }
        }
    }
    ret
}

fn trailhead(grid: &Vec<Vec<char>>, r: usize, c: usize, unique: bool) -> usize {
    if grid[r][c] == '0' {
        let temp = num_possible(grid, '0', r, c);
        // println!("found: {:?}", temp);
        if unique {
            return temp.into_iter().unique().count();
        } else {
            return temp.len();
        }
    }
    0
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut ret = Vec::with_capacity(100);
    for line in input.trim().lines() {
        let mut row = Vec::with_capacity(100);
        for ch in line.trim().chars() {
            row.push(ch);
        }
        ret.push(row);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        const TEST_GRID: &str = r#"
            9990999
            9991999
            9992999
            6543456
            7111117
            8111118
            9111119"#;

        assert_eq!(
            parse(&TEST_GRID),
            [
                ['9', '9', '9', '0', '9', '9', '9'],
                ['9', '9', '9', '1', '9', '9', '9'],
                ['9', '9', '9', '2', '9', '9', '9'],
                ['6', '5', '4', '3', '4', '5', '6'],
                ['7', '1', '1', '1', '1', '1', '7'],
                ['8', '1', '1', '1', '1', '1', '8'],
                ['9', '1', '1', '1', '1', '1', '9']
            ]
        )
    }
    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
            9990999
            9991999
            9992999
            6543456
            7111117
            8111118
            9111119"#;

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "2");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            9990999
            9991999
            9992999
            6543456
            7111117
            8111118
            9111119"#;

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "2");
    }
}
