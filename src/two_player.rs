use std::io;
use tictacrs::game;
use tictacrs::game::board::Piece;

/// Function to two_player Tic-Tac-Toe, returns true if another game is desired
pub fn two_player() ->bool{
    let mut game_board = game::board::Board::new();
    let mut current_player = Piece::X;

    loop {
        println!("Player {} Please Enter Your Move (q to quit)", current_player);
        println!("{}", game_board);
        // Get player input
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        let pmove = buffer.trim();
        match pmove {
            "Q"|"q"|"Quit"|"quit"=>{return false;}
            _=>{}
        }
        match game_board.player_move(pmove, &format!("{}",current_player)){
            Ok(_) => {}
            Err(game::board::BoardError::InvalidMove) => {
                println!("Sorry, invalid move");
                continue;
            }
            Err(game::board::BoardError::NotEmpty) => {
                println!("Sorry, that space is occupied");
                continue;
            }
            Err(_)=>{
                println!("Sorry, an unknown error occurred, please try again");
                continue;
            }
        }
        match game_board.check_winner() {
            None => {}
            Some(piece) => {
                println!("Congratulations Player {}, You Win!", piece);
                break;
            }
        }
        if game_board.is_full(){
            println!("No Winner!");
            break;
        }
        current_player = match current_player{
            Piece::X => {Piece::O}
            Piece::O => {Piece::X}
            Piece::Empty => {panic!("Current Player Error!")}
        }
    }
    println!("Would you like to two_player again? [y/n]");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    match buffer.trim() {
        "y"|"Y"|"yes"|"Yes" => {return true},
        "n"|"N"|"no"|"No" => {return false},
        _=>{
            println!("Sorry, couldn't understand your response, exiting...");
        }
    }
    false
}