use core::fmt;
use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{parsing::input_lines, point::Point};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T = char> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        (0..self.width() as isize).contains(&p.x) && (0..self.height() as isize).contains(&p.y)
    }

    fn index_of(&self, p: Point) -> Option<usize> {
        self.in_bounds(p)
            .then(|| p.y as usize * self.width + p.x as usize)
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        let i = self.index_of(p)?;
        Some(&mut self.cells[i])
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            cells: self.cells.into_iter().map(f).collect(),
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = Point> + use<T> {
        let (w, h) = (self.width, self.height);
        (0..h).flat_map(move |y| (0..w).map(move |x| Point::from((x, y))))
    }

    pub fn set(&mut self, p: Point, ch: T) -> bool {
        match self.get_mut(p) {
            Some(cell) => {
                *cell = ch;
                true
            }
            None => false,
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            width,
            height,
            cells: vec![fill; width * height],
        }
    }

    pub fn get(&self, p: Point) -> Option<T> {
        self.index_of(p).map(|i| self.cells[i])
    }

    pub fn find_where<'a>(
        &'a self,
        pred: impl Fn(T) -> bool + 'a,
    ) -> impl Iterator<Item = Point> + 'a {
        self.iter()
            .filter(move |&(_, c)| pred(c))
            .map(|(pos, _)| pos)
    }

    pub fn iter(&self) -> GridIterator<'_, T> {
        GridIterator { grid: self, i: 0 }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find_all(&self, ch: T) -> impl Iterator<Item = Point> + '_ {
        self.find_where(move |c| c == ch)
    }

    pub fn find(&self, ch: T) -> Option<Point> {
        self.find_all(ch).next()
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    i: usize,
}

impl<T: Copy> Iterator for GridIterator<'_, T> {
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        let c = *self.grid.cells.get(self.i)?;
        let pos = Point::from((self.i % self.grid.width, self.i / self.grid.width));
        self.i += 1;
        Some((pos, c))
    }
}

impl FromStr for Grid<char> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = input_lines(input)
            .map(|line| line.trim().chars().collect())
            .collect();
        Grid::try_from(cells)
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        &self.cells[p.y as usize * self.width + p.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        &mut self.cells[p.y as usize * self.width + p.x as usize]
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = String;

    fn try_from(rows: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = rows.len();
        let width = rows.first().map_or(0, Vec::len);
        if let Some((y, row)) = rows.iter().enumerate().find(|(_, row)| row.len() != width) {
            return Err(format!(
                "Row {y} has wrong width: expected {width}, got {}",
                row.len()
            ));
        }
        Ok(Grid {
            width,
            height,
            cells: rows.into_iter().flatten().collect(),
        })
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.width == 0 {
            return Ok(());
        }

        for (i, row) in self.cells.chunks(self.width).enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            for cell in row {
                write!(f, "{cell}")?;
            }
        }
        Ok(())
    }
}

impl<'a, T: Copy> IntoIterator for &'a Grid<T> {
    type Item = (Point, T);
    type IntoIter = GridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Grid {
        "abc\ndef\nghi".parse().unwrap()
    }

    #[test]
    fn test_grid_from_str_valid() {
        let input = "abc\ndef\nghi";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.cells, ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_grid_from_str_wrong_width() {
        let input = "ab\ndef\nghi";
        let err = Grid::from_str(input).unwrap_err();
        assert!(err.contains("Row 1 has wrong width: expected 2, got 3"));
    }

    #[test]
    fn test_grid_from_str_single_cell() {
        let input = "x";
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.cells[0], 'x');
        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
    }

    #[test]
    fn test_iterator() {
        let grid: Grid = "abc\ndef\nghi".parse().unwrap();

        let test: Vec<_> = grid.into_iter().collect();

        assert_eq!(test[0], (Point::new(0, 0), 'a'));
        assert_eq!(test[1], (Point::new(1, 0), 'b'));
        assert_eq!(test[2], (Point::new(2, 0), 'c'));
        assert_eq!(test[3], (Point::new(0, 1), 'd'));
        assert_eq!(test[4], (Point::new(1, 1), 'e'));
        assert_eq!(test[5], (Point::new(2, 1), 'f'));
        assert_eq!(test[6], (Point::new(0, 2), 'g'));
        assert_eq!(test[7], (Point::new(1, 2), 'h'));
        assert_eq!(test[8], (Point::new(2, 2), 'i'));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let grid = sample();
        assert_eq!(grid.get(Point::new(-1, 0)), None);
        assert_eq!(grid.get(Point::new(0, -1)), None);
        assert_eq!(grid.get(Point::new(3, 0)), None);
        assert_eq!(grid.get(Point::new(0, 3)), None);
        assert_eq!(grid.get(Point::new(2, 2)), Some('i'));
    }

    #[test]
    fn test_in_bounds_matches_get() {
        let grid = sample();
        for x in -1..=3 {
            for y in -1..=3 {
                let p = Point::new(x, y);
                assert_eq!(grid.in_bounds(p), grid.get(p).is_some(), "{p:?}");
            }
        }
    }

    #[test]
    fn test_set() {
        let mut grid = sample();
        assert!(grid.set(Point::new(1, 1), 'z'));
        assert_eq!(grid.get(Point::new(1, 1)), Some('z'));
        assert!(!grid.set(Point::new(5, 5), 'z'));
    }

    #[test]
    fn test_index() {
        let mut grid = sample();
        assert_eq!(grid[Point::new(1, 1)], 'e');
        grid[Point::new(1, 1)] = 'z';
        assert_eq!(grid[Point::new(1, 1)], 'z');
    }

    #[test]
    fn test_find_all() {
        let grid: Grid = "a.a\n...\n.a.".parse().unwrap();
        let found: Vec<_> = grid.find_all('a').collect();
        assert_eq!(
            found,
            [Point::new(0, 0), Point::new(2, 0), Point::new(1, 2)]
        );
        assert_eq!(grid.find('a'), Some(Point::new(0, 0)));
    }

    #[test]
    fn test_find_missing() {
        let grid = sample();
        assert_eq!(grid.find_all('z').count(), 0);
        assert_eq!(grid.find('z'), None);
    }

    #[test]
    fn test_find_where() {
        let grid: Grid = "..#\n.#.\n...".parse().unwrap();
        let found: Vec<_> = grid.find_where(|c| c != '.').collect();
        assert_eq!(found, [Point::new(2, 0), Point::new(1, 1)]);
    }

    #[test]
    fn test_display_round_trip() {
        let input = "abc\ndef\nghi";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.to_string(), input);
    }

    #[test]
    fn test_from_str_ignores_surrounding_whitespace() {
        let messy: Grid = "\n   abc\n   def\n   ghi\n\n".parse().unwrap();
        assert_eq!(messy, sample());
    }

    #[test]
    fn test_empty_grid() {
        let grid: Grid = "".parse().unwrap();
        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
        assert_eq!(grid.to_string(), "");
        assert_eq!(grid.iter().count(), 0);
    }

    #[test]
    fn test_new_bool_grid() {
        let grid = Grid::new(3, 2, false);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert!(grid.iter().all(|(_, on)| !on));
    }

    #[test]
    fn test_generic_u32_grid() {
        let mut grid = Grid::<u32>::new(2, 2, 0);
        grid[Point::new(1, 0)] = 5;
        assert_eq!(grid.iter().map(|(_, v)| v).sum::<u32>(), 5);
        assert_eq!(grid.find(5), Some(Point::new(1, 0)));
        assert_eq!(grid.to_string(), "05\n00");
    }
}
