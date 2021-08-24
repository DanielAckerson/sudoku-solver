use super::board::Board;

use std::collections::BTreeSet;

type Tags = Vec<BTreeSet<isize>>;

// TODO: Do I actually want to make another type?
// That means more boiler-plate code like in board.rs
struct ScratchBoard {
    pub board: Board,
    pub tags: Tags,
}

pub fn solve(board: &Board) -> Option<Board> {
    // let mut sb = ScratchBoard { board: board

    Some(Board::new(9))
}

fn is_valid<'a, I>(vals: I, board: &Board) -> bool
where
    I: IntoIterator<Item = &'a isize>,
{
    let mut values_found: BTreeSet<isize> = BTreeSet::new();

    for val in vals {
        if *val == 0 {
            continue;
        } else if
                *val < 0
                || *val > board.degree as isize + 1
                || values_found.contains(val) {
            return false;
        } else {
            values_found.insert(*val);
        }
    }

    true
}

pub fn is_board_valid(board: Board) -> bool {
    for row in board.rows() {
        if !is_valid(row, &board) {
            return false;
        }
    }

    for col in board.cols() {
        if !is_valid(col, &board) {
            return false;
        }
    }

    for zone in board.zones() {
        if !is_valid(zone, &board) {
            return false;
        }
    }

    true
}

pub fn is_board_solved(board: Board) -> bool {
    let solved_set: BTreeSet<isize> = (1..board.degree as isize + 1).collect();

    for row in board.rows() {
        let row_set: BTreeSet<isize> = row.into_iter().cloned().collect();
        let diff: BTreeSet<isize> = solved_set.difference(&row_set).cloned().collect();

        if !diff.is_empty() {
            return false;
        }
    }

    for col in board.cols() {
        let col_set: BTreeSet<isize> = col.into_iter().cloned().collect();
        let diff: BTreeSet<isize> = solved_set.difference(&col_set).cloned().collect();

        if !diff.is_empty() {
            return false;
        }
    }

    for zone in board.zones() {
        let zone_set: BTreeSet<isize> = zone.into_iter().cloned().collect();
        let diff: BTreeSet<isize> = solved_set.difference(&zone_set).cloned().collect();

        if !diff.is_empty() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    enum BoardIterType {
        Rows_,
        Cols_,
        Zones_,
    }

    fn invalid_board(bIterType: BoardIterType) -> Board {
        let mut board = Board::new(9);

        match bIterType {
            Rows_ => {
                // 1 0 0 /**/ 1 0 0
                board[(0, 0)] = 1;
                board[(0, 3)] = 1;
            },
            Cols_ => {
                board[(3, 4)] = 1; // 0 1 0
                                   // /***/
                board[(6, 4)] = 1; // 0 1 0
            },
            Zones_ => {
                board[(6, 3)] = 1; // 1 0 0
                board[(7, 4)] = 1; // 0 1 0
            },
        }

        board
    }

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

    fn solved_board() -> Board {
        Board::from(vec![
            4, 6, 2, /**/ 5, 9, 1, /**/ 7, 3, 8,
            7, 9, 3, /**/ 8, 6, 2, /**/ 5, 1, 4,
            1, 8, 5, /**/ 3, 7, 4, /**/ 6, 9, 2,
            /**********************************/
            3, 5, 7, /**/ 4, 2, 9, /**/ 1, 8, 6,
            9, 4, 1, /**/ 6, 5, 8, /**/ 3, 2, 7,
            6, 2, 8, /**/ 1, 3, 7, /**/ 4, 5, 9,
            /**********************************/
            8, 1, 9, /**/ 7, 4, 5, /**/ 2, 6, 3,
            2, 3, 4, /**/ 9, 1, 6, /**/ 8, 7, 5,
            5, 7, 6, /**/ 2, 8, 3, /**/ 9, 4, 1,
        ])
    }

    #[test]
    fn test_solve() {
        let unsolved = unsolved_board();
        let solved = solved_board();

        assert_eq!(solve(unsolved).unwrap(), solved)
    }

    #[test]
    fn test_is_board_valid() {
        assert_eq!(is_board_valid(unsolved_board()), true);
        assert_eq!(is_board_valid(solved_board()), true);
        assert_eq!(is_board_valid(invalid_board(BoardIterType::Rows_)), false);
        assert_eq!(is_board_valid(invalid_board(BoardIterType::Cols_)), false);
        assert_eq!(is_board_valid(invalid_board(BoardIterType::Zones_)), false);
    }

    #[test]
    fn test_is_board_solved() {
        assert_eq!(is_board_solved(unsolved_board()), false);
        assert_eq!(is_board_solved(solved_board()), true);
    }
}
