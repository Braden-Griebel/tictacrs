use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use annealing::{INITIAL_EXPLORATION_RATE, INITIAL_LEARNING_RATE};
use tictacrs::agents::players::Player;
use tictacrs::agents::trainer::Trainer;
use tictacrs::game::board::Piece;

mod two_player;
mod single_player;
mod annealing;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Play{trained_directory}) => {
            println!("Welcome to TicTacRs!");
            game(trained_directory.clone());
            println!("Thank you for playing!");
        }
        Some(Commands::Train {
                 iterations,
                 output_directory,
                 progress_bar,
             }
        ) => {
            let iterations: u32 = match iterations {
                None => {1000}
                Some(i) => {*i}
            };
            let output_directory: PathBuf = match output_directory {
                None => {
                    std::env::current_dir().unwrap()
                }
                Some(out) => {out.clone()}
            };
            println!("Training iterations: {}", iterations);
            let mut player1 = Player::new(Piece::X,
                                          INITIAL_LEARNING_RATE,
                                          INITIAL_EXPLORATION_RATE,
                                          annealing::learning_rate_function,
                                          annealing::exploration_rate_function);
            let mut player2 = Player::new(Piece::O,
                                          INITIAL_LEARNING_RATE,
                                          INITIAL_EXPLORATION_RATE,
                                          annealing::learning_rate_function,
                                          annealing::exploration_rate_function);
            _ = Trainer::train(&mut player1, &mut player2, iterations,
                           &output_directory, *progress_bar)
        }
        None => {}
    }
}


/// Wrapper function to determine if two-player, or one-player mode is desired
fn game(trained_player_dir: Option<PathBuf>) {
    let mut new_game: bool = true;
    // Game Loop
    loop {
        if new_game {
            println!("One or two players? (1/2)");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            let choice = buffer.trim();
            match choice {
                "1" => {

                    new_game = single_player::single_player(trained_player_dir.clone());
                }
                "2" => {
                    new_game = two_player::two_player();
                }
                _ => {
                    println!("Sorry, couldn't understand, please try again");
                    continue;
                }
            }
        } else {
            break;
        }
    }
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
    Play {
        /// Directory containing the trained players
        #[arg(short,long)]
        trained_directory: Option<PathBuf>,
    },
    /// Train the players
    Train {
        /// Number of training iterations to run
        #[arg(short, long, value_name = "iterations")]
        iterations: Option<u32>,
        /// Where the trained player data will be saved to
        #[arg(short, long)]
        output_directory: Option<PathBuf>,
        /// Whether a progress bar should be shown
        #[arg(short, long)]
        progress_bar: bool,
    },
}
