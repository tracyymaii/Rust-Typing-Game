/*
File : Graphics.rs
Purpose: Stores functions related to rendering graphics in the game.
*/ 
use std::io::{stdout, Write};

/*
Render Car Position
Renders the player's car position and moves on the track based on the player's progress.
Params: player_progress: usize - The player, total_length: usize - The total length of the track.
Returns: None
*/
pub fn render_car_position(player_progress: usize, total_length: usize) {
    let mut track = vec!['-'; total_length];
    if player_progress < total_length {
        track[player_progress] = '🚗';
    }
    print!("\r{}", track.iter().collect::<String>());
    stdout().flush().unwrap();
}
