use std::iter;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse_position(input: &str) -> IResult<&str, (i32, i32)> {
    // println!("pos? {:?}", input);
    let (input, p) = preceded(
        tag("p="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;

    // println!("{:?} ({:?})", p, input);

    Ok((input, p))
}

fn parse_velocity(input: &str) -> IResult<&str, (i32, i32)> {
    // println!("vel? {:?}", input);
    let (input, v) = preceded(
        tag(" v="),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;

    // println!("{:?} ({:?})", v, input);

    Ok((input, v))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (pos, vel)) = tuple((parse_position, parse_velocity))(input)?;
    // println!("{:?}", input);
    // println!("{:?}", pos);
    // println!("{:?}", vel);

    Ok((
        input,
        Robot {
            x: pos.0,
            y: pos.1,
            vx: vel.0,
            vy: vel.1,
        },
    ))
}

fn parse(input: &str) -> Vec<Robot> {
    separated_list1(line_ending, parse_robot)(input).unwrap().1
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input, 101, 103);
    dbg!(output);
}

fn move_robot(robot: &Robot, steps: usize, width: i32, height: i32) -> (usize, usize) {
    let mut x = (robot.x + robot.vx * steps as i32) % (width);
    let mut y = (robot.y + robot.vy * steps as i32) % (height);
    if x < 0 {
        x += width;
    }
    if y < 0 {
        y += height;
    }

    (x as usize, y as usize)
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for r in grid {
        for c in r {
            if *c > 0 {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn part2(input: &str, width: i32, height: i32) -> i32 {
    let robots = parse(input);

    for i in 0..10000 {
        let mut grid = vec![vec![0; width as usize]; height as usize];
        for robot in &robots {
            let new_pos = move_robot(&robot, i, width, height);
            grid[new_pos.1][new_pos.0] += 1;
        }

        println!("{i}");
        print_grid(&grid);
    }

    return 0;

    todo!();

    let mut tree_compare = vec![vec![0; width as usize]; height as usize];

    let mid = width / 2;
    for r in 0..height - 1 {
        let low = mid - (r / 2);
        let high = mid + (r / 2);
        println!("r: {r} {low}--{high}");
        for c in low..high + 1 {
            tree_compare[r as usize][c as usize] = 1;
        }
    }
    tree_compare[height as usize - 1][mid as usize] = 1;

    print_grid(&tree_compare);

    let mut i = 0;
    loop {
        i += 1;
        let mut grid = vec![vec![0; width as usize]; height as usize];

        let mut found = true;
        // let mut inside = 0;
        // let mut outside = 0;
        for robot in &robots {
            let new_pos = move_robot(&robot, i, width, height);
            if tree_compare[new_pos.1][new_pos.0] == 0 {
                // outside += 1;
                found = false;
                break;
                // } else {
                //     inside += 1;
            }
            grid[new_pos.1][new_pos.0] += 1;
        }
        if i % 1000 == 0 {
            print!(".");
        }

        if !found {
            continue;
        }
        // if inside < outside {
        //     continue;
        // }

        print_grid(&grid);
        return i as i32;
    }

    0
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!(part2(dummy, 11, 7), 12);
    }
}
