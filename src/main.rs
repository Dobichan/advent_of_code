use advent_of_code::{AoCSolution, get_solutions};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Uage: cargo run <YEAR> <DAY>");
        return;
    }

    let year: u16 = args[1].parse().expect("Invalid YEAR");
    let day: u8 = args[2].parse().expect("Invalid DAY");

    println!("Running {year} - {day}");

    let solutions = get_solutions();

    if let Some(solution) = solutions.get(&(year, day)) {
        let solution: Box<dyn AoCSolution> = solution();
        solution.run();
    } else {
        println!("Solution not found (or not registered)");
    }
}
