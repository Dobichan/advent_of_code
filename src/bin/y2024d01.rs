use advent_of_code::parsing::*;

fn input_to_vectors(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    lines.iter().map(|line| numbers_to_pair(line)).unzip()
}

fn part1(lines: &Vec<String>) -> i64 {
    let (mut list1, mut list2) = input_to_vectors(lines);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    let (mut list1, mut list2) = input_to_vectors(lines);

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for &num in &list1 {
        let count = list2.iter().filter(|&&x| x == num).count() as i64;
        sum += num * count;
    }

    sum
}

fn main() {
    const YEAR: u16 = 2024;
    const DAY: u8 = 01;

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
        const EXAMPLE_INPUT: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#;

        let (firsts, seconds) = input_to_vectors(
            &EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        );

        assert_eq!(firsts, [3, 4, 2, 1, 3, 3]);
        assert_eq!(seconds, [4, 3, 5, 3, 9, 3]);

        let answer = part1(
            &EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        );

        assert_eq!(answer, 11);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#;

        let answer = part2(
            &EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        );

        assert_eq!(answer, 31);
    }
}
