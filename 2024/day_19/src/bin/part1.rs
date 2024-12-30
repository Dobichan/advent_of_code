use std::collections::HashMap;


fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, targets) = input.split_once("\n\n").unwrap();
    (
        towels.split(",").map(|t| t.trim()).collect(),
        targets.lines().collect(),
    )
}

fn can_create<'a>(pattern: &'a str, towels: &Vec<&str>, hash:&mut HashMap<&'a str, bool>) -> bool {
    if let Some(possible) = hash.get(pattern){
        return *possible;
    }
    if pattern.len() == 0 {
        // println!("success!");
        return true;
    }
    for towel in towels {
        // println!("Checking if {pattern} starts with {towel}");
        if let Some(remaining_pattern) = pattern.strip_prefix(towel) {
            let ret = can_create(remaining_pattern, towels, hash);
            hash.insert(pattern, ret);

            if ret {
                return true;
            }
        }
    }
    false
}

fn part1(input: &str) -> i32 {
    let (towels, targets) = parse(input);

    let mut ret = 0;
    let mut hash = HashMap::new();

    for target in targets {
        println!("Target: {target}");
        if can_create(target, &towels, &mut hash) {
            ret += 1;
        }
        println!();
    }
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        assert_eq!(part1(dummy), 6);
    }
}
