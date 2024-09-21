use std::fmt;
use std::fmt::format;
use std::io::Write;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Copy, Debug, Clone, Hash, BorshSerialize, BorshDeserialize, PartialOrd, Eq,  Ord)]
pub enum Piece {
    Empty,
    X,
    O,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Empty => { write!(f, " ") }
            Piece::X => { write!(f, "X") }
            Piece::O => { write!(f, "O") }
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Piece::Empty => {
                match other {
                    Piece::Empty => true,
                    Piece::X => false,
                    Piece::O => false,
                }
            }
            Piece::X => {
                match other {
                    Piece::Empty => false,
                    Piece::X => true,
                    Piece::O => false,
                }
            }
            Piece::O => {
                match other {
                    Piece::Empty => false,
                    Piece::X => false,
                    Piece::O => true,
                }
            }
        }
    }
}

pub struct Board {
    squares: [[Piece; 3]; 3],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        repr.push_str(
            &format!(
                "
     1   2   3
       |   |
a    {} | {} | {}
    ___|___|___
       |   |
b    {} | {} | {}
    ___|___|___
       |   |
c    {} | {} | {}
       |   |   \n",
                self.squares[0][0], self.squares[0][1], self.squares[0][2],
                self.squares[1][0], self.squares[1][1], self.squares[1][2],
                self.squares[2][0], self.squares[2][1], self.squares[2][2],
            )
        );
        write!(f, "{}", repr)
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.squares == other.squares
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [[Piece::Empty, Piece::Empty, Piece::Empty],
                [Piece::Empty, Piece::Empty, Piece::Empty],
                [Piece::Empty, Piece::Empty, Piece::Empty], ]
        }
    }

    pub fn player_move(&mut self, move_specification: &str, piece_specification: &str) -> Result<(), BoardError> {
        let move_specification_chars: Vec<char> = move_specification.chars().collect();
        let row: usize = match move_specification_chars[0] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            _ => { return Err(BoardError::InvalidMove) }
        };
        let col: usize = match move_specification_chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => { return Err(BoardError::InvalidMove) }
        };
        self.make_move(row, col, piece_specification)?;
        Ok(())
    }

    fn make_move(&mut self, row: usize, col: usize, val: &str) -> Result<(), BoardError> {
        match self.squares[row][col] {
            Piece::Empty => {}
            Piece::X => { return Err(BoardError::NotEmpty) }
            Piece::O => { return Err(BoardError::NotEmpty) }
        }
        match val {
            "X" | "x" => {
                self.squares[row][col] = Piece::X;
                Ok(())
            }
            "O" | "o" => {
                self.squares[row][col] = Piece::O;
                Ok(())
            }
            _ => { Err(BoardError::InvalidPiece) }
        }
    }

    /// Make a move using a Piece object instead of a str
    pub(crate) fn make_auto_player_move(&mut self, row:u8, col:u8, piece: Piece){
        self.squares[row as usize][col as usize] = piece;
    }

    pub fn clear_board(&mut self){
        for row in 0..3{
            for col in 0..3{
                self.squares[row][col] = Piece::Empty;
            }
        }
    }

    pub fn get_compact_state(&self) -> [Piece; 9] {
        let mut compact_state = [Piece::Empty; 9];
        for row in 0..3 {
            for col in 0..3 {
                compact_state[3 * row + col] = self.squares[row][col];
            }
        }
        compact_state
    }

    /// Check if the board is full, returns true if the board is full, and false otherwise
    pub fn is_full(&self)->bool{
        for row in 0..3{
            for col in 0..3{
                if self.squares[row][col]==Piece::Empty{
                    return false
                }
            }
        }
        true
    }

    /// Determine if there is a winner, if neither player has won return None
    pub fn check_winner(&self) -> Option<Piece> {
        if let Some(winner) = self.check_winner_col() {
            return Some(winner);
        }
        if let Some(winner) = self.check_winner_row() {
            return Some(winner);
        }
        if let Some(winner) = self.check_winner_diag() {
            return Some(winner);
        }
        None
    }

    fn check_winner_col(&self) -> Option<Piece> {
        for col in 0usize..3 {
            if self.squares[0][col].eq(&self.squares[1][col]) &&
                self.squares[0][col].eq(&self.squares[2][col]) &&
                !self.squares[0][col].eq(&Piece::Empty) {
                return Some(self.squares[0][col]);
            }
        }
        None
    }
    fn check_winner_row(&self) -> Option<Piece> {
        for row in 0usize..3 {
            if self.squares[row][0].eq(&self.squares[row][1]) &&
                self.squares[row][0].eq(&self.squares[row][2]) &&
                !self.squares[row][0].eq(&Piece::Empty) {
                return Some(self.squares[row][0]);
            }
        }
        None
    }

    fn check_winner_diag(&self) -> Option<Piece> {
        if self.squares[0][0].eq(&self.squares[1][1]) &&
            self.squares[0][0].eq(&self.squares[2][2]) &&
            !self.squares[0][0].eq(&Piece::Empty) {
            return Some(self.squares[0][0]);
        }
        if self.squares[2][0].eq(&self.squares[1][1]) &&
            self.squares[2][0].eq(&self.squares[0][2]) &&
            !self.squares[2][0].eq(&Piece::Empty) {
            return Some(self.squares[2][0]);
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum BoardError {
    NotEmpty,
    InvalidPiece,
    InvalidMove,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        _ = Board::new();
    }

    #[test]
    fn test_make_move() -> Result<(), BoardError> {
        let mut test_board = Board::new();
        test_board.make_move(1, 1, "x")?;
        assert_eq!(test_board.squares[1][1], Piece::X);
        assert_eq!(test_board.squares[1][2], Piece::Empty);
        Ok(())
    }

    #[test]
    fn test_player_move() -> Result<(), BoardError> {
        let mut test_board = Board::new();
        test_board.player_move("b2", "X")?;
        assert_eq!(test_board.squares[1][1], Piece::X);
        assert_eq!(test_board.squares[1][2], Piece::Empty);
        Ok(())
    }

    #[test]
    fn test_nonempty_move() {
        let mut test_board = Board::new();
        _ = test_board.player_move("c1", "o");
        let res = test_board.player_move("c1", "o");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::NotEmpty));
    }

    #[test]
    fn test_invalid_piece() {
        let mut test_board = Board::new();
        let res = test_board.player_move("c2", "z");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::InvalidPiece));
    }

    #[test]
    fn test_invalid_move() {
        let mut test_board = Board::new();
        let res = test_board.player_move("z2", "o");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::InvalidMove));

        let mut test_board = Board::new();
        let res = test_board.player_move("c5", "o");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::InvalidMove));
    }

    #[test]
    fn test_check_winner() {
        let mut test_board = Board::new();
        assert_eq!(test_board.check_winner(), None);
        test_board.player_move("a1", "o").unwrap();
        test_board.player_move("a2", "o").unwrap();
        test_board.player_move("a3", "o").unwrap();
        assert_eq!(test_board.check_winner_row(), Some(Piece::O));
        assert_eq!(test_board.check_winner(), Some(Piece::O));

        let mut test_board = Board::new();
        assert_eq!(test_board.check_winner(), None);
        test_board.player_move("a1", "o").unwrap();
        test_board.player_move("b1", "o").unwrap();
        test_board.player_move("c1", "o").unwrap();
        assert_eq!(test_board.check_winner_col(), Some(Piece::O));
        assert_eq!(test_board.check_winner(), Some(Piece::O));
    }

    #[test]
    fn test_compact_representation() {
        let mut test_board = Board::new();
        assert_eq!(test_board.get_compact_state(), [Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty]);
        test_board.player_move("c2", "X").unwrap();
        assert_eq!(test_board.get_compact_state(),
                   [
                       Piece::Empty, Piece::Empty, Piece::Empty,
                       Piece::Empty, Piece::Empty, Piece::Empty,
                       Piece::Empty, Piece::X, Piece::Empty,
                   ]);
        test_board.player_move("a1", "O").unwrap();
        assert_eq!(test_board.get_compact_state(),
                   [
                       Piece::O, Piece::Empty, Piece::Empty,
                       Piece::Empty, Piece::Empty, Piece::Empty,
                       Piece::Empty, Piece::X, Piece::Empty,
                   ]);
        test_board.player_move("a3", "X").unwrap();
        assert_eq!(test_board.get_compact_state(),
                   [
                       Piece::O, Piece::Empty, Piece::X,
                       Piece::Empty, Piece::Empty, Piece::Empty,
                       Piece::Empty, Piece::X, Piece::Empty,
                   ]);
        test_board.player_move("b2", "O").unwrap();
        assert_eq!(test_board.get_compact_state(),
                   [
                       Piece::O, Piece::Empty, Piece::X,
                       Piece::Empty, Piece::O, Piece::Empty,
                       Piece::Empty, Piece::X, Piece::Empty,
                   ]);
    }
}
