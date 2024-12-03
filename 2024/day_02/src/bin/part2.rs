#[derive(Debug)]
pub struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    pub fn valid(&self) -> bool {
        if check_levels(&self.levels) {
            return true;
        }

        // dampener function - remove at index and test
        for i in 0..self.levels.len() {
            let mut sub_data = self.levels.clone();
            sub_data.remove(i);
            if check_levels(&sub_data) {
                return true;
            }
        }
        false
    }
}

fn check_levels(data: &Vec<i32>) -> bool {
    let ascending: bool;

    if data[0] < data[1] {
        ascending = true;
    } else {
        ascending = false;
    }

    for i in 0..(data.len() - 1) {
        if ascending {
            if data[i] > data[i + 1] {
                return false;
            }
        } else {
            if data[i] < data[i + 1] {
                return false;
            }
        }
        let diff = i32::abs(data[i] - data[i + 1]);
        if diff > 3 || diff == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let reports = parse_input(input);
    let answ = part2(&reports);
    dbg!(answ);
}

fn parse_input(input: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = vec![];
    for line in input.lines() {
        let mut rep: Report = Report { levels: vec![] };

        for level in line.split_whitespace() {
            rep.levels.push(level.parse::<i32>().unwrap());
        }

        reports.push(rep);
    }

    reports
}

fn part2(reports: &Vec<Report>) -> i32 {
    reports.iter().filter(|&report| report.valid()).count() as i32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = parse_input(dummy);
        let answ = part2(&reports);
        println!("{answ}");

        assert_eq!(answ, 4);
    }
}
