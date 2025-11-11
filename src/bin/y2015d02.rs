use std::cmp::min;

use advent_of_code::parsing::*;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::sequence::separated_pair;

fn get_dimmentions(input: &str) -> IResult<&str, (i64, i64, i64)> {
    let inner_parser = separated_pair(i64, tag("x"), i64);
    separated_pair(i64, tag("x"), inner_parser)
        .map(|(l, (w, h))| (l, w, h))
        .parse(input)
    // Ok((input, (0, 0, 0)))
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    for line in lines {
        let (_, (l, w, h)) = get_dimmentions(line).unwrap();
        let area_1 = l * w;
        let area_2 = w * h;
        let area_3 = h * l;

        let extra = min(area_1, min(area_2, area_3));
        total += 2 * area_1 + 2 * area_2 + 2 * area_3 + extra;
    }
    total
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    for line in lines {
        let (_, (l, w, h)) = get_dimmentions(line).unwrap();
        let mut dimmentions: Vec<_> = vec![l, w, h];
        dimmentions.sort();

        let extra = l * w * h;
        total += 2 * dimmentions[0] + 2 * dimmentions[1] + extra;
    }
    total
}

fn main() {
    const YEAR: u16 = 2015;
    const DAY: u8 = 2;

    let input = read_input_lines(YEAR, DAY);

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
        const INPUT1: &str = r"2x3x4";
        assert_eq!(
            part1(&INPUT1.trim().lines().map(|l| l.to_string()).collect()),
            58
        );

        const INPUT2: &str = r"1x1x10";
        assert_eq!(
            part1(&INPUT2.trim().lines().map(|l| l.to_string()).collect()),
            43
        );
    }

    #[test]
    fn test_part2() {
        const INPUT1: &str = r"2x3x4";
        assert_eq!(
            part2(&INPUT1.trim().lines().map(|l| l.to_string()).collect()),
            34
        );

        const INPUT2: &str = r"1x1x10";
        assert_eq!(
            part2(&INPUT2.trim().lines().map(|l| l.to_string()).collect()),
            14
        );
    }
}
