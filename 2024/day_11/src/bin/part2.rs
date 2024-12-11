use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<String> {
    let ret = input.split(" ").map(|n| n.to_string()).collect();
    ret
}

fn blink(
    loop_num: u32,
    digit: u128,
    max_count: u32,
    cache: Rc<RefCell<HashMap<(u32, u32), u64>>>,
) -> u64 {
    if loop_num >= max_count {
        // println!("{loop_num} {digit} -> 1");
        return 1;
    }

    if digit < 10 {
        if let Some(value) = cache.borrow().get(&(digit as u32, loop_num)) {
            return *value;
        }
    }

    let ret_a: u128;
    let mut ret_b: Option<u128> = None;

    // println!("{loop_num} a: {digit}");

    let to_run = max_count - loop_num;

    if digit == 0 {
        if to_run == 1 {
            // 0->1
            return 1;
        }
        if to_run == 2 {
            // 1->2024
            return 1;
        }

        if to_run == 3 {
            // 2024 -> 20,24
            return 2;
        }

        if to_run == 4 {
            // 20, 24 -> 2, 0, 2, 4
            return 4;
        }

        // 2, 0, 2, 4 -> 4048, 1, 4048, 8096
        let ret = blink(loop_num + 5, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 5, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 5, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 5, 8096, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 1 {
        if to_run == 1 {
            // 1 -> 2024
            return 1;
        }
        if to_run == 2 {
            // 2024 -> 20,24
            return 2;
        }
        if to_run == 3 {
            // 20, 24 -> 2, 0, 2, 4
            return 4;
        }
        // 2, 0, 2, 4 -> 4048, 1, 4048, 8096
        let ret = blink(loop_num + 4, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 8096, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 2 {
        if to_run == 1 {
            // 2 -> 4048
            return 1;
        }
        if to_run == 2 {
            // 4048 -> 40, 48
            return 2;
        }
        if to_run == 3 {
            // 40, 48 -> 4, 0, 4, 8
            return 4;
        }
        // 4, 0, 4, 8 -> 8096, 1, 8096, 16192
        let ret = blink(loop_num + 4, 8096, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 8096, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 16192, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 3 {
        if to_run == 1 {
            // 3 -> 6072
            return 1;
        }
        if to_run == 2 {
            // 6072 -> 60, 72
            return 2;
        }
        if to_run == 3 {
            // 60, 72 -> 6, 0, 7, 2
            return 4;
        }
        // 6, 0, 7, 2 -> 12144, 1, 14168, 4048
        let ret = blink(loop_num + 4, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 14168, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 4048, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 4 {
        if to_run == 1 {
            // 4 -> 8096
            return 1;
        }
        if to_run == 2 {
            // 8096 -> 80, 96
            return 2;
        }
        if to_run == 3 {
            // 80, 96 -> 8,0,9,6
            return 4;
        }
        // 8,0,9,6 -> 16192, 1, 18216, 12144
        let ret = blink(loop_num + 4, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 18216, max_count, Rc::clone(&cache))
            + blink(loop_num + 4, 12144, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 5 {
        if to_run == 1 {
            // 5 -> 10120
            return 1;
        }
        if to_run == 2 {
            // 10120 -> 20482880
            return 1;
        }
        if to_run == 3 {
            // 20482880 -> 2048, 2880
            return 2;
        }
        if to_run == 4 {
            // 2048, 2880 -> 20, 48, 28, 80
            return 4;
        }
        if to_run == 5 {
            // 20, 48, 28, 80 -> 2,0,4,8,2,8,8,0
            return 8;
        }
        // 2,0,4,8,2,8,8,0 -> 4048, 1, 8096, 16192, 4048, 16192, 16192, 1
        let ret = blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 8096, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 1, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 6 {
        if to_run == 1 {
            // 6 -> 12144
            return 1;
        }
        if to_run == 2 {
            // 12144 -> 24579456
            return 1;
        }
        if to_run == 3 {
            // 24579456 -> 2457, 9456
            return 2;
        }
        if to_run == 4 {
            // 2457, 9456 -> 24,57,94,56
            return 4;
        }
        if to_run == 5 {
            // 24,57,94,56 -> 2,4,5,7,9,4,5,6
            return 8;
        }
        // 2,4,5,7,9,4,5,6 -> 4048, 8096, 10120, 14168, 18216, 8096, 10120, 12144
        let ret = blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 8096, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 10120, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 14168, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 18216, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 8096, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 10120, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 7 {
        if to_run == 1 {
            // 7 -> 14168
            return 1;
        }
        if to_run == 2 {
            // 14168 -> 28676032
            return 1;
        }
        if to_run == 3 {
            // 28676032 -> 2867, 6032
            return 2;
        }
        if to_run == 4 {
            // 2867, 6032 -> 28, 67, 60, 32
            return 4;
        }
        if to_run == 5 {
            // 28, 67, 60, 32 -> 2, 8, 6, 7, 6, 0, 3, 2
            return 8;
        }
        // 2, 8, 6, 7, 6, 0, 3, 2 -> 4048, 16192, 12144, 14168, 12144, 1, 6072, 4048
        let ret = blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 14168, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 1, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 6072, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 4048, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 8 {
        if to_run == 1 {
            // 8 -> 16192
            return 1;
        }
        if to_run == 2 {
            // 16192 -> 32772608
            return 1;
        }
        if to_run == 3 {
            // 32772608 -> 3277, 2608
            return 2;
        }
        if to_run == 4 {
            // 3277, 2608 -> 32, 77, 26, 08
            return 4;
        }
        if to_run == 5 {
            // 32, 77, 26, 08 -> 3, 2, 7, 7, 2, 6, 8
            return 7;
        }
        // 3, 2, 7, 7, 2, 6, 8 -> 6072, 4048, 14168, 14168, 4048, 12144, 32772608
        let ret = blink(loop_num + 6, 6072, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 14168, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 14168, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 4048, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 32772608, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit == 9 {
        if to_run == 1 {
            // 9->18216
            return 1;
        }
        if to_run == 2 {
            // 18216 -> 36869184
            return 1;
        }
        if to_run == 3 {
            // 36869184 -> 3686, 9184
            return 2;
        }
        if to_run == 4 {
            // 3686, 9184 -> 36, 86, 91, 84
            return 4;
        }
        if to_run == 5 {
            // 36, 86, 91, 84 -> 3, 6, 8, 6, 9, 1, 8, 4
            return 8;
        }
        // 3, 6, 8, 6, 9, 1, 8, 4 -> 6072, 12144, 16192, 12144, 18216, 2024, 16192, 8096
        let ret = blink(loop_num + 6, 6072, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 12144, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 18216, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 2024, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 16192, max_count, Rc::clone(&cache))
            + blink(loop_num + 6, 8096, max_count, Rc::clone(&cache));
        cache.borrow_mut().insert((digit as u32, loop_num), ret);
        return ret;
    } else if digit.to_string().len() % 2 == 0 {
        let temp = digit.to_string();
        let split = temp.split_at(temp.len() / 2);
        ret_a = split.0.parse().unwrap();
        ret_b = Some(split.1.parse().unwrap());
    } else {
        ret_a = digit * 2024;
    }
    // println!("ret_a: {:?}  b: {:?}", ret_a, ret_b);

    let mut ret_num = blink(loop_num + 1, ret_a, max_count, Rc::clone(&cache));
    if ret_b.is_some() {
        ret_num += blink(loop_num + 1, ret_b.unwrap(), max_count, Rc::clone(&cache));
    }
    ret_num
}

fn part1(input: &str) -> u64 {
    let data = parse(input);

    let mut count: u64 = 0;

    // let digit = &data[0];

    let cache: HashMap<(u32, u32), u64> = HashMap::new();
    let cache = Rc::new(RefCell::new(cache));

    for digit in data {
        println!("digit: {digit}");
        count += blink(0, digit.parse().unwrap(), 75, Rc::clone(&cache));
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
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 1, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 3);
    }

    #[test]
    fn test_2_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 2, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 4);
    }

    #[test]
    fn test_3_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 3, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 5);
    }

    #[test]
    fn test_4_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 4, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 9);
    }

    #[test]
    fn test_5_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 5, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 13);
    }

    #[test]
    fn test_6_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 6, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 22);
    }

    #[test]
    fn test_25_blinks() {
        let dummy = "125 17";
        let arr = parse(&dummy);
        let cache: HashMap<(u32, u32), u64> = HashMap::new();
        let cache = Rc::new(RefCell::new(cache));
        let mut count = 0;
        for d in arr {
            count += blink(0, d.parse().unwrap(), 25, Rc::clone(&cache));
        }
        println!("{count}");

        assert_eq!(count, 55312);
    }
}
