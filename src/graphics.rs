use std::io::{stdout, Write};

pub fn render_car_position(player_progress: usize, total_length: usize) {
    let mut track = vec!['-'; total_length];
    if player_progress < total_length {
        track[player_progress] = '🚗';
    }
    print!("\r{}", track.iter().collect::<String>());
    stdout().flush().unwrap();
}
