use std::collections::HashMap;
use crate::game::board::Piece;
use rand::rngs::SmallRng;
use::rand::distributions::Standard;
use borsh::{BorshSerialize, BorshDeserialize};


// Description of the player:
// - Has a table of numbers, one for each possible state of the game. Each
//   number will be the latest estimate of the probability of the player winning
//   the game from that state
// - For all states where our player wins (3 x's/o's in a row), the value is 1
// - For states where out player wins (other player wins, or full board no winner)
//   the value is 0
// - All other probabilities are initialized to 0.5
// - Most of the time the player acts greedily, moving to the position with the
//   greatest value
// - Occasionally, the player selects randomly from the other moves instead
// - After each greedy move, the previous state is updated, moving it a little closer to the
//   state after the greedy move (v(s_t)<- v(s_t)+a[v(s_{t+1})-v(s_t)], where a is the learning
//   rate, v(s_t) is the value of the state at time t, and v(s_{t+1}) is the value of the state
//   at time t+1

/// Struct representing the "savable" part of the player
#[derive(BorshSerialize, BorshDeserialize)]
struct SaveState {
    /// Which piece the player uses
    piece: Piece,
    /// The states and probability of winning from each (modification of this is how learning occurs)
    state_space: HashMap<[Piece;9], f64>,
    /// How fast the probabilities of winning from a position are updated
    learning_rate: f64,
    /// How often a less than optimum choice is made
    exploration_rate:f64,
    /// Number of games played (used to taper the learning rate)
    iteration: u32,
}



/// Struct representing the computer "Player"
pub struct Player{
    /// The savable state of the player
    save_state: SaveState,
    /// Function to update the learning rate over time, takes in the current learning rate
    /// and the iteration and returns a new learning rate
    learning_annealing_function: fn(f64, u32)->f64,
    /// Function to update the exploration rate over time, takes in the current exploration rate
    /// and the iteration, and returns a new exploration rate
    exploration_annealing_function: fn(f64, u32)->f64,
    /// Random number generator used by the player to make decisions
    generator: SmallRng,
}

impl Player {

    fn check_winner(compact_state: &[Piece;9])->Option<Piece>{
        match Self::check_winner_row(compact_state){
            None => {
                match Self::check_winner_col(compact_state){
                    None => {
                        Self::check_winner_diag(compact_state)
                    }
                    Some(piece) => {Some(piece)}
                }
            }
            Some(piece) => {Some(piece)}
        }
    }
    fn check_winner_col(compact_state: &[Piece;9])->Option<Piece>{
        for col in 0..3{
            if compact_state[col+0] == compact_state[col+3] &&
                compact_state[col+0] == compact_state[col+6] &&
                compact_state[col+0] != Piece::Empty{
                return Some(compact_state[col+0]);
            }
        }
        None
    }

    fn check_winner_row(compact_state: &[Piece;9])->Option<Piece>{
        for row in 0..3 {
            if compact_state[3*row+0] == compact_state[3*row+1] &&
                compact_state[3*row+0] == compact_state[3*row+2] &&
                compact_state[3*row+0] != Piece::Empty {
                return Some(compact_state[3*row+0])
            }
        }
        None
    }

    fn check_winner_diag(compact_state: &[Piece;9])->Option<Piece>{
        if compact_state[3*0+0] == compact_state[3*1+1] &&
            compact_state[3*0+0] == compact_state[3*2+2] &&
            compact_state[3*0+0] != Piece::Empty {
            return Some(compact_state[3*0+0])
        }

        if compact_state[3*2+0] == compact_state[3*1+1] &&
            compact_state[3*2+0] == compact_state[3*0+2] &&
            compact_state[3*0+0] != Piece::Empty {
            return Some(compact_state[3*0+0])
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::agents::players::Player;
    use crate::game::board::Piece;

    #[test]
    fn test_check_winner_col(){
        let test_board: [Piece;9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_col(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::O,
            Piece::O, Piece::X, Piece::X,
        ];
        assert_eq!(Player::check_winner_col(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_col(&test_board), Some(Piece::X));
    }

    #[test]
    fn test_check_winner_row(){
        let test_board: [Piece;9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_row(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::O, Piece::O,
        ];
        assert_eq!(Player::check_winner_row(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::X, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::O, Piece::O,
        ];
        assert_eq!(Player::check_winner_row(&test_board), Some(Piece::X));
    }

    #[test]
    fn check_winner_diag(){
        let test_board: [Piece;9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::X, Piece::X,
            Piece::O, Piece::O, Piece::O,
            Piece::X, Piece::X, Piece::X,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::O, Piece::O,
            Piece::O, Piece::X, Piece::O,
            Piece::O, Piece::O, Piece::X,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), Some(Piece::X));
    }

    #[test]
    fn test_check_winner(){
        let test_board: [Piece;9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::X, Piece::O,
        ];
        assert_eq!(Player::check_winner(&test_board), None);
        let test_board: [Piece;9] = [
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner(&test_board), Some(Piece::X));
    }
}