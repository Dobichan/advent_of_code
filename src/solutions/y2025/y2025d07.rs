use std::collections::HashMap;

use crate::{AoCSolution, grid::Grid};

const YEAR: u16 = 2025;
const DAY: u8 = 7;

pub struct Solution {}

pub fn process_timelines(
    grid: &Grid,
    visited: &mut HashMap<(usize, usize), u64>,
    row: usize,
    col: usize,
) -> u64 {
    if row == grid.height() - 1 {
        return 1;
    }
    if grid[row][col] == '^' {
        if let Some(number) = visited.get(&(row, col)) {
            return *number;
        }
        let mut ret = 0;
        if col > 0 {
            ret += process_timelines(grid, visited, row + 1, col - 1);
        }
        if col < grid.width() - 1 {
            ret += process_timelines(grid, visited, row + 1, col + 1);
        }
        visited.insert((row, col), ret);
        return ret;
    }
    return process_timelines(grid, visited, row + 1, col);
}

pub fn process_beams(grid: &mut Grid, row: usize, beam_col: usize) {
    grid[row + 1][beam_col] = '|';
    for r in row + 1..grid.height() {
        for c in 0..grid.width() {
            if grid[r][c] == '^' && grid[r - 1][c] == '|' {
                if c > 0 {
                    grid[r][c - 1] = '|';
                }
                if c < grid.width() - 2 {
                    grid[r][c + 1] = '|';
                }
            }
            if grid[r][c] == '.' && grid[r - 1][c] == '|' {
                grid[r][c] = '|';
            }
        }
    }
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut grid: Grid = input.trim().parse().expect("Illegal input grid");
        let start_col = grid[0]
            .iter()
            .enumerate()
            .find(|(_i, c)| **c == 'S')
            .unwrap()
            .0;

        process_beams(&mut grid, 0, start_col);

        let mut ret = 0;

        for (pos, ch) in grid.into_iter() {
            if *ch == '^' && grid[pos.1 - 1][pos.0] == '|' {
                ret += 1;
            }
        }

        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid: Grid = input.trim().parse().expect("Illegal input grid");
        let start_col = grid[0]
            .iter()
            .enumerate()
            .find(|(_i, c)| **c == 'S')
            .unwrap()
            .0;

        let mut visited = HashMap::with_capacity(100000);

        process_timelines(&mut grid, &mut visited, 1, start_col).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "\n\
              .......S.......\n\
              ...............\n\
              .......^.......\n\
              ...............\n\
              ......^.^......\n\
              ...............\n\
              .....^.^.^.....\n\
              ...............\n\
              ....^.^...^....\n\
              ...............\n\
              ...^.^...^.^...\n\
              ...............\n\
              ..^...^.....^..\n\
              ...............\n\
              .^.^.^.^.^...^.\n\
              ...............";

        let sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "21");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "\n\
              .......S.......\n\
              ...............\n\
              .......^.......\n\
              ...............\n\
              ......^.^......\n\
              ...............\n\
              .....^.^.^.....\n\
              ...............\n\
              ....^.^...^....\n\
              ...............\n\
              ...^.^...^.^...\n\
              ...............\n\
              ..^...^.....^..\n\
              ...............\n\
              .^.^.^.^.^...^.\n\
              ...............";

        let sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "40");
    }
}
