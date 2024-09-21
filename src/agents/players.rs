use crate::game::board::Piece;
use borsh::{BorshDeserialize, BorshSerialize};
use rand::distributions::Standard;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
/*
Description of the player:
- Has a table of numbers, one for each possible state of the game. Each
  number will be the latest estimate of the probability of the player winning
  the game from that state
- For all states where our player wins (3 x's/o's in a row), the value is 1
- For states where out player wins (other player wins, or full board no winner)
  the value is 0
- All other probabilities are initialized to 0.5
- Most of the time the player acts greedily, moving to the position with the
  greatest value
- Occasionally, the player selects randomly from the other moves instead
- After each greedy move, the previous state is updated, moving it a little closer to the
  state after the greedy move (v(s_t)<- v(s_t)+a[v(s_{t+1})-v(s_t)], where a is the learning
  rate, v(s_t) is the value of the state at time t, and v(s_{t+1}) is the value of the state
  at time t+1
 */

/// Struct representing the "savable" part of the player
#[derive(BorshSerialize, BorshDeserialize)]
struct SaveState {
    /// Which piece the player uses
    piece: Piece,
    /// The states and probability of winning from each (modification of this is how learning occurs)
    state_space: HashMap<[Piece; 9], f64>,
    /// How fast the probabilities of winning from a position are updated
    initial_learning_rate: f64,
    /// How often a less than optimum choice is made
    initial_exploration_rate: f64,
    /// Number of games played (used to taper the learning rate)
    iteration: u32,
}


/// Struct representing the computer "Player"
pub struct Player {
    /// The savable state of the player
    save_state: SaveState,
    /// Function to update the learning rate over time, takes in the current learning rate
    /// and the iteration and returns a new learning rate
    learning_annealing_function: fn(f64, u32) -> f64,
    /// Function to update the exploration rate over time, takes in the current exploration rate
    /// and the iteration, and returns a new exploration rate
    exploration_annealing_function: fn(f64, u32) -> f64,
    /// Random number generator used by the player to make decisions
    generator: SmallRng,
}

struct PotentialMoves {
    /// Describes the row and column of the potential next move
    next_moves: Vec<[u8; 2]>,
    /// Win probabilities for each of the moves
    probabilities: Vec<f64>,
}

impl Player {
    /// Create a new player
    pub fn new(piece: Piece, initial_learning_rate: f64, initial_exploration_rate: f64,
               learning_annealing_function: fn(f64, u32) -> f64,
               exploration_annealing_function: fn(f64, u32) -> f64, ) -> Player {
        Player {
            save_state: SaveState {
                piece,
                state_space: HashMap::new(),
                initial_learning_rate,
                initial_exploration_rate,
                iteration: 0,
            },
            learning_annealing_function,
            exploration_annealing_function,
            generator: SmallRng::from_entropy(),
        }
    }

    /// Get which piece the player plays
    pub fn get_player_piece(&self) -> Piece {
        self.save_state.piece
    }

    /// Read in a player save state from a file, additionally requires the learning and
    /// exploration annealing functions (as those can't be serialized).
    pub fn new_from_file<P: AsRef<Path>>(&self, file_path: P,
                                         learning_annealing_function: fn(f64, u32) -> f64,
                                         exploration_annealing_function: fn(f64, u32) -> f64,
    ) -> Result<Player, PlayerError> {
        let file = match File::open(file_path) {
            Ok(f) => { f }
            Err(_) => { return Err(PlayerError::InvalidFile) }
        };
        let mut reader = BufReader::new(file);
        let save_state: SaveState = match borsh::de::from_reader(&mut reader) {
            Ok(p) => p,
            Err(_) => { return Err(PlayerError::UnableToRead) }
        };

        Ok(Player {
            save_state,
            learning_annealing_function,
            exploration_annealing_function,
            generator: SmallRng::from_entropy(),
        })
    }

    /// Save the player data to a file
    pub fn save_player_state<P: AsRef<Path>>(&self, file_path: P) -> Result<(), PlayerError> {
        let file = match File::create(file_path) {
            Ok(f) => { f }
            Err(_) => { return Err(PlayerError::InvalidFile) }
        };
        let mut writer = BufWriter::new(file);
        match borsh::to_writer(&mut writer, &self.save_state) {
            Ok(_) => {}
            Err(_) => {
                return Err(PlayerError::UnableToSave);
            }
        };
        Ok(())
    }

    /// Given a board state, determine which move to make
    pub fn make_move(&mut self, board_state: &[Piece; 9]) -> [u8; 2] {
        // First, choose whether this move will be optimal, or exploratory
        let rand_val: f64 = self.generator.sample(Standard);
        let exp_rate = (self.exploration_annealing_function)(self.save_state.initial_exploration_rate, self.save_state.iteration);
        if rand_val < exp_rate {
            // Make an exploratory move
            self.make_random_move(board_state)
        } else {
            // Make an optimal move
            self.make_optimal_move(board_state)
        }
    }

