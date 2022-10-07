#![allow(warnings, unused)]
use clearscreen::ClearScreen;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fs, io};

/// loads words list from text file
fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

/// asks for input after printing a msg
fn input() -> String {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    return response.trim().to_string();
}

/// Asks if you want to play again
fn play_again(words_list: Vec<String>) {
    println!("\nDo you want to play again?");
    let mut response = input();
    response = response.to_lowercase();
    if response == "yes" || response == "y" {
        play(words_list)
    } else {
        println!("Thanks for playing!");
    }
}

trait TakeRandom<T> {
    fn take_random_item(self: &mut Self) -> T;
}

impl<T> TakeRandom<T> for Vec<T> {
    fn take_random_item(self: &mut Self) -> T {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.len());
        self.swap_remove(i)
    }
}

/// Joins a vector of characters with `sep`.
fn join_vector(vec: Vec<char>, sep: String) -> String {
    // TODO make this a trait like TakeRandom
    return vec
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(&sep);
}

/// Displays stick man with n parts shown
fn get_stickman(parts: u8) -> String {
    match parts {
        0 => return "\n    |-------|\n    |       |\n    |        \n    |      \n    |      \n    |        \n    |________________".to_string(),
        1 => return "\n    |-------|\n    |       |\n    |       O\n    |      \n    |      \n    |        \n    |________________".to_string(),
        2 => return "\n    |-------|\n    |       |\n    |       O\n    |       |  \n    |      \n    |        \n    |________________".to_string(),
        3 => return "\n    |-------|\n    |       |\n    |       O\n    |      /| \n    |      \n    |        \n    |________________".to_string(),
        4 => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      \n    |        \n    |________________".to_string(),
        5 => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      /  \n    |        \n    |________________".to_string(),
        _ => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      / \\\n    |        \n    |________________".to_string(),
    }
}

/// Prints `hidden_word` with all letters not in `known_letters` replaced with _
fn censor_hidden_word(hidden_word: &String, known_letters: &Vec<char>) -> (String, bool) {
    let mut censored_string_vec: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in hidden_word.chars() {
        if known_letters.contains(&c.to_ascii_lowercase()) || &c == &' ' {
            censored_string_vec.push(c);
        } else {
            censored_string_vec.push('_');
            missing_count = missing_count + 1;
        }
    }
    let censored_word: String = join_vector(censored_string_vec, " ".to_string());
    (censored_word, missing_count == 0)
}

fn play(mut words_list: Vec<String>) {
    if words_list.len() < 1 {
        println!("\nYou win!\nThere are no more words left.");
        input();
        return;
    }
    let hidden_word = words_list.take_random_item();
    let mut known_letters: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut losses: u8 = 0;
    let mut error = String::new();
    while losses < 6 {
        // clears terminal before each rewrite
        ClearScreen::default().clear().expect("Clear Failed");
        // new game rewrite of for each loop
        println!("Welcome to the game of Hangman\n");
        // censores hidden word
        let (censored_word, win) = censor_hidden_word(&hidden_word, &known_letters);
        println!("{censored_word}");
        // prints stickman
        let stickman = get_stickman(losses);
        println!("{stickman}");
        // checks if user has won
        if win {
            println!("\nYou Win!");
            play_again(words_list);
            return;
        }
        // shows all incorrect guesses if any exist
        if incorrect_guesses.len() > 0 {
            let wrong_guesses = incorrect_guesses.join(", ");
            println!("\nIncorrect Guesses: {wrong_guesses}");
        }
        // show errors if they exist
        if error.len() > 0 {
            println!("{error}");
        }
        // gets guess5
        println!("\nType a letter or a full guess:");
        let mut guess = input();
        guess = guess.to_lowercase();
        // full guess
        if guess == hidden_word.to_lowercase() {
            println!("\nYou win!");
            play_again(words_list);
            return;
            // letter guess
        } else if guess.len() == 1 {
            let guess_char = guess.chars().next().expect("string is empty");
            // guessed letter was already chosen
            if known_letters.contains(&guess_char) {
                error = "\nYou already guessed that correctly.".to_string();
            }
            // guessed letter or word was already used incorrectly
            else if incorrect_guesses.contains(&guess) {
                error = "\nYou already guessed that incorrectly.".to_string();
            // guessed letter is in the current word
            } else if hidden_word.to_lowercase().contains(&guess) {
                known_letters.push(guess_char);
            // blank response causes a new prompt for a guess again
            } else if guess == "" {
                error = "\nPlease type in a valid guess.".to_string();
            } else {
                incorrect_guesses.push(guess.trim().to_string());
                losses = losses + 1;
                // TODO fix incorrect reset after loss
                let stickman = get_stickman(losses);
                println!("{stickman}");
            }
        }
    }
    println!("\nYou lose!");
    play_again(words_list);
}

fn main() {
    // let file_path = "words_list.txt";
    // let words_list = load_words_list(file_path)?;

    let words_list = vec![
        "Rust".to_string(),
        "Linux".to_string(),
        "Programming".to_string(),
        "Virtual Reality".to_string(),
        "Algorithm".to_string(),
        "Arrays".to_string(),
        "Binary".to_string(),
        "Computer".to_string(),
        "Terminal".to_string(),
    ];

    play(words_list)
}

#[cfg(test)]
mod hangman_tests {
    use super::*;

    #[test]
    fn take_random_item_works() {
        let perm_words_list: [String; 2] = ["Rust".to_string(), "Linux".to_string()];
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            let word = words_list.take_random_item();
            let result = perm_words_list.contains(&word);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn take_random_item_empties_vector() {
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            words_list.take_random_item();
        }
        assert_eq!(words_list.len(), 0);
    }

    #[test]
    fn hidden_word_printing_no_win() {
        let known_letters: Vec<char> = vec!['t', 'e'];
        let (string, win) = censor_hidden_word(&"Test this".to_string(), &known_letters);
        // tests string
        assert_eq!(string, "T e _ t   t _ _ _".to_string());
        // tests win
        assert_eq!(win, false);
    }

    #[test]
    fn hidden_word_printing_win() {
        let known_letters: Vec<char> = vec!['t', 'e', 's'];
        let (string, win) = censor_hidden_word(&"Test".to_string(), &known_letters);
        // tests string
        assert_eq!(string, "T e s t".to_string());
        // tests win
        assert_eq!(win, true);
    }
}
