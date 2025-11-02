use core::fmt;
use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn find(&self, ch: char) -> Option<(usize, usize)> {
        for (y, r) in self.cells.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if *c == ch {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn change(&mut self, pos: (usize, usize), ch: char) {
        if pos.0 < self.width() && pos.1 < self.height() {
            self.cells[pos.1][pos.0] = ch;
        }
    }

    pub fn inside(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.width() && pos.1 < self.height()
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = ((usize, usize), &'a char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let current_pos = (self.x, self.y);
        let current_char = &self.grid[self.y][self.x];

        self.x += 1;
        if self.x >= self.grid.width() {
            self.y += 1;
            self.x = 0;
        }

        Some((current_pos, current_char))
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.trim().lines();
        let height = lines.count();

        let mut cells = Vec::with_capacity(height);

        let mut lines = input.trim().lines();
        let mut width = None;

        for y in 0..height {
            let line = lines.next().unwrap();
            let chars: Vec<char> = line.trim().chars().collect();
            if let Some(w) = width {
                if chars.len() != w {
                    return Err(format!(
                        "Line {} has wrong width: expected {}, got {}",
                        y,
                        w,
                        chars.len()
                    ));
                }
            } else {
                width = Some(chars.len());
            }
            cells.push(chars);
        }
        Ok(Grid { cells })
    }
}

impl Index<usize> for Grid {
    type Output = [char];
    fn index(&self, row: usize) -> &Self::Output {
        &self.cells[row]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.cells[row]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            let row_string: String = row.iter().collect();
            write!(f, "{}", row_string)?;
            if i < self.height() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = ((usize, usize), &'a char);
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_str_valid() {
        let input = "abc\ndef\nghi";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.cells[0], ['a', 'b', 'c']);
        assert_eq!(grid.cells[1], ['d', 'e', 'f']);
        assert_eq!(grid.cells[2], ['g', 'h', 'i']);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_grid_from_str_wrong_width() {
        let input = "ab\ndef\nghi";
        let err = Grid::from_str(input).unwrap_err();
        assert!(err.contains("Line 1 has wrong width: expected 2, got 3"));
    }

    #[test]
    fn test_grid_from_str_single_cell() {
        let input = "x";
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.cells[0][0], 'x');
        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 1);
    }

    #[test]
    fn test_iterator() {
        let grid: Grid = "abc\ndef\nghi".parse().unwrap();

        let test: Vec<_> = grid.into_iter().collect();

        assert_eq!(test[0], ((0, 0), &'a'));
        assert_eq!(test[1], ((1, 0), &'b'));
        assert_eq!(test[2], ((2, 0), &'c'));
        assert_eq!(test[3], ((0, 1), &'d'));
        assert_eq!(test[4], ((1, 1), &'e'));
        assert_eq!(test[5], ((2, 1), &'f'));
        assert_eq!(test[6], ((0, 2), &'g'));
        assert_eq!(test[7], ((1, 2), &'h'));
        assert_eq!(test[8], ((2, 2), &'i'));
    }
}
