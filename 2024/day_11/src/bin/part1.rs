
fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<String> {
    let ret = input.split(" ").map(|n| n.to_string()).collect();
    ret
}

fn blink(loop_num: u32, digit: &str, max_count: u32) -> u64 {
    if loop_num >= max_count {
        // println!("{loop_num} {digit} -> 1");
        return 1;
    }
    let a: u128 = digit.parse::<u128>().unwrap();
    let mut ret_a = String::new();
    let mut ret_b = String::new();

    // println!("{loop_num} a: {a}");
    if a == 0 {
        ret_a = "1".to_string();
    } else if digit.len() % 2 == 0 {
        let split = digit.split_at(digit.len() / 2);
        let a: u64 = split.0.parse().unwrap();
        let b: u64 = split.1.parse().unwrap();
        ret_a = a.to_string();
        ret_b = b.to_string();
    } else {
        ret_a = (a * 2024).to_string();
    }
    // println!("ret_a: {:?}  b: {:?}", ret_a, ret_b);

    let mut ret_num = blink(loop_num + 1, ret_a.as_str(), max_count);
    if ret_b.len() > 0 {
        ret_num += blink(loop_num + 1, ret_b.as_str(), max_count);
    }
    ret_num
}
fn part1(input: &str) -> u64 {
    let data = parse(input);

    let mut count:u64 = 0;

    for digit in data{
        count += blink(0, &digit, 25);
    }
    count
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1_blink() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 1);
        }
        println!("{count}");

        assert_eq!(count, 3);
    }

    #[test]
    fn test_2_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 2);
        }
        println!("{count}");

        assert_eq!(count, 4);
    }

    #[test]
    fn test_3_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 3);
        }
        println!("{count}");

        assert_eq!(count, 5);
    }

    #[test]
    fn test_4_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 4);
        }
        println!("{count}");

        assert_eq!(count, 9);
    }

    #[test]
    fn test_5_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 5);
        }
        println!("{count}");

        assert_eq!(count, 13);
    }

    #[test]
    fn test_6_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 6);
        }
        println!("{count}");

        assert_eq!(count, 22);
    }

    #[test]
    fn test_25_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let mut count = 0;
        for d in arr {
            count += blink(0, &d, 25);
        }
        println!("{count}");

        assert_eq!(count, 55312);
    }
}
