use std::fs;
use std::path::Path;

pub fn read_input(input_file: &str) -> String {
    let path = Path::new(input_file);

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {:?}", path))
}

pub fn read_input_lines(input_file: &str) -> Vec<String> {
    read_input(input_file)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn input_to_vectors(input: &str) -> Vec<String> {
    input.trim().lines().map(|s| s.to_string()).collect()
}

pub fn numbers_to_pair(line: &str) -> (i64, i64) {
    let mut splits = line.split_whitespace();
    (
        splits
            .next()
            .expect("No elements")
            .parse()
            .expect("First element not a number"),
        splits
            .next()
            .expect("Second element missing")
            .parse()
            .expect("Second element is not a number"),
    )
}

pub fn line_numbers_to_vec(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|elem| elem.parse::<i64>().expect("Not a number in list: {elem}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parsing::{line_numbers_to_vec, numbers_to_pair};

    #[test]
    fn test_number_to_pair() {
        let dummy: &str = "3 4";
        let (a, b) = numbers_to_pair(dummy);
        assert_eq!(a, 3);
        assert_eq!(b, 4);
    }

    #[test]
    fn test_number_to_pair_failures() {
        // Test that numbers_to_pair panics when input is empty
        let result = std::panic::catch_unwind(|| {
            numbers_to_pair("");
        });
        assert!(result.is_err());

        // Test that numbers_to_pair panics when only one number is present
        let result = std::panic::catch_unwind(|| {
            numbers_to_pair("42");
        });
        assert!(result.is_err());

        // Test that numbers_to_pair panics when non-numeric input is present
        let result = std::panic::catch_unwind(|| {
            numbers_to_pair("a b");
        });
        assert!(result.is_err());

        // Test that numbers_to_pair works with negative numbers
        let (a, b) = numbers_to_pair("-5 10");
        assert_eq!(a, -5);
        assert_eq!(b, 10);

        // Test that numbers_to_pair works with extra whitespace
        let (a, b) = numbers_to_pair("  7    8  ");
        assert_eq!(a, 7);
        assert_eq!(b, 8);
    }

    #[test]
    fn test_numbers_to_vec() {
        let test = line_numbers_to_vec("1 2 3 4 5 6 7");

        assert_eq!(test, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_numbers_to_vec_failures() {
        // Test that line_numbers_to_vec panics when input contains non-numeric values
        let result = std::panic::catch_unwind(|| {
            line_numbers_to_vec("1 2 a 4");
        });
        assert!(result.is_err());

        // Test that line_numbers_to_vec panics when input contains empty string
        let result = std::panic::catch_unwind(|| {
            line_numbers_to_vec("");
        });

        // This should not panic, it should return an empty Vec
        assert!(result.is_ok());
        assert_eq!(line_numbers_to_vec(""), Vec::<i64>::new());

        // Test that line_numbers_to_vec works with negative numbers
        let test = line_numbers_to_vec("-1 -2 3");
        assert_eq!(test, [-1, -2, 3]);

        // Test that line_numbers_to_vec works with extra whitespace
        let test = line_numbers_to_vec("   4   5  6 ");
        assert_eq!(test, [4, 5, 6]);

        // Test that line_numbers_to_vec panics when input contains floating point numbers
        let result = std::panic::catch_unwind(|| {
            line_numbers_to_vec("1 2.5 3");
        });
        assert!(result.is_err());
    }
}
