use std::result;

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    let word = "MAS";
    let mut count = 0;

    for line in input.lines() {
        grid.push(line.chars().collect())
    }
    let rows = grid.len();
    let cols = grid[0].len();
    println!("grid: {rows}x{cols}");

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            // Check for the middle char
            if grid[row][col] == word.chars().nth(1).unwrap() {
                // Check right diag matches
                if (grid[row - 1][col - 1] == word.chars().nth(0).unwrap()
                    && grid[row + 1][col + 1] == word.chars().nth(2).unwrap())
                    || (grid[row - 1][col - 1] == word.chars().nth(2).unwrap()
                        && grid[row + 1][col + 1] == word.chars().nth(0).unwrap())
                {
                    // Check left diag matches
                    if (grid[row - 1][col + 1] == word.chars().nth(0).unwrap()
                        && grid[row + 1][col - 1] == word.chars().nth(2).unwrap())
                        || (grid[row - 1][col + 1] == word.chars().nth(2).unwrap()
                            && grid[row + 1][col - 1] == word.chars().nth(0).unwrap())
                    {
                        // println!("{row}.{col}");
                        // println!("{:?} {:?}", grid[row - 1][col - 1], grid[row - 1][col + 1]);
                        // println!("  {:?}", grid[row][col]);
                        // println!("{:?} {:?}", grid[row + 1][col - 1], grid[row + 1][col + 1]);
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part2(dummy), 9);
    }
}
