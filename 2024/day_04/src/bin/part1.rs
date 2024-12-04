use std::result;

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input, "XMAS");
    dbg!(output);
}

#[derive(Debug)]
enum hit_direction {
    forward(usize, usize),
    reverse(usize, usize),
    up_left(usize, usize),
    up(usize, usize),
    up_right(usize, usize),
    down_left(usize, usize),
    down(usize, usize),
    down_right(usize, usize),
}

fn part1(input: &str, word: &str) -> i32 {
    let mut results = vec![];
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect())
    }
    let rows = grid.len();
    let cols = grid[0].len();
    println!("grid: {rows}x{cols} - word_len: {:?}", word.len());

    for row in 0..rows {
        println!("{:?}", grid[row]);
        for col in 0..cols {
            // Check forward
            let mut found = true;
            if col <= (cols - word.len()) {
                for (j, c) in word.chars().enumerate() {
                    if grid[row][col + j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::forward(row, col));
            }

            // Check reverse
            found = true;
            if col >= word.len() - 1 {
                for (j, c) in word.chars().enumerate() {
                    if grid[row][col - j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::reverse(row, col));
            }

            // Check up_left
            found = true;
            if col >= word.len() - 1 && row >= word.len() - 1 {
                for (j, c) in word.chars().enumerate() {
                    if grid[row - j][col - j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::up_left(row, col));
            }

            // Check up
            found = true;
            if row >= word.len() - 1 {
                for (j, c) in word.chars().enumerate() {
                    if grid[row - j][col] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::up(row, col));
            }

            // Check up_right
            found = true;
            if col <= (cols - word.len()) && row >= word.len() - 1 {
                for (j, c) in word.chars().enumerate() {
                    if grid[row - j][col + j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::up_right(row, col));
            }

            // Check down_left
            found = true;
            if col >= word.len() - 1 && row <= (rows - word.len()) {
                for (j, c) in word.chars().enumerate() {
                    if grid[row + j][col - j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::down_left(row, col));
            }

            // Check down
            found = true;
            if row <= (rows - word.len()) {
                for (j, c) in word.chars().enumerate() {
                    if grid[row + j][col] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::down(row, col));
            }

            // Check down_right
            found = true;
            if col <= (cols - word.len()) && row <= (rows - word.len()) {
                for (j, c) in word.chars().enumerate() {
                    if grid[row + j][col + j] != c {
                        found = false;
                        break;
                    }
                }
            } else {
                found = false;
            }
            if found {
                results.push(hit_direction::down_right(row, col));
            }
        }
    }

    println!("{:?}", results);
    results.len() as i32
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

        assert_eq!(part1(dummy, "XMAS"), 18);
    }
}
