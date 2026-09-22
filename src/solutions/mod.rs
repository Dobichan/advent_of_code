use crate::runner::{Runner, runner};

pub mod y2015;
pub mod y2024;
pub mod y2025;

pub fn lookup(year: u16, day: u8) -> Option<Runner> {
    Some(match (year, day) {
        (2015, 1) => runner::<y2015::day01::Solution>(),
        (2015, 2) => runner::<y2015::day02::Solution>(),
        (2015, 3) => runner::<y2015::day03::Solution>(),
        (2015, 4) => runner::<y2015::day04::Solution>(),
        (2015, 5) => runner::<y2015::day05::Solution>(),

        (2024, 1) => runner::<y2024::day01::Solution>(),
        (2024, 2) => runner::<y2024::day02::Solution>(),
        (2024, 3) => runner::<y2024::day03::Solution>(),
        (2024, 4) => runner::<y2024::day04::Solution>(),
        (2024, 5) => runner::<y2024::day05::Solution>(),
        (2024, 6) => runner::<y2024::day06::Solution>(),
        (2024, 7) => runner::<y2024::day07::Solution>(),
        (2024, 8) => runner::<y2024::day08::Solution>(),
        (2024, 9) => runner::<y2024::day09::Solution>(),
        (2024, 10) => runner::<y2024::day10::Solution>(),

        (2025, 1) => runner::<y2025::day01::Solution>(),
        (2025, 2) => runner::<y2025::day02::Solution>(),
        (2025, 3) => runner::<y2025::day03::Solution>(),
        (2025, 4) => runner::<y2025::day04::Solution>(),
        (2025, 5) => runner::<y2025::day05::Solution>(),
        (2025, 6) => runner::<y2025::day06::Solution>(),
        (2025, 7) => runner::<y2025::day07::Solution>(),
        (2025, 8) => runner::<y2025::day08::Solution>(),
        (2025, 9) => runner::<y2025::day09::Solution>(),
        (2025, 10) => runner::<y2025::day10::Solution>(),
        _ => return None,
    })
}
