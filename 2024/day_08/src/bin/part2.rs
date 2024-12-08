#[derive(Debug, PartialEq)]
struct Antenna {
    frequency: char,
    x: i32,
    y: i32,
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse(input: &str) -> (Vec<Antenna>, i32, i32) {
    let mut antennas: Vec<Antenna> = vec![];
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    for (r, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                let a = Antenna {
                    frequency: ch,
                    x: col as i32,
                    y: r as i32,
                };
                antennas.push(a);
            }
            cols = col as i32;
        }
        rows = r as i32;
    }

    (antennas, rows + 1, cols + 1)
}

fn part2(input: &str) -> i32 {
    let (antennas, rows, cols) = parse(input);

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _ in 0..rows {
        let mut g: Vec<i32> = Vec::new();
        for _ in 0..cols {
            g.push(0);
        }
        grid.push(g);
    }

    for ant in &antennas {
        // println!("Checking: {:?}", ant);

        for other in &antennas {
            if other.frequency == ant.frequency && other.x != ant.x && other.y != ant.y {
                // println!("  against: {:?}", other);

                grid[ant.y as usize][ant.x as usize] = 1;
                grid[other.y as usize][other.x as usize] = 1;

                let delta_x = other.x - ant.x;
                let delta_y = other.y - ant.y;

                let mut test_x = ant.x - delta_x;
                let mut test_y = ant.y - delta_y;

                while test_x >= 0 && test_x < cols && test_y >= 0 && test_y < rows {
                    // println!("      res @ {first_res_x}x{first_res_y}");
                    grid[test_y as usize][test_x as usize] = 1;
                    test_x -= delta_x;
                    test_y -= delta_y;
                }

                test_x = other.x + delta_x;
                test_y = other.y + delta_y;

                while test_x >= 0 && test_x < cols && test_y >= 0 && test_y < rows {
                    // println!("      res @ {second_res_x}x{second_res_y}");
                    grid[test_y as usize][test_x as usize] = 1;
                    test_x += delta_x;
                    test_y += delta_y;
                }
            }
        }
    }
    println!("{:?}", grid);
    grid.iter().flat_map(|r| r.iter()).sum::<i32>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "............
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
............";

        assert_eq!(part2(dummy), 34);
    }
}
