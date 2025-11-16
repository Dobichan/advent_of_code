use advent_of_code::parsing::*;
use md5::Digest;
use md5::Md5;

fn part1(input: &str) -> i64 {
    let mut done = false;
    let mut append = 0;

    while !done {
        let check = format!("{}{}", input, append);
        let md5 = format!("{:x}", Md5::digest(check.as_bytes()));
        if md5.starts_with("00000") {
            done = true;
        } else {
            append += 1;
        }
    }

    append
}

fn part2(input: &str) -> i64 {
    let mut done = false;
    let mut append = 0;

    while !done {
        let check = format!("{}{}", input, append);
        let md5 = format!("{:x}", Md5::digest(check.as_bytes()));
        if md5.starts_with("000000") {
            done = true;
        } else {
            append += 1;
        }
    }

    append
}

fn main() {
    const YEAR: u16 = 2015;
    const DAY: u8 = 4;

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
        const INPUT1: &str = "abcdef";
        assert_eq!(part1(INPUT1), 609043);
    }

    #[test]
    fn test_part2() {}
}
