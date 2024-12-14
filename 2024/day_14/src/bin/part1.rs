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
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input, 101, 103, 100);
    dbg!(output);
}

fn part1(input: &str, width: i32, height: i32, iterations: i32) -> i32 {
    let mut robots = parse(input);

    let mut quadrants = vec![0, 0, 0, 0];

    for ref mut robot in robots {
        // println!("Moving robot: {:?}", robot);

        robot.x = (robot.x + robot.vx * iterations) % (width);
        robot.y = (robot.y + robot.vy * iterations) % (height);
        if robot.x < 0 {
            robot.x += width;
        }
        if robot.y < 0 {
            robot.y += height;
        }

        println!("New position: {:?}", robot);

        // Quadrant1
        if robot.x < width / 2 && robot.y < height / 2 {
            quadrants[0] += 1;
        }

        // Quadrant2
        if robot.x > width / 2 && robot.y < height / 2 {
            quadrants[1] += 1;
        }
        // Quadrant1
        if robot.x < width / 2 && robot.y > height / 2 {
            quadrants[2] += 1;
        }
        // Quadrant1
        if robot.x > width / 2 && robot.y > height / 2 {
            quadrants[3] += 1;
        }
    }

    println!("Quadrants: {:?}", quadrants);
    quadrants.iter().product()
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

        assert_eq!(part1(dummy, 11, 7, 100), 12);
    }

    #[test]
    fn test_parse() {
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

        let expected = vec![
            Robot {
                x: 0,
                y: 4,
                vx: 3,
                vy: -3,
            },
            Robot {
                x: 6,
                y: 3,
                vx: -1,
                vy: -3,
            },
            Robot {
                x: 10,
                y: 3,
                vx: -1,
                vy: 2,
            },
            Robot {
                x: 2,
                y: 0,
                vx: 2,
                vy: -1,
            },
            Robot {
                x: 0,
                y: 0,
                vx: 1,
                vy: 3,
            },
            Robot {
                x: 3,
                y: 0,
                vx: -2,
                vy: -2,
            },
            Robot {
                x: 7,
                y: 6,
                vx: -1,
                vy: -3,
            },
            Robot {
                x: 3,
                y: 0,
                vx: -1,
                vy: -2,
            },
            Robot {
                x: 9,
                y: 3,
                vx: 2,
                vy: 3,
            },
            Robot {
                x: 7,
                y: 3,
                vx: -1,
                vy: 2,
            },
            Robot {
                x: 2,
                y: 4,
                vx: 2,
                vy: -3,
            },
            Robot {
                x: 9,
                y: 5,
                vx: -3,
                vy: -3,
            },
        ];

        assert_eq!(parse(dummy), expected);
    }

    #[test]
    fn test_position_parsing() {
        let input = "p=0,4 v=3,-3";

        let pos = parse_position(input);

        assert_eq!(pos, Ok((" v=3,-3", (0, 4))));
    }

    #[test]
    fn test_velocity_parsing() {
        let input = " v=3,-3";

        let pos = parse_velocity(input);

        assert_eq!(pos, Ok(("", (3, -3))));
    }

    #[test]
    fn test_robot_parsing() {
        let input = "p=0,4 v=3,-3";

        let robot = parse_robot(input);

        let expected_robot = Robot {
            x: 0,
            y: 4,
            vx: 3,
            vy: -3,
        };

        assert_eq!(robot, Ok(("", expected_robot)));
    }
}
