use std::{
    fmt::Display,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::AoCSolution;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    One,
    Two,
    Both,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            "a" | "all" => Ok(Part::Both),
            other => Err(format!("Illegal part {other:?}, use 1, 2 or a")),
        }
    }
}

impl Part {
    fn is_part1(self) -> bool {
        self == Part::One || self == Part::Both
    }

    fn is_part2(self) -> bool {
        self == Part::Two || self == Part::Both
    }
}

pub type Runner = fn(input: &str, part: Part);

pub fn run<S: AoCSolution>(solution: S, input: &str, part: Part) {
    let (data, elapsed) = timed(|| solution.parse(input));
    println!("Parse: in {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    if part.is_part1() {
        let (answer, elapsed) = timed(|| solution.part1(&data));
        report(1, answer, elapsed);
    }

    if part.is_part2() {
        let (answer, elapsed) = timed(|| solution.part2(&data));
        report(2, answer, elapsed);
    }
}

fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let ret_val = f();
    (ret_val, start.elapsed())
}

fn report(part: u8, answer: impl Display, elapsed: Duration) {
    println!(
        "Part {part}: {answer} - in {:.3} ms",
        elapsed.as_secs_f64() * 1000.0
    )
}

pub fn runner<S: AoCSolution + Default>() -> Runner {
    |input, part| run(S::default(), input, part)
}
