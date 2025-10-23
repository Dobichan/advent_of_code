use advent_of_code::parsing::*;

fn is_valid_pair(a: &i64, b: &i64, ascending: &bool) -> bool {
    let diff = b - a;
    if *ascending && diff > 0 && diff <= 3 {
        return true;
    }
    if !*ascending && diff < 0 && diff >= -3 {
        return true;
    }
    false
}

fn is_valid_sequence(numbers: &[i64], ascending: &bool) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    numbers
        .windows(2)
        .all(|w| is_valid_pair(&w[0], &w[1], ascending))
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut count = 0;

    for line in lines {
        let nums = line_numbers_to_vec(&line);
        let ascending = nums[1] > nums[0];

        if is_valid_sequence(&nums, &ascending) {
            count += 1;
        }
    }
    count
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut count = 0;

    for line in lines {
        let nums = line_numbers_to_vec(&line);
        let length = nums.len();

        if is_valid_sequence(&nums, &(nums[1] > nums[0])) {
            count += 1;
            continue;
        }

        for i in 0..length {
            let mut pruned_list = nums.clone();
            pruned_list.remove(i);
            if is_valid_sequence(&pruned_list, &(pruned_list[1] > pruned_list[0])) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn main() {
    const YEAR: u16 = 2024;
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
        const EXAMPLE_INPUT: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#;

        let answer = part1(
            &EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        );

        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#;

        let answer = part2(
            &EXAMPLE_INPUT
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        );

        assert_eq!(answer, 4);
    }
}
