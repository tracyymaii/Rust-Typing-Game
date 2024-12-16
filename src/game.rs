use crate::graphics::render_car_position;
use crate::input::read_player_input;
use crate::leaderboard::{save_player_score, show_leaderboard};
use rand::prelude::SliceRandom;
use std::fs;
use std::time::Instant;

pub enum GameState {
    MainMenu,
    Race,
    Leaderboard,
    Quit,
}

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self},
};

pub fn show_main_menu() -> GameState {
    let choice = read_player_input();
    match choice.as_str() {
        "1" => start_race(),
        "2" => show_leaderboard(),
        "3" => GameState::Quit,
         &_ => todo!()
    }
}
pub fn start_race() -> GameState {
    let dictionary_path = "src/assets/dictionary.txt";
    let history_path = "src/assets/history.json";
    let top5_path = "src/assets/top5.json";

    // Player input for name
    println!("\nEnter your name:");
    let player_name = read_player_input();

    // Generate the sentence
    let sentence = generate_sentence(dictionary_path, 10);
    println!("\nType the following sentence:");
    println!("{}", sentence);

    // Initialize race variables
    let total_chars = sentence.len();
    let mut player_progress = 0;
    let mut typed_chars = String::new();

    let start_time = Instant::now();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    // Main game loop
    loop {
        render_car_position(player_progress, total_chars);

        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        typed_chars.push(c);
                        player_progress += 1;
                    }
                    KeyCode::Backspace => {
                        typed_chars.pop();
                        if player_progress > 0 {
                            player_progress -= 1;
                        }
                    }
                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        println!("\nRace cancelled.");
                        return GameState::MainMenu;
                    }
                    _ => {}
                }
            }
        }

        if player_progress >= total_chars {
            break;
        }
    }

    // End of race
    terminal::disable_raw_mode().unwrap();
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let wpm = (player_progress as f64 / 5.0) / (elapsed_time / 60.0);
    let accuracy = calculate_accuracy(&typed_chars, &sentence);

    println!("\nRace complete!");
    println!("Your speed: {:.2} WPM", wpm);
    println!("Your accuracy: {:.2}%", accuracy);

    // Save scores
    save_player_score(history_path, top5_path, &player_name, wpm, accuracy);

    // Menu after race

    println!("\n1. Play Again");
    println!("2. View Leaderboard");
    println!("3. Quit");
    show_main_menu()
}



pub fn calculate_accuracy(typed: &str, target: &str) -> f64 {
    let typed_words: Vec<&str> = typed.split_whitespace().collect();
    let target_words: Vec<&str> = target.split_whitespace().collect();

    let mut correct_words = 0;
    for (typed_word, target_word) in typed_words.iter().zip(target_words.iter()) {
        if typed_word == target_word {
            correct_words += 1;
        }
    }

    (correct_words as f64 / target_words.len() as f64) * 100.0
}

pub fn generate_sentence(dictionary_path: &str, word_limit: usize) -> String {
    let words: Vec<String> = fs::read_to_string(dictionary_path)
        .expect("Failed to load dictionary")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut rng = rand::thread_rng();
    let selected: Vec<_> = words.choose_multiple(&mut rng, word_limit).cloned().collect();
    selected.join(" ")
}


