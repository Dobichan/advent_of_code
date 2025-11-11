use advent_of_code::parsing::*;

fn part1(input: &str) -> i64 {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

fn part2(input: &str) -> i64 {
    let mut lvl = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            lvl += 1;
        } else {
            lvl -= 1;
        }
        if lvl == -1 {
            return (i + 1) as i64;
        }
    }
    -1
}

fn main() {
    const YEAR: u16 = 2015;
    const DAY: u8 = 1;

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
        const INPUT1A: &str = r"(())";
        const INPUT1B: &str = r"()()";
        const INPUT2A: &str = r"(((";
        const INPUT2B: &str = r"(()(()(";
        const INPUT3: &str = r"))(((((";
        const INPUT4A: &str = r"())";
        const INPUT4B: &str = r"))(";
        const INPUT5A: &str = r")))";
        const INPUT5B: &str = r")())())";

        assert_eq!(part1(INPUT1A), 0);
        assert_eq!(part1(INPUT1B), 0);
        assert_eq!(part1(INPUT2A), 3);
        assert_eq!(part1(INPUT2B), 3);
        assert_eq!(part1(INPUT3), 3);
        assert_eq!(part1(INPUT4A), -1);
        assert_eq!(part1(INPUT4B), -1);
        assert_eq!(part1(INPUT5A), -3);
        assert_eq!(part1(INPUT5B), -3);
    }

    #[test]
    fn test_part2() {
        const INPUT1A: &str = r")";
        const INPUT1B: &str = r"()())";

        assert_eq!(part2(INPUT1A), 1);
        assert_eq!(part2(INPUT1B), 5);
    }
}
