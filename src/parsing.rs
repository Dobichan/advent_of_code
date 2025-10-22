use std::fs;
use std::path::Path;

pub fn read_input(year: u16, day: u8) -> String {
    let filename = format!("day{:02}.txt", day);
    let path = Path::new("input").join(year.to_string()).join(filename);

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {:?}", path))
}

pub fn read_input_lines(year: u16, day: u8) -> Vec<String> {
    read_input(year, day)
        .lines()
        .map(|s| s.to_string())
        .collect()
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

#[cfg(test)]
mod tests {
    use crate::parsing::numbers_to_pair;

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
}
