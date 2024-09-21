use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tictacrs::agents::players::Player;
use tictacrs::agents::trainer::Trainer;
use tictacrs::game::board::Piece;

mod two_player;
mod single_player;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Play) => {
            println!("Welcome to TicTacRs!");
            game();
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
                                          learning_rate_function,
                                          exploration_rate_function);
            let mut player2 = Player::new(Piece::O,
                                          INITIAL_LEARNING_RATE,
                                          INITIAL_EXPLORATION_RATE,
                                          learning_rate_function,
                                          exploration_rate_function);
            _ = Trainer::train(&mut player1, &mut player2, iterations,
                           &output_directory, *progress_bar)
        }
        None => {}
    }
}

const INITIAL_LEARNING_RATE: f64 = 0.75;
const INITIAL_EXPLORATION_RATE: f64 = 0.2;


/// Function used for calculating the learning rate
fn learning_rate_function(initial_rate: f64, iteration: u32) -> f64 {
    // Currently uses a step decay
    let drop_rate:f64 = 0.9;
    let step_size: u32 = 20;
    initial_rate * drop_rate.powi((iteration/step_size) as i32)
}

/// Function used for calculating the exploration rate
fn exploration_rate_function(initial_rate: f64, iteration: u32) -> f64 {
    // Currently uses a step decay
    let drop_rate: f64 = 0.9;
    let step_size: u32 = 10;
    initial_rate * drop_rate.powi((iteration/step_size) as i32)
}

/// Wrapper function to determine if two-player, or one-player mode is desired
fn game() {
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
                    // Not implemented yet
                    new_game = single_player::single_player();
                }
                "2" => {
                    new_game = two_player::two_player();
                }
                _ => {
                    println!("Sorry, couldn't understand, please try again");
                    continue;
                }
            }
            new_game = two_player::two_player();
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
    Play,
    /// Train the players
    Train {
        #[arg(short, long, value_name = "iterations")]
        iterations: Option<u32>,
        #[arg(short, long)]
        output_directory: Option<PathBuf>,
        #[arg(short, long)]
        progress_bar: bool,
    },
}
