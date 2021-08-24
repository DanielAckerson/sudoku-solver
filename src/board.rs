use std::fmt;
use std::ops::{Index, IndexMut};
use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct Board {
    cells: Vec<isize>,
    pub degree: usize,
}

impl Board {
    pub fn new(degree: usize) -> Board {
        Board { cells: vec![0; degree], degree: degree }
    }

    pub fn simple_display(&self) -> String {
        let mut text = "".to_owned();

        for y in 0..9 {
            for x in 0..9 {
                let mut cell = self[(x, y)];

                if cell > 9 {
                    cell -= 10;
                }

                text.push_str(&cell.to_string());
            }
            text.push('\n');
        }

        text
    }
}

impl From<Vec<isize>> for Board {
    // TODO: change this to something else. From<_> must always succeed but we can't guarantee that
    // item's size will be a square number
    fn from(item: Vec<isize>) -> Self {
        let degree = (item.len() as f64).sqrt() as usize;
        // TODO: validate cell values?

        Board { cells: item.clone(), degree: degree }
    }
}

impl Index<usize> for Board {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl Index<(usize, usize)> for Board {
    type Output = isize;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.cells[pos.1 + pos.0 * 9]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[pos.1 + pos.0 * 9]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}
