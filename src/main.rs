use advent_of_code::{
    parsing,
    solutions::{y2015, y2024, y2025},
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
    let part = &args[4];
    let dryrun = args.get(5);

    let input = parsing::read_input(file);

    match year {
        2015 => y2015::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2016 => y2016::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2017 => y2017::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2018 => y2018::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2019 => y2019::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2020 => y2020::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2021 => y2021::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2022 => y2022::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        // 2023 => y2023::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        2024 => y2024::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),
        2025 => y2025::run(&input, day, part.chars().next().unwrap(), dryrun.is_some()),

        _ => panic!("Unsupported year {}", year),
    }

    // if let Some(solution) = solutions.get(&(year, day)) {
    //     let mut solution: Box<dyn AoCSolution> = solution();
    //     match part.chars().next().unwrap() {
    //         '1' => solution.run_part1(&file, dryrun.is_some()),
    //         '2' => solution.run_part2(&file, dryrun.is_some()),
    //         'a' => solution.run(&file, dryrun.is_some()),
    //         _ => panic!("Illegal part option"),
    //     }
    // } else {
    //     println!("Solution not found (or not registered)");
    // }
}
