use crate::AoCSolution;
use crate::grid::*;

const YEAR: u16 = 2024;
const DAY: u8 = 4;

enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
];

const MAS: [char; 3] = ['M', 'A', 'S'];

fn check_xmas(grid: &Grid, dir: Direction, x: usize, y: usize) -> bool {
    for (i, c) in MAS.iter().enumerate() {
        match dir {
            Direction::UpLeft => {
                if x >= MAS.len() && y >= MAS.len() {
                    if grid[y - i - 1][x - i - 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::Up => {
                if y >= MAS.len() {
                    if grid[y - i - 1][x] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::UpRight => {
                if y >= MAS.len() && x <= grid.width() - MAS.len() {
                    if grid[y - i - 1][x + i + 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::Right => {
                if x < grid.width() - MAS.len() {
                    if grid[y][x + i + 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::DownRight => {
                if y < grid.height() - MAS.len() && x <= grid.width() - MAS.len() {
                    if grid[y + i + 1][x + i + 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::Down => {
                if y < grid.height() - MAS.len() {
                    if grid[y + i + 1][x] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::DownLeft => {
                if y < grid.height() - MAS.len() && x >= MAS.len() {
                    if grid[y + i + 1][x - i - 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            Direction::Left => {
                if x >= MAS.len() {
                    if grid[y][x - i - 1] != *c {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn check_mas(grid: &Grid, x: usize, y: usize) -> bool {
    if x > 0 && x < grid.width() - 1 && y > 0 && y < grid.height() - 1 {
        if (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
            || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
        {
            if (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
                || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
            {
                return true;
            }
        }
    }
    false
}

pub struct Solution {}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }

    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> String {
        let mut ret = 0;
        let grid: Grid = input.parse().expect("Failed to parse grid");
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[y][x] == 'X' {
                    for dir in DIRECTIONS {
                        if check_xmas(&grid, dir, x, y) {
                            ret += 1;
                        }
                    }
                }
            }
        }
        ret.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut ret = 0;
        let grid: Grid = input.parse().expect("Failed to parse grid");

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[y][x] == 'A' {
                    if check_mas(&grid, x, y) {
                        ret += 1;
                    }
                }
            }
        }
        ret.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

        let sol = Solution {};
        assert_eq!(sol.part1(EXAMPLE_INPUT), "18");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

        let sol = Solution {};
        assert_eq!(sol.part2(EXAMPLE_INPUT), "9");
    }
}
