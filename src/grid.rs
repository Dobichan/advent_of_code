use std::{ops::Index, str::FromStr};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_str_valid() {
        let input = "abc\ndef\nghi";
        let grid = Grid::from_str(input).unwrap();
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
}
