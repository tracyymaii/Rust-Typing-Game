
# Type Game in Rust

## Description
Typing Speed Racer is an interactive console-based game designed to test and improve your typing speed and accuracy. Players types while their progress is visualized as a car moving along a track. The game offers a variety of features such as sentence generation, live progress visualization, and a leaderboard to track top scores.

## How to Run
1. Install Rust
2. Clone the Repository
3. Run the game with "cargo run"

## How to Play
**Main Menu:**
Select an option by typing the corresponding number:
- 1 to start the race.
- 2 to view the leaderboard.
- 3 to quit the game.
**Race Mode:**
- Enter your name to begin.
- A random sentence will appear on the screen. Type it as quickly and accurately as possible.
- Use backspace to correct mistakes. Press Esc to cancel the race.
**Leaderboard:**
-View your past scores or the top 5 players.
**Screenshots to Show How to Play the Game**
![Picture1](image.png)
![Picture2](image-1.png)
![Picture3](image-2.png)
![Picture4](image-3.png)
![Picture5](image-4.png)

## Link to YouTube Video
https://youtu.be/p8x412F171A

## Data Types
Enums:
- GameState: Represents the current state of the game (e.g., MainMenu, Race, Leaderboard, Quit).

Structs:
- Player: Stores player details such as name, speed, and accuracy.

File Handling:
- JSON files (history.json and top5.json) store game data for persistence.

Collections:
- Used vectors for track visualization, sentence generation, and leaderboard storage.

## Difficulties and Solutions
**Challenges**

Creating Live Car Movement:
- Issue: In the beginnning the car would stay static. Then other issues that I faced was that it would never stop.

- Solution: To have the car move as the user typed, I added "termion" into the dependencies, because it allows live tracking. To create a set track for the car to move on, I made it based on the length of the sentence.

Leaderboard Persistence:
- Issue: It was hard to keep both the history and leaderboard updated. 

- Solution: I just worked more carefully to fix it.

Rust:
- Issue: Honestly, learning Rust, the functions, the states, the precise data types; all of it was very hard. I felt like I needed to pay much much MUCH more attention to detail and often felt like I was running in circles.

- Solution: Perseverance :D. I had no choice but to get through it. It was hard, and I changed my gamemany times to make it simpler. However, I perservered since I knew that every struggle that I went to would help me and my learning in the future.


## The Good, the Bad, and the Ugly
### The Good

Learning Rust's enums, structs, and external crates was a rewarding experience. It felt really good to be able to create an entire interactive game from scratch, complete with moving visuals and multiple features, entirely by myself. This project pushed me to really engage with Rust's capabilities and helped solidify my understanding of the language. 
### The Bad

Although not inherently bad, this was my first time using crates, and I realized that anytime I wanted to go beyond Rust's built-in capabilities, I had to incorporate external crates. While crates are powerful and essential, figuring out how to integrate them properly added complexity. Additionally, debugging the program, especially with features like raw mode and input handling, was a tedious and time-consuming process.
### The Ugly

Rust's data type precision, while a strength of the language, was a bit of a double-edged sword for me. It was frustrating to constantly ensure that every data type matched exactly, as even a minor mistake would throw errors. I often got lost in figuring out which data types were compatible with others, forcing me to be extra careful and precise throughout the process.

## Learning Experience
From this project, I learned a lot about file management in Rust, such as how to read, write, and manage JSON files using external crates like serde. I also gained practical experience in using external crates to extend Rust’s functionality and learned how to structure and manage multiple files in a Rust project effectively. This assignment was a great introduction to building something more complex and hands-on with Rust!
Understand Rust’s ownership model and its implications in file handling and input management.

