use std::path::{Path, PathBuf};

use advent_of_code::{parsing, runner::Part, solutions};

const USAGE: &str = "\
Uage: advent_of_code <YEAR> <DAY> [PART] [FILE] [-d|--dry-run]
   PART  1 | 2 | a  (default: a)
   FILE  input file (default: input/<YEAR>/day<DAY>.txt)";

struct Args {
    year: u16,
    day: u8,
    part: Part,
    file: PathBuf,
    dryrun: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut positional = Vec::new();
    let mut dryrun = false;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-d" | "--dry-run" => dryrun = true,
            "-h" | "--help" => return Err(USAGE.to_string()),
            _ if arg.starts_with('-') => return Err(format!("Unknown flag {arg}\n{USAGE}")),
            _ => positional.push(arg),
        }
    }

    let [year, day, rest @ ..] = positional.as_slice() else {
        return Err(USAGE.to_string());
    };

    if rest.len() > 2 {
        return Err(USAGE.to_string());
    }

    let year: u16 = year.parse().map_err(|_| format!("Invalid year {year:?}"))?;
    let day: u8 = day.parse().map_err(|_| format!("Invalid day {day:?}"))?;
    let part = match rest.first() {
        Some(p) => p.parse()?,
        None => Part::Both,
    };
    let file = match rest.get(1) {
        Some(f) => PathBuf::from(f),
        None => default_filename(year, day),
    };

    Ok(Args {
        year,
        day,
        part,
        file,
        dryrun,
    })
}

fn default_filename(year: u16, day: u8) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(year.to_string())
        .join(format!("day{day:02}.txt"))
}

fn main() {
    let Args {
        year,
        day,
        part,
        file,
        dryrun,
    } = parse_args().unwrap_or_else(|msg| {
        eprintln!("{msg}");
        std::process::exit(2);
    });

    let input = parsing::read_input(&file);

    let Some(run) = solutions::lookup(year, day) else {
        eprintln!("No solution registered for {year}, day {day}");
        std::process::exit(1);
    };

    let suffix = if dryrun { " dry-run" } else { "" };
    println!("-- Advent of Code {year} day {day}{suffix} --");

    if dryrun {
        println!(
            "Input: {} bytes, {} lines",
            input.len(),
            input.lines().count()
        );
        return;
    }
    run(&input, part);
}
