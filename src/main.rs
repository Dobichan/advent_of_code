use advent_of_code::{AoCSolution, get_solutions};

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
    let part = &args[4];
    let mut dryrun = false;
    if args.len() == 6 && &args[5] == "d" {
        dryrun = true;
    }

    let solutions = get_solutions();

    if let Some(solution) = solutions.get(&(year, day)) {
        let mut solution: Box<dyn AoCSolution> = solution();
        match part.chars().next().unwrap() {
            '1' => solution.run_part1(&file, dryrun),
            '2' => solution.run_part2(&file, dryrun),
            'a' => solution.run(&file, dryrun),
            _ => panic!("Illegal part option"),
        }
    } else {
        println!("Solution not found (or not registered)");
    }
}
