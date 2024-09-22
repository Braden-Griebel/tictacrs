use std::path::PathBuf;
use std::io;
use tictacrs::agents::players::Player;
use tictacrs::game::board::{Board, Piece};
use crate::annealing;

pub(crate) fn single_player(trained_player_dir: Option<PathBuf>) -> bool {
    let trained_player_dir = trained_player_dir.unwrap_or_else(|| { std::env::current_dir().unwrap() });
    let mut play_board = Board::new();
    // Start the game loop
    loop {
        play_board.clear_board();
        println!("Would you like to play as X or O? (X/O)");
        // Piece selection loop
        let computer_piece: Piece;
        let human_piece: Piece;
        let mut computer_piece_str: String = String::new();
        let mut human_piece_str: String = String::new();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            let choice = buffer.trim();
             human_piece = match choice {
                "X" | "x" => {
                    human_piece_str.push_str("X");
                    computer_piece_str.push_str("O");
                    computer_piece = Piece::O;
                    Piece::X
                },
                "O" | "o" => {
                    human_piece_str.push_str("O");
                    computer_piece_str.push_str("X");
                    computer_piece=Piece::X;
                    Piece::O
                },
                "Q" | "q" => {
                    return false;
                }
                _ => {
                    println!("Sorry, couldn't understand choice, try again");
                    continue;
                }
            };
            break;
        };
        // Now try to read in a trained opponent, if not possible create a new opponent
        let trained_player_file = match computer_piece {
            Piece::X => trained_player_dir.join(PathBuf::from("player_x_save.ttr")),
            Piece::O => trained_player_dir.join(PathBuf::from("player_o_save.ttr")),
            _=>{panic!("Impossible Automated Player Piece")}
        };
        let mut computer_player:Player = match Player::new_from_file(
            trained_player_file,
            annealing::learning_rate_function,
            annealing::exploration_rate_function,
        ){
          Ok(p)=>p,
            Err(_)=>{
                println!("Couldn't find trained automatic player, creating a new one");
                Player::new(
                    computer_piece,
                    annealing::INITIAL_LEARNING_RATE,
                    annealing::INITIAL_EXPLORATION_RATE,
                    annealing::learning_rate_function,
                    annealing::exploration_rate_function,
                )
            }
        };
        let mut computer_move:String;
        let mut human_move:String;
        // If the computer goes first, get its move
        if computer_piece == Piece::X {
            println!("{}", play_board);
            computer_move = Player::to_human_move(&computer_player.make_move(
                &play_board.get_compact_state())
            );
            // This can't fail, since the board must be empty
            // Also the computer player should never make an invalid move
            _=play_board.player_move(&computer_move, &computer_piece_str).expect("Computer failed to make possible move");
        }
        // Store a copy of the board state right after the computer plays
        // in order to show it that as a losing position
        let mut prev_board: [Piece; 9] =
            [
                Piece::Empty, Piece::Empty, Piece::Empty,
                Piece::Empty, Piece::Empty, Piece::Empty,
                Piece::Empty, Piece::Empty, Piece::Empty,
            ];
        // Start the game itself
        loop {
            println!("{}", play_board);
            // Start with the human player
            human_move = get_move_selection();
            if human_move=="q" || human_move=="Q"{
                return false;
            }
            match play_board.player_move(&human_move, &human_piece_str) {
                Ok(_)=>{
                    println!("{}", play_board);
                },
                Err(_)=>{
                    println!("Sorry, invalid move, try again");
                    continue;
                }
            }
            // Check if the player won
            if let Some(_) = play_board.check_winner() {
                // If there is a winner, it has to be due to the most recent move
                // in this case the players
                println!("{}", play_board);
                println!("Congratulations Player! You Win!");
                // Show the computer the losing state so it can update
                computer_player.show_loosing_state(&prev_board);
                break;
            }
            // Check if the board is full
            if play_board.is_full(){
                println!("{}", play_board);
                println!("Sorry, it's a tie.");
                break;
            }
            // Now allow the computer to move
            computer_move = Player::to_human_move(&computer_player.make_move(&play_board.get_compact_state()));
            _=play_board.player_move(&computer_move, &computer_piece_str).expect("Computer failed to make possible move");
            if let Some(_) = play_board.check_winner(){
                println!("{}", play_board);
                println!("Oh No! You have been defeated by a computer! :-(");
                break;
            }
            if play_board.is_full(){
                println!("{}", play_board);
                println!("Sorry, it's a tie.");
                break;
            }
            prev_board = play_board.get_compact_state();
        }
        computer_player.update_iteration(computer_player.get_iteration());
        // Now that the game has been played, save the automated player
        let trained_player_file = match computer_piece {
            Piece::X => trained_player_dir.join(PathBuf::from("player_x_save.ttr")),
            Piece::O => trained_player_dir.join(PathBuf::from("player_o_save.ttr")),
            _=>{panic!("Impossible Automated Player Piece")}
        };
        match computer_player.save_player_state(trained_player_file){
            Ok(_)=>{},
            Err(_)=>{
                println!("Couldn't save automated player state.");
            }
        };
    }
}

fn get_move_selection()->String{
    println!("Please select your move (q to quit):");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}