use std::{cmp::Reverse, fmt::Display};

use itertools::Itertools;

use crate::{AoCSolution, parsing::input_lines, point::Point};

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    common: isize,
    from: isize,
    to: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    top_left: Point,
    bot_left: Point,
    top_right: Point,
    bot_right: Point,
    area: isize,
}

impl Rectangle {
    fn spanning(a: Point, b: Point) -> Self {
        let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
        let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));

        Self {
            top_left: Point::new(x1, y1),
            top_right: Point::new(x2, y1),
            bot_left: Point::new(x1, y2),
            bot_right: Point::new(x2, y2),
            area: (x2 - x1 + 1) * (y2 - y1 + 1),
        }
    }
}

fn rectangles(points: &[Point]) -> Vec<Rectangle> {
    let mut rectangles: Vec<Rectangle> = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rectangle::spanning(*a, *b))
        .collect();
    rectangles.sort_by_key(|r| Reverse(r.area));
    rectangles
}

fn boundary_lines(points: &[Point]) -> (Vec<Line>, Vec<Line>) {
    let mut horizontal = Vec::with_capacity(points.len());
    let mut vertical = Vec::with_capacity(points.len());

    for (p1, p2) in points.iter().circular_tuple_windows() {
        if p1.x == p2.x {
            vertical.push(Line {
                common: p1.x,
                from: p1.y.min(p2.y),
                to: p1.y.max(p2.y),
            });
        } else {
            horizontal.push(Line {
                common: p1.y,
                from: p1.x.min(p2.x),
                to: p1.x.max(p2.x),
            });
        }
    }
    horizontal.sort();
    vertical.sort();

    (horizontal, vertical)
}

fn points_inside(rect: &Rectangle, points: &[Point]) -> bool {
    points.iter().any(|p| {
        p.x > rect.top_left.x
            && p.x < rect.top_right.x
            && p.y > rect.top_left.y
            && p.y < rect.bot_left.y
    })
}

fn horizontals_crossing(rect: &Rectangle, horizontals: &[Line]) -> bool {
    horizontals.iter().any(|l| {
        l.common > rect.top_left.y
            && l.common < rect.bot_left.y
            && l.from < rect.top_right.x
            && l.to > rect.top_left.x
    })
}

fn verticals_crossing(rect: &Rectangle, verticals: &[Line]) -> bool {
    verticals.iter().any(|l| {
        l.common > rect.top_left.x
            && l.common < rect.top_right.x
            && l.from < rect.bot_left.y
            && l.to > rect.top_left.y
    })
}

impl AoCSolution for Solution {
    type Parsed = Vec<Point>;

    fn parse(&self, input: &str) -> Self::Parsed {
        input_lines(input)
            .map(|line| {
                let (x, y) = line.split_once(',').expect("Expected x,y");
                Point::new(
                    x.parse().expect("x is not a number"),
                    y.parse().expect("y is not a number"),
                )
            })
            .collect()
    }

    fn part1(&self, points: &Self::Parsed) -> impl Display {
        points
            .iter()
            .tuple_combinations()
            .map(|(a, b)| Rectangle::spanning(*a, *b).area)
            .max()
            .expect("Need at least two points")
    }

    fn part2(&self, points: &Self::Parsed) -> impl Display {
        let (horizontal, vertical) = boundary_lines(points);

        rectangles(points)
            .iter()
            .filter(|r| !points_inside(r, points))
            .filter(|r| !horizontals_crossing(r, &horizontal))
            .filter(|r| !verticals_crossing(r, &vertical))
            .map(|r| r.area)
            .next()
            .expect("No rectangle satisfies the constraints")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "50");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "24");
    }
}
