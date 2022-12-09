#![allow(warnings, unused)]
use clearscreen::ClearScreen;
use rand::Rng;
use std::{fs, io, path};

fn load_words() -> Vec<String> {
    const FILE_PATH: &str = "words_list.txt";
    let words_list: Vec<String>;
    if path::Path::new(FILE_PATH).exists() {
        let msg = "Should have been able to read this file";
        words_list = fs::read_to_string(FILE_PATH)
            .expect(msg)
            .lines()
            .map(|s: &str| s.to_string())
            .collect();
    } else {
        words_list = vec![
            "Array".to_string(),
            "Binary".to_string(),
            "Computer".to_string(),
            "Function".to_string(),
            "Linux".to_string(),
            "Programming".to_string(),
            "Rust".to_string(),
            "Variable".to_string(),
        ]
    }
    let words_list_len = words_list.len();
    println!("{words_list_len}");
    words_list
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
        println!("\nThanks for playing!");
    }
}

trait RandomChoice<T> {
    fn random_choice(self: &mut Self) -> T;
}

impl<T> RandomChoice<T> for Vec<T> {
    fn random_choice(self: &mut Self) -> T {
        if self.is_empty() {}
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.len());
        self.swap_remove(i)
    }
}

/// Joins a vector of characters with `sep`.
fn join_vector(vec: Vec<char>, sep: String) -> String {
    return vec
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(&sep);
}

/// Displays stick man with n parts shown
fn get_stickman(parts: u8) -> String {
    match parts {
        0 => return "\n    |-------|\n    |       |\n    |\n    |\n    |\n    |\n    |________________".to_string(),
        1 => return "\n    |-------|\n    |       |\n    |       O\n    |\n    |\n    |\n    |________________".to_string(),
        2 => return "\n    |-------|\n    |       |\n    |       O\n    |       |\n    |\n    |\n    |________________".to_string(),
        3 => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\n    |\n    |\n    |________________".to_string(),
        4 => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |\n    |\n    |________________".to_string(),
        5 => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      /\n    |\n    |________________".to_string(),
        _ => return "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      / \\\n    |\n    |________________".to_string(),
    }
}

/// Prints `hidden_word` with all letters not in `known_letters` replaced with _
fn censor_hidden_word(hidden_word: &String, known_letters: &Vec<char>) -> (String, bool) {
    let mut censored_string_vec: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in hidden_word.chars() {
        if known_letters.contains(&c.to_ascii_lowercase()) || c == ' ' {
            censored_string_vec.push(c);
        } else {
            censored_string_vec.push('_');
            missing_count += 1;
        }
    }
    let censored_word: String = join_vector(censored_string_vec, " ".to_string());
    (censored_word, missing_count == 0)
}

fn play(mut words_list: Vec<String>) {
    if words_list.len() == 0 {
        println!("\nYou win!\nThere are no more words left.");
        input();
        return;
    }
    let hidden_word = words_list.random_choice();
    let mut known_letters: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut losses: u8 = 0;
    let mut error = String::new();
    loop {
        // clears terminal before each rewrite
        ClearScreen::default().clear().expect("Clear Failed");
        // new game rewrite of for each loop
        println!("Welcome to the game of Hangman\n");
        // censores hidden word
        let (censored_word, win) = censor_hidden_word(&hidden_word, &known_letters);
        // TODO figure out how to center below
        println!("    {censored_word}");
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
        // checks for loss
        if losses >= 6 {
            println!("\nYou lose!\nIt was {hidden_word}");
            play_again(words_list);
            return;
        } else {
            // show errors if they exist
            if error.len() > 0 {
                println!("{error}");
            }
            // reset error
            error = "".to_string();
            // gets guess
            println!("\nType a letter or a full guess:");
            let guess = input().to_lowercase();
            // letter guess
            if guess.len() == 1 {
                let guess_char = guess.chars().next().expect("string is empty");
                // guessed letter was already chosen
                if known_letters.contains(&guess_char) {
                    error = "\nYou already guessed that correctly.".to_string();
                }
                // guessed letter or word was already used incorrectly
                else if incorrect_guesses.contains(&guess) {
                    error = format!("\nYou already guessed '{guess}' incorrectly.").to_string();
                    // guessed letter is in the current word
                } else if hidden_word.to_lowercase().contains(&guess) {
                    known_letters.push(guess_char);
                // blank response causes a new prompt for a guess again
                } else if guess == "" {
                    error = "\nPlease type in a valid guess.".to_string();
                // TODO write comment
                } else {
                    incorrect_guesses.push(guess.trim().to_string());
                    losses += 1;
                }
            // full guess
            } else if guess == hidden_word.to_lowercase() {
                println!("\nYou win!");
                play_again(words_list);
                return;
            // TODO fix duplicate
            } else if incorrect_guesses.contains(&guess) {
                error = format!("\nYou already guessed '{guess}' incorrectly.").to_string();
            } else {
                incorrect_guesses.push(guess.trim().to_string());
                losses += 1;
            }
        }
    }
}

fn main() {
    let words_list = load_words();
    play(words_list);
}

#[cfg(test)]
mod hangman_tests {
    use super::*;

    #[test]
    fn random_choice_works() {
        let perm_words_list: [String; 2] = ["Rust".to_string(), "Linux".to_string()];
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            let word = words_list.random_choice();
            let result = perm_words_list.contains(&word);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn random_choice_empties_vector() {
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            words_list.random_choice();
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
