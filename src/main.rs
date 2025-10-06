use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;
use std::fs;
use serde::{Serialize, Deserialize};

const SCORES_FILE: &str = "data/scores.json";

fn load_scores() -> GameScores {
    match fs::read_to_string(SCORES_FILE) {
        Ok(data) => {
            // Attempt to deserialize the JSON data into a GameScores struct
            match serde_json::from_str(&data) {
                Ok(scores) => scores,
                Err(e) => {
                    eprintln!("Error parsing scores file: {}. Starting new scores.", e);
                    GameScores::default()
                }
            }
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // File not found, which is fine for the first run
            println!("Scores file not found. Starting with 0 wins, 0 losses, 0 ties.");
            GameScores::default()
        },
        Err(e) => {
            // Other file reading errors
            eprintln!("Error reading scores file: {}. Starting new scores.", e);
            GameScores::default()
        }
    }
}

fn save_scores(scores: &GameScores) {
    match serde_json::to_string_pretty(scores) {
        Ok(json) => {
            match fs::write(SCORES_FILE, json) {
                Ok(_) => println!("Scores saved successfully!"),
                Err(e) => eprintln!("Error writing scores file: {}", e),
            }
        },
        Err(e) => eprintln!("Error serializing scores to JSON: {}", e),
    }
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct GameScores
{
    wins: i32,
    loses: i32,
    ties: i32,
}

fn main() 
{
    let mut scores = load_scores();

    loop 
    {
        let mut user_input = String::new();

        println!("Welcome to Rock paper scissors");
        println!("What do you want to do? \n\
        1.Play Game \n\
        2.Check Scores \n\
        3.Exit");

        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

        let user_input = user_input.trim();

        if user_input == "1"
        {
            println!();
            println!("Starting Game!");
            println!();

            'game_loop: loop
            {
                let mut game_input = String::new();

                let mut player_inputed: bool = false;

                println!("What will you play?\n\
                1.rock \n\
                2.paper \n\
                3.scissors");

                io::stdin()
                .read_line(&mut game_input)
                .expect("Failed to read line");

                let game_input = game_input.trim();

                let mut player_hand:&str  = "Invalid input";

                if game_input == "1"
                {
                    player_hand = "Rock";

                    player_inputed = true;
                }
                else if game_input == "2"
                {
                    player_hand = "Paper";

                    player_inputed = true;
                }
                else if  game_input == "3"
                {
                    player_hand = "Scissors";

                    player_inputed = true;
                }
                else 
                {
                    println!("Invalid input");
                }

                if player_inputed == true 
                {
                    let computer_hands = vec!["Rock", "Paper", "Scissors"];

                    let mut rng = rand::rng();

                    let index = rng.random_range(0..3);

                    let enemy_hand = computer_hands[index];

                    match player_hand 
                    {
                        "Rock" =>
                        {
                            match enemy_hand 
                            {
                                "Rock" => 
                                {
                                    println!();

                                    println!("You played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("It was a tie!");

                                    scores.ties += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Paper" =>
                                {
                                    println!();

                                    println!("You played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You lost!");

                                    scores.loses += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Scissors" =>
                                {
                                    println!();

                                    println!("You played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You win!");

                                    scores.wins += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                _ => 
                                {
                                    println!("Idk how but you got an error hooray this is not an achievement and you should be sad that you managed this");
                                }
                            }
                        },

                        "Paper" => 
                        {
                            match enemy_hand 
                            {
                                "Rock" => 
                                {
                                    println!();

                                    println!("You played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You win!");

                                    scores.wins += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Paper" =>
                                {
                                    println!();

                                    println!("You played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("It was a tie!");

                                    scores.ties += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Scissors" =>
                                {
                                    println!();

                                    println!("You played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You lose!");

                                    scores.loses += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                _ => 
                                {
                                    println!("How error, like genuenly how theres like a few checks to make sure this doesn't happen");
                                }
                            }
                        },

                        "Scissors" =>
                        {
                            match enemy_hand 
                            {
                                "Rock" => 
                                {
                                    println!();

                                    println!("You played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Rock");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You lose!");

                                    scores.loses += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Paper" =>
                                {
                                    println!();

                                    println!("You played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Paper");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You win!");

                                    scores.wins += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                "Scissors" =>
                                {
                                    println!();

                                    println!("You played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();

                                    println!("Enemy played Scissors");

                                    thread::sleep(Duration::from_secs(1));

                                    println!();
                                    println!("You tied!");

                                    scores.ties += 1;

                                    thread::sleep(Duration::from_secs(2));
                                },

                                _ => 
                                {
                                    println!("Guess who won the I break code because I am dumb award? you! this shouldn't be possible");
                                }
                            }
                        },

                        _ => 
                        {
                            println!("Code error this should not be happening since there has already been an invalid check");
                        }
                    }
                }
                else
                {
                    println!("unable to continue due to invalid input");

                    thread::sleep(Duration::from_secs(1));
                }

                break 'game_loop;
            }
        }
        else if user_input == "2"
        {
            println!();

            println!("Wins = {} \n\
            Loses = {} \n\
            Ties = {}", scores.wins, scores.loses, scores.ties);

            println!();

            thread::sleep(Duration::from_secs(10));
        }
        else if user_input == "3"
        {
            println!();

            println!("Thanks for playing!");
            
            thread::sleep(Duration::from_secs(1));

            println!("Saving Game");

            save_scores(&scores);

            break;
        }
        else 
        {
            println!();
            println!("Invalid input please try again");
            println!();

            thread::sleep(Duration::from_secs(1));
        }
    }
}
