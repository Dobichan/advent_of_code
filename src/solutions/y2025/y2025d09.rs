use crate::AoCSolution;
use std::{fs::File, io::Write, path::Path, usize};

const YEAR: u16 = 2025;
const DAY: u8 = 9;

const WIDTH: usize = 2000;
const HEIGHT: usize = 2000;

#[derive(Default, Debug)]
pub struct Solution {
    pub(crate) points: Option<Vec<(usize, usize)>>,
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        self.points = Some(Vec::with_capacity(1000));
        let points = self.points.as_mut().unwrap();

        for line in input.trim().lines() {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            points.push((x, y));
        }

        let mut maks_area = 0;
        let mut maks_pos1 = 0;
        let mut maks_pos2 = 0;

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let x1 = points[i].0.min(points[j].0);
                let x2 = points[i].0.max(points[j].0);
                let y1 = points[i].1.min(points[j].1);
                let y2 = points[i].1.max(points[j].1);

                let area = (x2 - x1 + 1) * (y2 - y1 + 1);
                if area > maks_area {
                    maks_area = area;
                    maks_pos1 = i;
                    maks_pos2 = j;
                }
            }
        }
        println!(
            "Found maks area: {maks_area} ({:?},{:?} - {:?},{:?})",
            points[maks_pos1].0, points[maks_pos1].1, points[maks_pos2].0, points[maks_pos2].1
        );
        maks_area.to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        if self.points.is_none() {
            self.points = Some(Vec::with_capacity(1000));
            let points = self.points.as_mut().unwrap();

            for line in input.trim().lines() {
                let (x, y) = line.split_once(',').unwrap();
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                points.push((x, y));
            }
        }
        let points = self.points.as_mut().unwrap();

        let mut min_x = usize::MAX;
        let mut max_x = 0;
        let mut min_y = usize::MAX;
        let mut max_y = 0;

        for p in points.into_iter() {
            if p.0 < min_x {
                min_x = p.0;
            }
            if p.0 > max_x {
                max_x = p.0;
            }

            if p.1 < min_y {
                min_y = p.1;
            }
            if p.1 > max_y {
                max_y = p.1;
            }
        }

        // Define the output file name
        const FILENAME: &str = "plot_output.ppm";

        let min_x = points.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let max_x = points.iter().map(|(x, _)| *x).max().unwrap_or(WIDTH);
        let min_y = points.iter().map(|(_, y)| *y).min().unwrap_or(0);
        let max_y = points.iter().map(|(_, y)| *y).max().unwrap_or(HEIGHT);

        let range_x = (max_x - min_x) as f64;
        let range_y = (max_y - min_y) as f64;

        // Define the color for the line (e.g., bright red)
        const LINE_R: u8 = 255;
        const LINE_G: u8 = 0;
        const LINE_B: u8 = 0;
        // Define the background color (e.g., white)
        const BG_R: u8 = 255;
        const BG_G: u8 = 255;
        const BG_B: u8 = 255;

        let normalized_coords: Vec<(u32, u32)> = points
            .iter()
            .map(|&(x, y)| {
                // Normalize x to 0.0-1.0 range
                let norm_x = (x as f64 - min_x as f64) / range_x;
                // Normalize y to 0.0-1.0 range
                let norm_y = (y as f64 - min_y as f64) / range_y;

                // Scale to image dimensions.
                // The y-axis in graphics usually goes from top (0) to bottom (HEIGHT),
                // so we invert the y-coordinate (1.0 - norm_y) to match standard math plotting
                // where y increases going up.
                let scaled_x = (norm_x * (WIDTH - 1) as f64).round() as u32;
                let scaled_y = ((1.0 - norm_y) * (HEIGHT - 1) as f64).round() as u32;

                (scaled_x, scaled_y)
            })
            .collect();

        let total_pixels = (WIDTH * HEIGHT) as usize;
        // Buffer size is total_pixels * 3 components (R, G, B)
        let mut buffer: Vec<u8> = vec![BG_R, BG_G, BG_B].repeat(total_pixels);

        for i in 0..normalized_coords.len() {
            let (x2, y2) = normalized_coords[i];

            // Plot the current point as a small square for visibility
            plot_point(&mut buffer, x2, y2, LINE_R, LINE_G, LINE_B);

            // If not the first point, draw a line from the previous point to the current one
            if i > 0 {
                let (x1, y1) = normalized_coords[i - 1];
                draw_line(&mut buffer, x1, y1, x2, y2, LINE_R, LINE_G, LINE_B);
            }
        }

        // --- 5. Write to PPM File (P3 ASCII format) ---
        let path = Path::new(FILENAME);
        let mut file = File::create(&path).unwrap();

        // Write PPM Header (P3: ASCII Color, WIDTH, HEIGHT, Max Color Value)
        let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
        file.write_all(header.as_bytes()).unwrap();

        // Write pixel data
        let mut data_string = String::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = ((y * WIDTH + x) * 3) as usize;
                // Get R, G, B components
                let r = buffer[index];
                let g = buffer[index + 1];
                let b = buffer[index + 2];

                // Format as "R G B " (with a space)
                data_string.push_str(&format!("{} {} {} ", r, g, b));

                // Add a newline periodically for better readability in ASCII
                if x % 10 == 9 {
                    data_string.push('\n');
                }
            }
            data_string.push('\n'); // Newline after each row
        }

        file.write_all(data_string.as_bytes()).unwrap();

        0.to_string()
    }
}

fn plot_point(buffer: &mut Vec<u8>, cx: u32, cy: u32, r: u8, g: u8, b: u8) {
    for dy in -1..=1 {
        for dx in -1..=1 {
            let x = (cx as i32 + dx) as u32;
            let y = (cy as i32 + dy) as u32;

            if x < WIDTH as u32 && y < HEIGHT as u32 {
                let index = ((y * WIDTH as u32 + x) * 3) as usize;
                buffer[index] = r;
                buffer[index + 1] = g;
                buffer[index + 2] = b;
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u8>, x1: u32, y1: u32, x2: u32, y2: u32, r: u8, g: u8, b: u8) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let steps = if dx > dy { dx } else { dy };

    let x_inc = (x2 as f64 - x1 as f64) / steps as f64;
    let y_inc = (y2 as f64 - y1 as f64) / steps as f64;

    let mut x: f64 = x1 as f64;
    let mut y: f64 = y1 as f64;

    for _i in 0..=steps {
        let ix = x.round() as u32;
        let iy = y.round() as u32;

        if ix < WIDTH as u32 && iy < HEIGHT as u32 {
            let index = ((iy * WIDTH as u32 + ix) * 3) as usize;
            buffer[index] = r;
            buffer[index + 1] = g;
            buffer[index + 2] = b;
        }

        x += x_inc;
        y += y_inc;
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
        let answer = sol.part1(&EXAMPLE_INPUT);

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
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "");
    }
}