    /// Update which iteration is the current one
    pub fn update_iteration(&mut self, new_iter: u32) {
        // Update the iteration value itself
        self.save_state.iteration = new_iter;
    }

    /// Choose the optimal move (or choose randomly from equivalent moves)
    fn make_optimal_move(&mut self, compact_state: &[Piece; 9]) -> [u8; 2] {
        // Variables to hold the current max probability, and
        let mut max_probability: f64 = 0.;
        let mut best_moves: Vec<[u8; 2]> = Vec::with_capacity(9usize);
        // Get all the possible moves
        let potential_moves = self.get_potential_moves(compact_state);
        for idx in 0..potential_moves.next_moves.len() {
            if potential_moves.probabilities[idx] > max_probability {
                // Found a new best probability, so clear all other moves
                best_moves.clear();
                max_probability = potential_moves.probabilities[idx];
                best_moves.push(potential_moves.next_moves[idx]);
            } else if potential_moves.probabilities[idx] == max_probability {
                best_moves.push(potential_moves.next_moves[idx]);
            }
        }
        // Update the state space
        // First check if the current position is in the state space,
        // assigning it a value if needed
        if !self.save_state.state_space.contains_key(compact_state) {
            self.save_state.state_space.insert(*compact_state, self.find_new_state_prob(compact_state));
        }
        let old_prob = self.save_state.state_space.get(compact_state).unwrap().clone();
        let lrate = (self.learning_annealing_function)(self.save_state.initial_learning_rate, self.save_state.iteration);
        self.save_state.state_space.entry(*compact_state)
            .and_modify(|prob|
                *prob += lrate * (max_probability - old_prob));
        // If there is only 1 best move, return that
        if best_moves.len() == 1 {
            best_moves[0usize]
        } else if best_moves.len() > 1 {
            // All the best moves are equal, just pick one at random
            *best_moves.choose(&mut self.generator).unwrap()
        } else {
            panic!("Couldn't select a move!")
        }
    }

    /// If exploring, choose a random (non-optimal) move
    fn make_random_move(&mut self, compact_state: &[Piece; 9]) -> [u8; 2] {
        let mut max_probability = 0f64;
        let potential_moves = self.get_potential_moves(compact_state);
        // Get the max value
        for idx in 0..potential_moves.probabilities.len() {
            if potential_moves.probabilities[idx] > max_probability {
                max_probability = potential_moves.probabilities[idx];
            }
        }
        //Get the moves that are less than max
        let mut exploration_moves: Vec<[u8; 2]> = Vec::with_capacity(9usize);
        for idx in 0..potential_moves.probabilities.len() {
            if potential_moves.probabilities[idx] < max_probability {
                exploration_moves.push(potential_moves.next_moves[idx]);
            }
        }
        // If all the moves have the same probability, choose randomly
        if exploration_moves.len() == 0 {
            *potential_moves.next_moves.choose(&mut self.generator).unwrap()
        } else {
            // Choose a random value from the exploration moves
            *exploration_moves.choose(&mut self.generator).unwrap()
        }
    }

    /// Get all possible potential moves
    fn get_potential_moves(&mut self, compact_state: &[Piece; 9]) -> PotentialMoves {
        let mut next_moves: Vec<[u8; 2]> = Vec::with_capacity(9);
        let mut probabilities: Vec<f64> = Vec::with_capacity(9);
        // Get a mutable clone of the board for looking up/generating probabilities
        let mut board = compact_state.clone();
        let mut counter: u8 = 0;
        for square in compact_state {
            if square.eq(&Piece::Empty) {
                next_moves.push([counter / 3, counter % 3]);
                probabilities.push(self.get_move_probability(&mut board,
                                                             [counter / 3, counter % 3],
                                                             self.save_state.piece))
            }
            counter += 1;
        }
        PotentialMoves {
            next_moves,
            probabilities,
        }
    }

    /// Get the win probability for a particular move on the given board
    fn get_move_probability(&mut self, compact_state: &mut [Piece; 9],
                            potential_move: [u8; 2], piece: Piece) -> f64 {
        if compact_state[(potential_move[0] * 3 + potential_move[1]) as usize] != Piece::Empty {
            panic!("Encountered impossible state in get move probability")
        }
        compact_state[(potential_move[0] * 3 + potential_move[1]) as usize] = piece;
        if !self.save_state.state_space.contains_key(compact_state) {
            self.save_state.state_space.insert(*compact_state, self.find_new_state_prob(compact_state));
        }
        let probability = self.save_state.state_space.get(compact_state).unwrap().clone();
        compact_state[(potential_move[0] * 3 + potential_move[1]) as usize] = Piece::Empty;
        probability
    }


