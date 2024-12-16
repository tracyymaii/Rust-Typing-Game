/*
Title : Type Rust Game
Purpose: Create a typing game that measures the player's typing speed and accuracy in Rust.
Author: Tracy Mai
Date: December 15, 2024
*/ 

mod game;
mod graphics;
mod input;
mod leaderboard;

use game::GameState;

fn main() {
    let mut state = GameState::MainMenu;

    println!("Welcome to Typing Speed Racer!");
    println!("1. Start Race");
    println!("2. View Leaderboard");
    println!("3. Quit");
    println!("\nIf you do not choose the correct options, the game will quit automatically.");


    while let GameState::Quit = state {
        match state {
            GameState::MainMenu => state = game::show_main_menu(),
            GameState::Race => state = game::start_race(),
            GameState::Leaderboard => state = leaderboard::show_leaderboard(),
            GameState::Quit => break,
        }
    }

    game::show_main_menu();

    println!("");
    println!("Goodbye! Thanks for playing!");
}
