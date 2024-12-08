#[derive(Debug, PartialEq)]
struct Antenna {
    frequency: char,
    x: i32,
    y: i32,
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
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

fn part1(input: &str) -> i32 {
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
        println!("Checking: {:?}", ant);

        for other in &antennas {
            if other.frequency == ant.frequency && other.x != ant.x && other.y != ant.y {
                // println!("  against: {:?}", other);

                // let delta_x = other.x - ant.x;
                // let delta_y = other.y - ant.y;

                let first_res_x = ant.x - other.x + ant.x;
                let first_res_y = ant.y - other.y + ant.y;
                let second_res_x = other.x + other.x - ant.x;
                let second_res_y = other.y + other.y - ant.y;

                if first_res_x >= 0 && first_res_x < rows && first_res_y >= 0 && first_res_y < cols
                {
                    // println!("      res @ {first_res_x}x{first_res_y}");
                    grid[first_res_y as usize][first_res_x as usize] = 1;
                }
                if second_res_x >= 0
                    && second_res_x < rows
                    && second_res_y >= 0
                    && second_res_y < cols
                {
                    // println!("      res @ {second_res_x}x{second_res_y}");
                    grid[second_res_y as usize][second_res_x as usize] = 1;
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

        assert_eq!(part1(dummy), 14);
    }
}