    /// Calculates the winning probability for a previously unseen state
    fn find_new_state_prob(&self, compact_state: &[Piece; 9]) -> f64 {
        if let Some(p) = Self::check_winner(compact_state) {
            // If this player wins, it has a probability of 1
            return if self.save_state.piece.eq(&p) {
                1f64
            // If this player looses, it has a probability of 0
            } else {
                0f64
            };
        }
        // If there is no winner, and the board is full, the win probability is 0
        if Self::check_full(compact_state) {
            return 0f64;
        }
        // Otherwise we don't know, so this new state gets a probability of 0.5
        0.5f64
    }

    /// Check if the board is full
    fn check_full(compact_state: &[Piece; 9]) -> bool {
        for p in compact_state.iter() {
            if p.eq(&Piece::Empty) {
                return false;
            }
        }
        true
    }

    /// Check who has won the game, returns None if no winner, and Some(Piece) where
    /// Piece represents the winner
    fn check_winner(compact_state: &[Piece; 9]) -> Option<Piece> {
        match Self::check_winner_row(compact_state) {
            None => {
                match Self::check_winner_col(compact_state) {
                    None => {
                        Self::check_winner_diag(compact_state)
                    }
                    Some(piece) => { Some(piece) }
                }
            }
            Some(piece) => { Some(piece) }
        }
    }
    fn check_winner_col(compact_state: &[Piece; 9]) -> Option<Piece> {
        for col in 0..3 {
            if compact_state[col + 0] == compact_state[col + 3] &&
                compact_state[col + 0] == compact_state[col + 6] &&
                compact_state[col + 0] != Piece::Empty {
                return Some(compact_state[col + 0]);
            }
        }
        None
    }

    fn check_winner_row(compact_state: &[Piece; 9]) -> Option<Piece> {
        for row in 0..3 {
            if compact_state[3 * row + 0] == compact_state[3 * row + 1] &&
                compact_state[3 * row + 0] == compact_state[3 * row + 2] &&
                compact_state[3 * row + 0] != Piece::Empty {
                return Some(compact_state[3 * row + 0]);
            }
        }
        None
    }

    fn check_winner_diag(compact_state: &[Piece; 9]) -> Option<Piece> {
        if compact_state[3 * 0 + 0] == compact_state[3 * 1 + 1] &&
            compact_state[3 * 0 + 0] == compact_state[3 * 2 + 2] &&
            compact_state[3 * 0 + 0] != Piece::Empty {
            return Some(compact_state[3 * 0 + 0]);
        }

        if compact_state[3 * 2 + 0] == compact_state[3 * 1 + 1] &&
            compact_state[3 * 2 + 0] == compact_state[3 * 0 + 2] &&
            compact_state[3 * 2 + 0] != Piece::Empty {
            return Some(compact_state[3 * 0 + 0]);
        }
        None
    }
}

pub enum PlayerError {
    InvalidFile,
    UnableToSave,
    UnableToRead,
}


#[cfg(test)]
mod tests {
    use crate::agents::players::Player;
    use crate::game::board::Piece;

    #[test]
    fn test_check_winner_col() {
        let test_board: [Piece; 9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_col(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::O,
            Piece::O, Piece::X, Piece::X,
        ];
        assert_eq!(Player::check_winner_col(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_col(&test_board), Some(Piece::X));
    }

    #[test]
    fn test_check_winner_row() {
        let test_board: [Piece; 9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_row(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::O, Piece::O,
        ];
        assert_eq!(Player::check_winner_row(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::X, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::O, Piece::O,
        ];
        assert_eq!(Player::check_winner_row(&test_board), Some(Piece::X));
    }

    #[test]
    fn check_winner_diag() {
        let test_board: [Piece; 9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::X, Piece::X,
            Piece::O, Piece::O, Piece::O,
            Piece::X, Piece::X, Piece::X,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::O, Piece::O,
            Piece::O, Piece::X, Piece::O,
            Piece::O, Piece::O, Piece::X,
        ];
        assert_eq!(Player::check_winner_diag(&test_board), Some(Piece::X));
    }

    #[test]
    fn test_check_winner() {
        let test_board: [Piece; 9] = [
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::O, Piece::X,
            Piece::O, Piece::O, Piece::X,
            Piece::X, Piece::X, Piece::O,
        ];
        assert_eq!(Player::check_winner(&test_board), None);
        let test_board: [Piece; 9] = [
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
            Piece::X, Piece::Empty, Piece::Empty,
        ];
        assert_eq!(Player::check_winner(&test_board), Some(Piece::X));
    }
}