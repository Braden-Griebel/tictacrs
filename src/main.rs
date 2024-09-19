use tictacrs::game;
use std::io;
use clap::{Parser, Subcommand};
use tictacrs::game::board::Piece;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Play) => {
            println!("Welcome to TicTacRs!");
            game();
            println!("Thank you for playing!");
        },
        Some(Commands::Train {iterations})=>{
            println!("Training iterations: {}", iterations);
        }
        None => {}
    }
}

/// Wrapper function to loop play until player quits
fn game(){
    let mut new_game:bool = true;
    // Game Loop
    loop {
        if new_game {
            new_game = play();
        } else {
            break;
        }
    }

}

/// Function to play Tic-Tac-Toe, returns true if another game is desired
fn play()->bool{
    let mut game_board = game::board::Board::new();
    let mut current_player = Piece::X;
    let mut buffer = String::new();
    loop {
        println!("Player {} Please Enter Your Move (q to quit)", current_player);
        println!("{}", game_board);
        // Get player input
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        match buffer.trim() {
            "Q"|"q"|"Quit"|"quit"=>{break;}
            _=>{}
        }
        match game_board.player_move(buffer.trim(), &format!("{}",current_player)){
            Ok(_) => {}
            Err(game::board::BoardError::InvalidMove) => {
                println!("Sorry, invalid move");
            }
            Err(game::board::BoardError::NotEmpty) => {
                println!("Sorry, that space is occupied")
            }
            Err(_)=>{
                println!("Sorry, an unknown error occurred, please try again")
            }
        }
        match game_board.check_winner() {
            None => {}
            Some(piece) => {
                println!("Congratulations Player {}, You Win!", piece);
                break;
            }
        }
        current_player = match current_player{
            Piece::X => {Piece::O}
            Piece::O => {Piece::X}
            Piece::Empty => {panic!("Current Player Error!")}
        }
    }
    println!("Would you like to play again? [y/n]");
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to Run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Play Game
    Play,
    /// Train the players
    Train {
        #[arg(short, long, value_name = "iterations")]
        iterations: u32,
    }
}
