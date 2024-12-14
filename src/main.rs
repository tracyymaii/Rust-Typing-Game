mod game;
mod graphics;
mod input;

enum Song {
    Mago,
    Shhh,
    Default,
}

impl Song {
    fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => Song::Mago,
            "2" => Song::Shhh,
            _=> Self::Default
        }
    }

    fn file_name(&self) -> &'static str {
        match self {
            Song::Mago => "song_a.mp3",
            Song::Shhh => "song_b.mp3",
            Song::Default => "default_song.mp3",
        }
    }
}



fn main() {

    println!("Welcome to Rust Dance Dance Revolution");

    println!("Enter your name!");
 
    let mut name= String::new(); // string
    std::io::stdin().read_line(&mut name).unwrap(); // string
    name = name.trim().to_string();

    println!("Hello {}", name);
    println!("Choose a song:");

    println!("1. Mago - by GFRIEND");
    println!("2. Shhh! - by VIVIZ");

    println!("Choose 1 or 2");
    

    let mut song_choice= String::new(); // string
    std::io::stdin().read_line(&mut song_choice).unwrap(); // string

    while song_choice.trim() != "1" && song_choice.trim() != "2"  {

        song_choice.clear();

        println!("You did not input 1 or 2");
        println!("Please choose again");

        println!("Choose a song:");

        println!("1. Mago - by GFRIEND");
        println!("2. Shhh! - by VIVIZ");
    
        println!("Choose 1 or 2");
        
        std::io::stdin().read_line(&mut song_choice).unwrap(); // str
        println!("You put in {}", song_choice);

        if song_choice.trim() == "1" || song_choice.trim() == "2" {
            break;
        }
    }

    if song_choice.trim() ==  "1" {
        println!("Mago - by GFRIEND");
    } else {
        println!("Shhh! - by VIVIZ");
    }

    println!("Match the keys on your keyboard with the ones shown on the screen!");

    let selected_song = Song::from_choice(song_choice.trim()).file_name();

    game::start_game(name, selected_song);
    
}
