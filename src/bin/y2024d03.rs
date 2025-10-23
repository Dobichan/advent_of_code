use advent_of_code::parsing::*;
use regex::Regex;

fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;

    for cap in re.captures_iter(input) {
        let a: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
        let b: i64 = cap.get(2).unwrap().as_str().parse().unwrap();

        result += a * b;
    }
    result
}

fn part2(input: &str) -> i64 {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut result = 0;
    let mut enabled = true;

    for cap in re.captures_iter(input) {
        let instruction = cap.get(0).unwrap().as_str();

        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let a: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
                let b: i64 = cap.get(2).unwrap().as_str().parse().unwrap();

                result += a * b;
            }
        }
    }
    result
}

fn main() {
    const YEAR: u16 = 2024;
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
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "#;

        let answer = part1(EXAMPLE_INPUT.trim());
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "#;

        let answer = part2(EXAMPLE_INPUT.trim());
        assert_eq!(answer, 48);
    }
}
