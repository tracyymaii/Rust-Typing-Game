/*
File : Leaderboard.rs
Purpose: Stores functions related core functions in the game.
*/ 
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

use crate::input::read_player_input;
use crate::GameState;
use crate::game::show_main_menu;

#[derive(Serialize, Deserialize, Debug, Clone)]

/*
Player Struct
Data structure to store player information
*/
pub struct Player {
    pub name: String,
    pub speed: f64,
    pub accuracy: f64,
}

/*
Save Player Score
Saves the player's score to the history and top 5 leaderboard.
Params: history_path: &str - The path to the history file, top5_path: &str - The path to the top 5 leaderboard file, name: &str - The player's name, wpm: f64 - The player's speed in words per minute, accuracy: f64 - The player's accuracy percentage.
Returns: None
*/
pub fn save_player_score(
    history_path: &str,
    top5_path: &str,
    name: &str,
    wpm: f64,
    accuracy: f64,
) {
    // Load existing history or initialize a new one
    let mut history = load_leaderboard(history_path).unwrap_or_else(Vec::new);

    let new_player = Player {
        name: name.to_string(),
        speed: wpm,
        accuracy,
    };

    history.push(new_player);

    match save_leaderboard(history_path, &history) {
        Ok(_) => println!("History updated successfully!"),
        Err(err) => eprintln!("Failed to save history: {:?}", err),
    }

    let mut top5 = history.clone();
    top5.sort_by(|a, b| b.accuracy.partial_cmp(&a.accuracy).unwrap_or(std::cmp::Ordering::Equal));
    top5.truncate(5);

    match save_leaderboard(top5_path, &top5) {
        Ok(_) => println!("Top 5 leaderboard updated successfully!"),
        Err(err) => eprintln!("Failed to save top 5 leaderboard: {:?}", err),
    }
}

/*
Load Leaderboard
Loads the leaderboard from a JSON file.
Params: file_path: &str - The path to the JSON file.
Returns: Option<Vec<Player>> - The leaderboard data or None if the file does not exist or is invalid.
*/
pub fn load_leaderboard(file_path: &str) -> Option<Vec<Player>> {
    if let Ok(data) = fs::read_to_string(file_path) {
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

/*
Save Leaderboard
Saves the leaderboard to a JSON file.
Params: file_path: &str - The path to the JSON file, leaderboard: &[Player] - The leaderboard data.
Returns: io::Result<()> - The result of the file write operation.
*/
pub fn save_leaderboard(file_path: &str, leaderboard: &[Player]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(leaderboard).expect("Failed to serialize leaderboard");
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())
}

/*
Show Leaderboard
Displays the leaderboard and provides options to view full history or top 5 leaderboard.
Params: None
Returns: GameState - The next game state based on player choice.
*/
pub fn show_leaderboard() -> GameState {
    println!("\n1. View Full History");
    println!("2. View Top 5 Leaderboard");
    println!("3. Quit");

    let choice = read_player_input();
    match choice.as_str() {
        "1" => {
            display_leaderboard("src/assets/history.json");
            println!("\n1. Play Again");
            println!("2. View Leaderboard");
            println!("3. Quit");
            
            show_main_menu()
        }
        "2" => {
            display_leaderboard("src/assets/top5.json");
            println!("\n1. Play Again");
            println!("2. View Leaderboard");
            println!("3. Quit");
            show_main_menu()
        }
        "3" => GameState::Quit,
        _ => {
            println!("Invalid choice.");
            show_main_menu()
        }
    }
}

/*
Display Leaderboard
Displays the leaderboard from a JSON file.
Params: path: &str - The path to the JSON file.
Returns: None
*/
pub fn display_leaderboard(path: &str) {
    let leaderboard = load_leaderboard(path).unwrap_or_default();

    println!("\nLeaderboard:");
    for (i, player) in leaderboard.iter().enumerate() {
        println!(
            "{}. {} - Speed: {:.2} WPM, Accuracy: {:.2}%",
            i + 1,
            player.name,
            player.speed,
            player.accuracy
        );
    }
}
