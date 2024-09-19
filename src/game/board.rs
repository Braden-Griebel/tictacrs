use std::fmt;
use std::fmt::format;
use std::io::Write;

#[derive(Copy, Debug, Clone)]
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
    Squares: [[Piece; 3]; 3],
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
                self.Squares[0][0], self.Squares[0][1], self.Squares[0][2],
                self.Squares[1][0], self.Squares[1][1], self.Squares[1][2],
                self.Squares[2][0], self.Squares[2][1], self.Squares[2][2],
            )
        );
        write!(f, "{}", repr)
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.Squares == other.Squares
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            Squares: [[Piece::Empty, Piece::Empty, Piece::Empty],
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
            _ => {return Err(BoardError::InvalidMove)},
        };
        let col: usize = match move_specification_chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => {return Err(BoardError::InvalidMove)},
        };
        self.make_move(row, col, piece_specification)?;
        Ok(())
    }

    fn make_move(&mut self, row: usize, col: usize, val: &str) -> Result<(), BoardError> {
        match self.Squares[row][col] {
            Piece::Empty => {},
            Piece::X => {return Err(BoardError::NotEmpty)},
            Piece::O => {return Err(BoardError::NotEmpty)},
        }
        match val {
            "X" | "x" => {
                self.Squares[row][col] = Piece::X;
                Ok(())
            }
            "O" | "o" => {
                self.Squares[row][col] = Piece::O;
                Ok(())
            }
            _ => {Err(BoardError::InvalidPiece)}
        }
    }

    pub fn check_winner(&self) -> Option<Piece> {
        if let Some(winner) = self.check_winner_col() {
            return Some(winner);
        }
        if let Some(winner) = self.check_winner_row() {
            return Some(winner);
        }
        if let Some(winner) = self.check_winner_diagonal() {
            return Some(winner);
        }
        None
    }

    fn check_winner_col(&self) -> Option<Piece> {
        for col in 0usize..3 {
            if self.Squares[0][col].eq(&self.Squares[1][col]) &&
                self.Squares[0][col].eq(&self.Squares[2][col]) &&
                !self.Squares[0][col].eq(&Piece::Empty) {
                return Some(self.Squares[0][col]);
            }
        }
        None
    }
    fn check_winner_row(&self) -> Option<Piece> {
        for row in 0usize..3 {
            if self.Squares[row][0].eq(&self.Squares[row][1]) &&
                self.Squares[row][0].eq(&self.Squares[row][2]) &&
                !self.Squares[row][0].eq(&Piece::Empty){
                return Some(self.Squares[row][0]);
            }
        }
        None
    }

    fn check_winner_diagonal(&self) -> Option<Piece> {
        if self.Squares[0][0].eq(&self.Squares[1][1]) &&
            self.Squares[0][0].eq(&self.Squares[2][2]) &&
            !self.Squares[0][0].eq(&Piece::Empty){
            return Some(self.Squares[0][0]);
        }
        if self.Squares[2][0].eq(&self.Squares[1][1]) &&
            self.Squares[2][0].eq(&self.Squares[0][2]) &&
            !self.Squares[2][0].eq(&Piece::Empty) {
            return Some(self.Squares[2][0]);
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
    fn test_board_creation(){
        _ = Board::new();
    }

    #[test]
    fn test_make_move()->Result<(), BoardError>{
        let mut test_board = Board::new();
        test_board.make_move(1,1, "x")?;
        assert_eq!(test_board.Squares[1][1], Piece::X);
        assert_eq!(test_board.Squares[1][2], Piece::Empty);
        Ok(())
    }

    #[test]
    fn test_player_move()->Result<(), BoardError>{
        let mut test_board = Board::new();
        test_board.player_move("b2", "X")?;
        assert_eq!(test_board.Squares[1][1], Piece::X);
        assert_eq!(test_board.Squares[1][2], Piece::Empty);
        Ok(())
    }

    #[test]
    fn test_nonempty_move(){
        let mut test_board = Board::new();
        _=test_board.player_move("c1", "o");
        let res = test_board.player_move("c1", "o");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::NotEmpty));
    }

    #[test]
    fn test_invalid_piece(){
        let mut test_board = Board::new();
        let res = test_board.player_move("c2", "z");
        assert!(res.is_err());
        assert_eq!(res, Err(BoardError::InvalidPiece));
    }

    #[test]
    fn test_invalid_move(){
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
    fn test_check_winner(){
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
}
