use itertools::Itertools;

use crate::AoCSolution;

const YEAR: u16 = 2025;
const DAY: u8 = 9;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    common: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rectangle {
    top_left: Point,
    bot_left: Point,
    top_right: Point,
    bot_right: Point,
    area: usize,
}
#[derive(Default, Debug)]
pub struct Solution {
    pub(crate) points: Option<Vec<Point>>,
}

fn get_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn get_rectangles(points: &Vec<Point>) -> Vec<Rectangle> {
    let mut rectangles = Vec::with_capacity(130000);

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let x1 = points[i].x.min(points[j].x);
            let x2 = points[i].x.max(points[j].x);
            let y1 = points[i].y.min(points[j].y);
            let y2 = points[i].y.max(points[j].y);

            let area = (x2 - x1 + 1) * (y2 - y1 + 1);

            let rect = Rectangle {
                top_left: { Point { x: x1, y: y1 } },
                top_right: { Point { x: x2, y: y1 } },
                bot_left: { Point { x: x1, y: y2 } },
                bot_right: { Point { x: x2, y: y2 } },
                area,
            };

            rectangles.push(rect);
        }
    }
    rectangles.sort_by(|a, b| b.area.cmp(&a.area));
    rectangles
}

fn points_inside(rect: &Rectangle, points: &Vec<Point>) -> bool {
    // println!("Checking {:?}", rect);

    for p in points {
        // println!("  {:?}", p);
        if p.x > rect.top_left.x
            && p.x < rect.top_right.x
            && p.y > rect.top_left.y
            && p.y < rect.bot_left.y
        {
            // println!("Inside");
            return true;
        }
    }
    // println!("None inside");
    false
}

fn horizontals_crossing(rect: &Rectangle, horizontals: &Vec<Line>) -> bool {
    // println!("horizontals: {:?}", rect);
    let mut crossing = horizontals.iter().filter(|l| {
        l.common > rect.top_left.y
            && l.common < rect.bot_left.y
            && ((l.from < rect.top_left.x && l.to > rect.top_left.x)
                || (l.from < rect.top_right.x && l.to > rect.top_right.y))
    });
    if crossing.next().is_some() {
        // println!("crossing found");
        return true;
    }
    // println!("No crossing");
    false
}

fn verticals_crossing(rect: &Rectangle, verticals: &Vec<Line>) -> bool {
    // println!("Verticals: {:?}", rect);
    let mut crossing = verticals.iter().filter(|l| {
        l.common > rect.top_left.x
            && l.common < rect.top_right.x
            && ((l.from < rect.top_left.y && l.to > rect.top_left.y)
                || (l.from < rect.bot_left.y && l.to > rect.bot_left.y))
    });
    if crossing.next().is_some() {
        // println!("Crossing found");
        return true;
    }
    // println!("No crossing");
    false
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str, dryrun: bool) -> String {
        if dryrun {
            return 1234567890.to_string();
        }
        let mut maks_area = 0;
        let points = get_points(input);

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let x1 = points[i].x.min(points[j].x);
                let x2 = points[i].x.max(points[j].x);
                let y1 = points[i].y.min(points[j].y);
                let y2 = points[i].y.max(points[j].y);

                let area = (x2 - x1 + 1) * (y2 - y1 + 1);
                if area > maks_area {
                    maks_area = area;
                }
            }
        }
        self.points = Some(points);
        println!("Found maks area: {maks_area}");

        maks_area.to_string()
    }

    fn part2(&mut self, input: &str, dryrun: bool) -> String {
        if dryrun {
            return 1234567890.to_string();
        }

        if self.points.is_none() {
            self.points = Some(get_points(input));
        }

        let points = self.points.as_ref().unwrap();

        let rectangles = get_rectangles(&points);
        let mut horizontal = Vec::with_capacity(points.len());
        let mut vertical = Vec::with_capacity(points.len());

        for (p1, p2) in points.iter().circular_tuple_windows() {
            if p1.x == p2.x {
                vertical.push(Line {
                    common: p1.x,
                    from: p1.y.min(p2.y),
                    to: p2.y.max(p1.y),
                });
            } else {
                horizontal.push(Line {
                    common: p1.y,
                    from: p1.x.min(p2.x),
                    to: p2.x.max(p1.x),
                });
            }
        }
        vertical.sort();
        horizontal.sort();

        let rects_no_points_inside = rectangles.iter().filter(|r| !points_inside(&r, points));
        let rects_no_horr_crossing =
            rects_no_points_inside.filter(|r| !horizontals_crossing(&r, &horizontal));
        let mut rects_no_vert_crossing =
            rects_no_horr_crossing.filter(|r| !verticals_crossing(&r, &vertical));

        rects_no_vert_crossing.next().unwrap().area.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "7,1\n\
                11,1\n\
                11,7\n\
                9,7\n\
                9,5\n\
                2,5\n\
                2,3\n\
                7,3";

        let mut sol = Solution {
            ..Default::default()
        };
        let answer = sol.part1(&EXAMPLE_INPUT, false);

        assert_eq!(answer, "50");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "7,1\n\
                11,1\n\
                11,7\n\
                9,7\n\
                9,5\n\
                2,5\n\
                2,3\n\
                7,3";

        let mut sol = Solution {
            ..Default::default()
        };
        let answer = sol.part2(&EXAMPLE_INPUT, false);

        assert_eq!(answer, "24");
    }
}
