
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
**The Good**

Engaging Gameplay: Visualizing progress with a moving car made the game interactive and fun.

Learning Rust: Gained hands-on experience with Rust’s enums, structs, and external crates.

**The Bad**

Raw Mode Issues: Debugging raw mode in the terminal was tedious.

**The Ugly**

Error Handling: Initial implementation lacked proper error handling, causing crashes during file I/O.


## Learning Experience
This assignment helped me:

Understand Rust’s ownership model and its implications in file handling and input management.

Learn how to use external crates like crossterm, serde, and rand effectively.

Build a modular and extensible codebase by separating concerns into different modules (e.g., game, graphics, input).

