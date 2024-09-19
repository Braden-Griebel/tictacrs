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

/// Wrapper function to determine if two-player, or one-player mode is desired
fn game(){
    let mut new_game:bool = true;
    // Game Loop
    loop {
        if new_game {
            println!("One or two players? (1/2)");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            let choice = buffer.trim();
            match choice {
                "1"=>{
                    // Not implemented yet
                    continue;
                },
                "2"=>{
                    new_game = two_player();
                }
                _=>{
                    println!("Sorry, couldn't understand, please try again")
                    continue;
                }
            }
            new_game = two_player();
        } else {
            break;
        }
    }

}

/// Function to two_player Tic-Tac-Toe, returns true if another game is desired
fn two_player() ->bool{
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
