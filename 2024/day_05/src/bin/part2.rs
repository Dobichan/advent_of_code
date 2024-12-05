use nom::{
    bytes::complete::tag, character::complete, multi::separated_list0, sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct PrintJob {
    pages: Vec<i32>,
    // rules: &'a multimap::MultiMap<i32, i32>,
}

impl PrintJob {
    fn is_valid(&self, rules: &multimap::MultiMap<i32, i32>) -> bool {
        for i in 0..self.pages.len() {
            let checks = rules.get_vec(&self.pages[i]);
            if checks.is_some() {
                for check in checks.unwrap() {
                    for k in 0..i {
                        if self.pages[k] == *check {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn correct_page_order(&self, rules: &multimap::MultiMap<i32, i32>) -> PrintJob {
        let mut new_order: Vec<i32> = vec![];
        for p in self.pages.iter() {
            // println!("page {:?}", p);

            let mut inserted = false;
            for k in 0..new_order.len() {
                // println!("check new p: {:?}[{k}]", new_order[k]);
                let temp = rules.get_vec(&p);
                // println!("temp: {:?}", temp);
                if temp.is_some() {
                    // println!("{:?}", temp.unwrap());
                    if temp.unwrap().contains(&new_order[k]) {
                        // println!("rules of p contains new[k]");
                        new_order.insert(k, *p);
                        inserted = true;
                        break;
                    }
                }
            }
            if !inserted {
                new_order.push(*p);
            }
            // println!("new order: {:?}", new_order);
        }
        PrintJob { pages: new_order }
    }

    fn get_middle_page(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_job(input: &str) -> IResult<&str, Vec<i32>> {
    let a = separated_list0(tag(","), complete::i32)(input)?;
    Ok(("", a.1))
}
fn page_rule(input: &str) -> IResult<&str, (i32, i32)> {
    let a = separated_pair(complete::i32, tag("|"), complete::i32)(input)?;
    Ok((input, (a.1 .0, a.1 .1)))
}

fn part2(input: &str) -> i32 {
    let mut rules = true;
    let mut rule_data: multimap::MultiMap<i32, i32> = multimap::MultiMap::new();
    let mut jobs: Vec<PrintJob> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            rules = false;
            continue;
        }

        if rules {
            let pair = page_rule(line).unwrap();
            rule_data.insert(pair.1 .0, pair.1 .1);
        } else {
            let pages = parse_job(line).unwrap();
            let job = PrintJob { pages: pages.1 };
            jobs.push(job);
        }
    }

    jobs.iter()
        .filter(|j| !j.is_valid(&rule_data))
        .map(|j| j.correct_page_order(&rule_data).get_middle_page())
        .sum::<i32>()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let dummy = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part2(dummy), 123);
    }
}
