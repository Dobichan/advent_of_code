use std::usize;

use crate::{AoCSolution, grid::Grid};

const YEAR: u16 = 2025;
const DAY: u8 = 4;
pub struct Solution {}

pub fn removeable_paper_roll(grid: &Grid, row: usize, col: usize) -> bool {
    if grid[row][col] == '@' {
        let mut others = 0;

        if row > 0 {
            if col > 0 && grid[row - 1][col - 1] == '@' {
                others += 1;
            }

            if grid[row - 1][col] == '@' {
                others += 1;
            }

            if col < grid.width() - 1 && grid[row - 1][col + 1] == '@' {
                others += 1;
            }
        }
        if col > 0 {
            if grid[row][col - 1] == '@' {
                others += 1;
            }
        }
        if col < grid.width() - 1 {
            if grid[row][col + 1] == '@' {
                others += 1;
            }
        }
        if row < grid.height() - 1 {
            if col > 0 && grid[row + 1][col - 1] == '@' {
                others += 1;
            }
            if grid[row + 1][col] == '@' {
                others += 1;
            }

            if col < grid.width() - 1 && grid[row + 1][col + 1] == '@' {
                others += 1;
            }
        }
        if others < 4 {
            return true;
        }
    }
    false
}

pub fn remove_rolls(grid: &mut Grid) -> usize {
    let mut removed = 0;

    for r in 0..grid.height() {
        for c in 0..grid.width() {
            if removeable_paper_roll(grid, r, c) {
                removed += 1;
                grid[r][c] = 'x';
            }
        }
    }
    removed
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let grid: Grid = input.parse().expect("Faild to parse input grid");
        let mut ret = 0;

        for r in 0..grid.height() {
            for c in 0..grid.width() {
                if removeable_paper_roll(&grid, r, c) {
                    ret += 1;
                }
            }
        }
        ret.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut grid: Grid = input.parse().expect("Failed to parse grid");
        let mut ret = 0;

        loop {
            let this_round_removed = remove_rolls(&mut grid);
            if this_round_removed == 0 {
                break;
            }
            ret += this_round_removed;
            // println!("{grid}");
        }

        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "..@@.@@@@.\n\
                @@@.@.@.@@\n\
                @@@@@.@.@@\n\
                @.@@@@..@.\n\
                @@.@@@@.@@\n\
                .@@@@@@@.@\n\
                .@.@.@.@@@\n\
                @.@@@.@@@@\n\
                .@@@@@@@@.\n\
                @.@.@@@.@.";

        let mut sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "13");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "..@@.@@@@.\n\
                @@@.@.@.@@\n\
                @@@@@.@.@@\n\
                @.@@@@..@.\n\
                @@.@@@@.@@\n\
                .@@@@@@@.@\n\
                .@.@.@.@@@\n\
                @.@@@.@@@@\n\
                .@@@@@@@@.\n\
                @.@.@@@.@.";

        let mut sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "43");
    }
}
