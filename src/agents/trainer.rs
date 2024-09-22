use std::path::{Path, PathBuf};
use indicatif::ProgressBar;
use crate::agents::players::Player;
use crate::game::board::{Board, Piece};

pub struct Trainer {
    iteration: u32,
}

impl Trainer {
    /// Given two players, train them and save the results into the out_directory,
    /// returns a tuple of the player_x save data path, and the player_o save data path
    pub fn train(player1: &mut Player,
                 player2: &mut Player,
                 iterations: u32,
                 out_directory: &Path,
                 progress_bar: bool,
    ) -> Result<(PathBuf, PathBuf), TrainerError> {
        let mut pbar: Option<ProgressBar> = None;
        if progress_bar {
            pbar = Some(ProgressBar::new(iterations as u64));
        }
        if player1.get_player_piece() == player2.get_player_piece() {
            return Err(TrainerError::InvalidPlayers);
        }
        let mut training_board: Board = Board::new();
        for it in 0..iterations {
            if let Some(ref bar) = pbar {
                bar.inc(1);
            }
            training_board.clear_board();
            // Update the players for the current iteration
            player1.update_iteration(it);
            player2.update_iteration(it);
            // Variable to hold the previous board state, to show to loosing player
            // in order to update their value function
            let mut prev_board1: [Piece; 9] =
                [
                    Piece::Empty, Piece::Empty, Piece::Empty,
                    Piece::Empty, Piece::Empty, Piece::Empty,
                    Piece::Empty, Piece::Empty, Piece::Empty,
                ];
            let mut prev_board2: [Piece; 9] =
                [
                    Piece::Empty, Piece::Empty, Piece::Empty,
                    Piece::Empty, Piece::Empty, Piece::Empty,
                    Piece::Empty, Piece::Empty, Piece::Empty,
                ];
            loop {
                // Get the first players move
                let p1_move = player1.make_move(&training_board.get_compact_state());
                training_board.make_auto_player_move(p1_move[0], p1_move[1], player1.get_player_piece());
                // If there is some winner, end the iteration
                if let Some(_) = training_board.check_winner() {
                    // Since player1 must have won, show the previous board as a losing position
                    // to player2
                    player2.show_loosing_state(&prev_board2);
                    break;
                }
                if training_board.is_full() {
                    break;
                }
                prev_board1 = training_board.get_compact_state();
                // If the first player didn't win, get the second players move
                let p2_move = player2.make_move(&training_board.get_compact_state());
                training_board.make_auto_player_move(p2_move[0], p2_move[1], player2.get_player_piece());
                if let Some(_) = training_board.check_winner() {
                    // Since player2 must have won, show the previous board as a losing position
                    // to player1
                    player1.show_loosing_state(&prev_board1);
                    break;
                }
                if training_board.is_full() {
                    break;
                }
                prev_board2 = training_board.get_compact_state();
            }
        }

        // Save the players data to desired files
        let player_x_file_path = out_directory.join("player_x_save.ttr");
        let player_o_file_path = out_directory.join("player_o_save.ttr");
        if player1.get_player_piece() == Piece::X {
            match player1.save_player_state(&player_x_file_path) {
                Ok(_) => {}
                Err(_) => { return Err(TrainerError::FailedToSave) }
            };
            match player2.save_player_state(&player_o_file_path) {
                Ok(_) => {}
                Err(_) => { return Err(TrainerError::FailedToSave) }
            }
        } else {
            match player2.save_player_state(&player_x_file_path) {
                Ok(_) => {}
                Err(_) => { return Err(TrainerError::FailedToSave) }
            };
            match player1.save_player_state(&player_o_file_path) {
                Ok(_) => {}
                Err(_) => { return Err(TrainerError::FailedToSave) }
            }
        }
        Ok((player_x_file_path, player_o_file_path))
    }
}

pub enum TrainerError {
    FailedToSave,
    InvalidPlayers,
}