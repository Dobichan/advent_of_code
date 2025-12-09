use std::collections::HashMap;

use multimap::MultiMap;

use crate::AoCSolution;

const YEAR: u16 = 2024;
const DAY: u8 = 8;
pub struct Solution {}

fn parse(input: &str) -> (MultiMap<char, (i32, i32)>, (i32, i32)) {
    let mut ret = MultiMap::new();
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    for (row, line) in input.lines().enumerate() {
        width = line.trim().len() as i32;
        height += 1;
        for (col, ch) in line.trim().chars().enumerate() {
            if ch != '.' {
                ret.insert(ch, (col as i32, row as i32));
            }
        }
    }
    (ret, (width, height))
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let (grid, (width, height)) = parse(input);
        let mut result = HashMap::new();

        // println!("{width}x{height}");

        for k in grid.keys() {
            let freq_nodes = grid.get_vec(k).unwrap();
            let n = freq_nodes.len();

            // println!("Processing {k}");

            for i in 0..n {
                for j in i + 1..n {
                    let (x1, y1) = freq_nodes[i];
                    let (x2, y2) = freq_nodes[j];

                    // Resonance 1
                    let r_x = x1 - x2 + x1;
                    let r_y = y1 - y2 + y1;

                    if r_x >= 0 && r_x < width && r_y >= 0 && r_y < height {
                        // println!("R1: ({x1},{y1}) - ({x2},{y2}) -> ({r_x},{r_y})");
                        result.insert((r_x, r_y), 1);
                    }

                    // Resonance 2
                    let r_x = x2 - x1 + x2;
                    let r_y = y2 - y1 + y2;

                    if r_x >= 0 && r_x < width && r_y >= 0 && r_y < height {
                        // println!("R2: ({x1},{y1}) - ({x2},{y2}) -> ({r_x},{r_y})");
                        result.insert((r_x, r_y), 1);
                    }
                }
            }
        }

        result.keys().count().to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let (grid, (width, height)) = parse(input);
        let mut result = HashMap::new();

        // println!("{width}x{height}");

        for k in grid.keys() {
            let freq_nodes = grid.get_vec(k).unwrap();
            let n = freq_nodes.len();

            // println!("Processing {k}");

            for i in 0..n {
                for j in i + 1..n {
                    let (x1, y1) = freq_nodes[i];
                    let (x2, y2) = freq_nodes[j];

                    let delta_x = x2 - x1;
                    let delta_y = y2 - y1;

                    // Resonance 1
                    let mut r_x = x1;
                    let mut r_y = y1;

                    while r_x >= 0 && r_x < width && r_y >= 0 && r_y < height {
                        // println!("R1: ({x1},{y1}) - ({x2},{y2}) -> ({r_x},{r_y})");
                        result.insert((r_x, r_y), 1);
                        r_x -= delta_x;
                        r_y -= delta_y;
                    }

                    // Resonance 2
                    let mut r_x = x2;
                    let mut r_y = y2;

                    while r_x >= 0 && r_x < width && r_y >= 0 && r_y < height {
                        // println!("R2: ({x1},{y1}) - ({x2},{y2}) -> ({r_x},{r_y})");
                        result.insert((r_x, r_y), 1);
                        r_x += delta_x;
                        r_y += delta_y;
                    }
                }
            }
        }

        result.keys().count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = r#"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            "#;

        let mut sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT.trim());

        assert_eq!(answer, "14");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            "#;

        let mut sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT.trim());

        assert_eq!(answer, "34");
    }
}
