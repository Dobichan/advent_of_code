use advent_of_code::{
    parsing,
    runner::{self, Part},
    solutions,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 && args.len() != 6 {
        eprintln!("Uage: advent_of_code <YEAR> <DAY> <file> <part> [<dryrun>]");
        eprintln!("      <file> - path to file with input");
        eprintln!("      <part> - which part to solve, 1 | 2 | a(ll)");
        eprintln!("      <dryrun> - optional dry-run, d");
        return;
    }

    let year: u16 = args[1].parse().expect("Invalid YEAR");
    let day: u8 = args[2].parse().expect("Invalid DAY");
    let file = &args[3];
    let part: Part = args[4].parse().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let dryrun = args.len() == 6;

    let input = parsing::read_input(file);

    let Some(mut solution) = solutions::lookup(year, day) else {
        eprintln!("No solution registered for {year}, day {day}");
        std::process::exit(1);
    };

    runner::run(solution.as_mut(), year, day, &input, part, dryrun);
}
