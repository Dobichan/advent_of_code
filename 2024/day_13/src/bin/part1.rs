use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct ClawMachine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

impl ClawMachine {
    fn get_win_cost(&self) -> i32 {
        let mut ret = 0;

        // A*a0 + B*b0 = prize0
        // A*a1 + B*b1 = prize1

        // Solving gives
        // A = (b0*p1 - b1*p0) / (a1*b0 - a0*b1)
        // B = (a1*p0 - a0*p1) / (a1*b0 - a0*b1)

        // If A and B is whole number (no fractions) we win
        // return A * 3 + B for the token cost

        let divisor: i32 = (self.a.1 * self.b.0) - (self.a.0 * self.b.1);
        if divisor != 0 {
            let a: f32 = ((self.b.0 * self.prize.1) - (self.b.1 * self.prize.0)) as f32;
            let b: f32 = ((self.a.1 * self.prize.0) - (self.a.0 * self.prize.1)) as f32;

            let a: f32 = a / divisor as f32;
            let b: f32 = b / divisor as f32;

            if a.fract() < f32::EPSILON && b.fract() < f32::EPSILON {
                ret = (a * 3.0 + b) as i32;
            }
        }

        ret
    }
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_button_a(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, a) = terminated(
        preceded(
            tag("Button A: X+"),
            separated_pair(complete::i32, tag(", Y+"), complete::i32),
        ),
        line_ending,
    )(input)?;

    Ok((input, a))
}

fn parse_button_b(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, b) = terminated(
        preceded(
            tag("Button B: X+"),
            separated_pair(complete::i32, tag(", Y+"), complete::i32),
        ),
        line_ending,
    )(input)?;

    Ok((input, b))
}

fn parse_button_prize(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, prize) = preceded(
        tag("Prize: X="),
        separated_pair(complete::i32, tag(", Y="), complete::i32),
    )(input)?;

    Ok((input, prize))
}

fn claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, (a, b, prize)) =
        tuple((parse_button_a, parse_button_b, parse_button_prize))(input)?;

    Ok((input, ClawMachine { a, b, prize }))
}

fn parse(input: &str) -> Vec<ClawMachine> {
    separated_list1(tuple((line_ending, line_ending)), claw_machine)(input)
        .unwrap()
        .1
}

fn part1(input: &str) -> i32 {
    let machines = parse(input);

    let mut total_prize = 0;

    for machine in machines {
        println!("{:?}", machine);
        println!("win cost: {:?}", machine.get_win_cost());
        total_prize += machine.get_win_cost();
    }
    total_prize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(part1(dummy), 480);
    }
}
