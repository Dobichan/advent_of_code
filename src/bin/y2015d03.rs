use std::collections::HashMap;

use advent_of_code::parsing::*;

fn part1(input: &str) -> i64 {
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos = (0, 0);
    houses.insert(pos, 1);

    for dir in input.chars() {
        pos = match dir {
            '^' => (pos.0, pos.1 - 1),
            '>' => (pos.0 + 1, pos.1),
            'v' => (pos.0, pos.1 + 1),
            '<' => (pos.0 - 1, pos.1),
            _ => panic!("Illegal character in input!!!!"),
        };
        if !houses.contains_key(&pos) {
            houses.insert(pos, 1);
        }
    }
    houses.len() as i64
}

fn part2(input: &str) -> i64 {
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    houses.insert(santa_pos, 1);

    for (i, dir) in input.chars().enumerate() {
        let new_pos = match dir {
            '^' => {
                if i % 2 == 0 {
                    (santa_pos.0, santa_pos.1 - 1)
                } else {
                    (robo_santa_pos.0, robo_santa_pos.1 - 1)
                }
            }
            '>' => {
                if i % 2 == 0 {
                    (santa_pos.0 + 1, santa_pos.1)
                } else {
                    (robo_santa_pos.0 + 1, robo_santa_pos.1)
                }
            }
            'v' => {
                if i % 2 == 0 {
                    (santa_pos.0, santa_pos.1 + 1)
                } else {
                    (robo_santa_pos.0, robo_santa_pos.1 + 1)
                }
            }
            '<' => {
                if i % 2 == 0 {
                    (santa_pos.0 - 1, santa_pos.1)
                } else {
                    (robo_santa_pos.0 - 1, robo_santa_pos.1)
                }
            }
            _ => panic!("Illegal character in input!!!!"),
        };
        if !houses.contains_key(&new_pos) {
            houses.insert(new_pos, 1);
        }
        if i % 2 == 0 {
            santa_pos = new_pos;
        } else {
            robo_santa_pos = new_pos;
        }
    }
    houses.len() as i64
}

fn main() {
    const YEAR: u16 = 2015;
    const DAY: u8 = 3;

    let input = read_input(YEAR, DAY);

    let start = std::time::Instant::now();
    let answer1 = part1(&input);
    let end = std::time::Instant::now();

    println!("");
    println!("Answer part 1: {answer1}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");

    let start = std::time::Instant::now();
    let answer2 = part2(&input);
    let end = std::time::Instant::now();

    println!("Answer part 2: {answer2}");
    println!("Elapsed: {:.3} ms", (end - start).as_secs_f64() * 1000.0);
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT1: &str = ">";
        assert_eq!(part1(INPUT1), 2);

        const INPUT2: &str = "^>v<";
        assert_eq!(part1(INPUT2), 4);

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(part1(INPUT3), 2);
    }

    #[test]
    fn test_part2() {
        const INPUT1: &str = "^v";
        assert_eq!(part2(INPUT1), 3);

        const INPUT2: &str = "^>v<";
        assert_eq!(part2(INPUT2), 3);

        const INPUT3: &str = "^v^v^v^v^v";
        assert_eq!(part2(INPUT3), 11);
    }
}
