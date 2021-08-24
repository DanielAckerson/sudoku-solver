use std::fmt;
use std::ops::{Index, IndexMut};
use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct Board {
    cells: Vec<isize>,
    pub degree: usize,
}

pub struct Rows<'a> {
    board: &'a Board,
    index: usize,
}

pub struct Cols<'a> {
    board: &'a Board,
    index: usize,
}

pub struct Zones<'a> {
    board: &'a Board,
    index: usize,
}

impl Board {
    pub fn new(degree: usize) -> Board {
        Board { cells: vec![0; degree], degree: degree }
    }

    pub fn rows(&self) -> Rows {
        Rows { board: &self, index: 0 }
    }

    pub fn cols(&self) -> Cols {
        Cols { board: &self, index: 0 }
    }

    pub fn zones(&self) -> Zones {
        Zones { board: &self, index: 0 }
    }

    pub fn simple_display(&self) -> String {
        let mut text = "".to_owned();

        for row in self.rows() {
            text.push_str(
                &format!("{}\n", row
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<String>()));
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

impl<'a> Iterator for Rows<'a> {
    type Item = Vec<&'a isize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.board.degree {
            let begin = self.index * self.board.degree;
            let end = begin + self.board.degree;

            self.index += 1;

            Some(self.board.cells[begin..end]
                .into_iter()
                .collect())

        } else {
            None
        }
    }
}

impl<'a> Iterator for Cols<'a> {
    type Item = Vec<&'a isize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.board.degree {
            let begin = self.index;
            self.index += 1;

            Some(self.board.cells[begin..]
                .into_iter()
                .step_by(self.board.degree)
                .collect())

        } else {
            None
        }
    }
}

impl<'a> Iterator for Zones<'a> {
    type Item = Vec<&'a isize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.board.degree {
            let zsize = (self.board.degree as f64).sqrt() as usize;
            let col_offset = zsize * (self.index % zsize);
            let row_pos = |offset| (zsize * (self.index / zsize) + offset) * self.board.degree;
            let begin: Vec<usize> = (0..3)
                .map(|x| row_pos(x) + col_offset)
                .collect();

            self.index += 1;

            Some(begin
                .into_iter()
                .map(|x| self.board
                    .cells[x..x + zsize]
                    .into_iter())
                .flatten()
                .collect())

        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unsolved_board() -> Board {
        Board::from(vec![
            4, 6, 2, /**/ 5, 0, 0, /**/ 0, 3, 8,
            7, 9, 3, /**/ 8, 0, 2, /**/ 5, 0, 4,
            0, 8, 0, /**/ 0, 0, 4, /**/ 6, 0, 0,
            /**********************************/
            0, 5, 0, /**/ 0, 2, 9, /**/ 1, 8, 6,
            0, 0, 1, /**/ 6, 5, 8, /**/ 3, 2, 7,
            6, 2, 0, /**/ 0, 3, 0, /**/ 0, 0, 0,
            /**********************************/
            8, 0, 9, /**/ 0, 0, 0, /**/ 2, 0, 0,
            2, 3, 4, /**/ 9, 0, 0, /**/ 8, 0, 0,
            5, 0, 0, /**/ 0, 0, 0, /**/ 9, 4, 1,
        ])
    }

    #[test]
    fn test_zones() {
        let board = unsolved_board();

        let zones: Vec<Vec<isize>> = vec![
            vec![4, 6, 2, 7, 9, 3, 0, 8, 0,], /**/
            vec![5, 0, 0, 8, 0, 2, 0, 0, 4,], /**/
            vec![0, 3, 8, 5, 0, 4, 6, 0, 0,],
                 /************************/
            vec![0, 5, 0, 0, 0, 1, 6, 2, 0,], /**/
            vec![0, 2, 9, 6, 5, 8, 0, 3, 0,], /**/
            vec![1, 8, 6, 3, 2, 7, 0, 0, 0,],
                 /************************/
            vec![8, 0, 9, 2, 3, 4, 5, 0, 0,], /**/
            vec![0, 0, 0, 9, 0, 0, 0, 0, 0,], /**/
            vec![2, 0, 0, 8, 0, 0, 9, 4, 1,],
        ];

        board.zones()
            .zip(zones)
            .for_each(|(output, truth)| {
                println!("{:?}    {:?}", output, truth);
                assert_eq!(
                    output.into_iter()
                        .cloned()
                        .collect::<Vec<isize>>(),
                    truth);
        });
    }

    #[test]
    fn test_idx() {
        let board = unsolved_board();
        let zsize = (board.degree as f64).sqrt() as usize;
        let mut zone_idx: Vec<Vec<usize>> = Vec::new();

        for i in 0..9 {
            let col_offset = zsize * (i % zsize);
            let row_offset = |x| (zsize * (i / zsize) + x) * board.degree;
            let begin: Vec<usize> = (0..3)
                .map(|x| row_offset(x) + col_offset)
                .collect();

            println!("{:?}", begin);
            zone_idx.push(begin);
        }

        assert_eq!(vec![
            vec![ 0,  9, 18],
            vec![ 3, 12, 21],
            vec![ 6, 15, 24],

            vec![27, 36, 45],
            vec![30, 39, 48],
            vec![33, 42, 51],

            vec![54, 63, 72],
            vec![57, 66, 75],
            vec![60, 69, 78],
        ], zone_idx)
    }
}
