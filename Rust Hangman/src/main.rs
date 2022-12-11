use clearscreen::ClearScreen;
use rand::Rng;
use std::{fs, io, path};

fn read_file(file_path: &path::Path) -> Vec<String> {
    let msg: &str = "Should have been able to read this file";
    fs::read_to_string(file_path)
        .expect(msg)
        .lines()
        .map(|s: &str| s.trim().to_string())
        .collect()
}

/// returns a word list from a file named words_list.txt in the same directory
/// or from a hardcoded backup list
fn load_words() -> Vec<String> {
    let shared_words_list = path::Path::new("../words_list.txt");
    let local_words_list = path::Path::new("words_list.txt");
    // TODO fill comment
    let words_list: Vec<String> = if shared_words_list.exists() {
        read_file(shared_words_list)
    // TODO fill comment
    } else if local_words_list.exists() {
        read_file(local_words_list)
    // backup vector in case text file is not found in local or shared paths
    } else {
        vec![
            "Array".to_string(),
            "Binary".to_string(),
            "Computer".to_string(),
            "Function".to_string(),
            "Linux".to_string(),
            "Programming".to_string(),
            "Rust".to_string(),
            "Variable".to_string(),
        ]
    };
    words_list
}

/// asks for input after printing a msg
fn input() -> String {
    let mut response: String = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    return response.trim().to_string();
}

/// Asks if you want to play again
fn play_again(words_list: Vec<String>) {
    println!("\nDo you want to play again?");
    let mut response: String = input();
    response = response.to_lowercase();
    if response == "yes" || response == "y" {
        play(words_list)
    } else {
        println!("\nThanks for playing!");
    }
}

trait RandomChoice<T> {
    fn random_choice(&mut self) -> T;
}

impl<T> RandomChoice<T> for Vec<T> {
    fn random_choice(&mut self) -> T {
        if self.is_empty() {}
        let mut rng = rand::thread_rng();
        let i: usize = rng.gen_range(0..self.len());
        self.swap_remove(i)
    }
}

/// Joins a vector of characters with `sep`.
fn join_vector(vec: Vec<char>, sep: String) -> String {
    return vec
        .iter()
        .map(|e: &char| e.to_string())
        .collect::<Vec<String>>()
        .join(&sep);
}

/// Displays stick man with n parts shown
fn get_stickman(parts: usize) -> &'static str {
    match parts {
        0 => "\n    |-------|\n    |       |\n    |\n    |\n    |\n    |\n    |________________",
        1 => "\n    |-------|\n    |       |\n    |       O\n    |\n    |\n    |\n    |________________",
        2 => "\n    |-------|\n    |       |\n    |       O\n    |       |\n    |\n    |\n    |________________",
        3 => "\n    |-------|\n    |       |\n    |       O\n    |      /|\n    |\n    |\n    |________________",
        4 => "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |\n    |\n    |________________",
        5 => "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      /\n    |\n    |________________",
        _ => "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      / \\\n    |\n    |________________",
    }
}

/// Returns `hidden_word` with all letters not in `known_letters` replaced with _
/// and a bool that is true if you won
fn censor_hidden_word(hidden_word: &str, known_letters: &[char]) -> (String, bool) {
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

/// Runs the entire game with `words_list`
fn play(mut words_list: Vec<String>) {
    // checks if any words are left
    if words_list.is_empty() {
        println!("\nYou win!\nThere are no more words left.");
        input();
        return;
    }
    // runs main play code
    let hidden_word: String = words_list.random_choice();
    let mut known_letters: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut error: String = String::new();
    loop {
        // clears terminal before each rewrite
        ClearScreen::default().clear().expect("Clear Failed");
        // new game rewrite of for each loop
        println!("Welcome to the game of Hangman\n");
        // censores hidden word
        let (censored_word, win) = censor_hidden_word(&hidden_word, &known_letters);
        println!("    {censored_word}");
        let incorrect_total = incorrect_guesses.len();
        // prints stickman
        let stickman = get_stickman(incorrect_total);
        println!("{stickman}");
        // checks if user has won
        if win {
            println!("\nYou Win!");
            play_again(words_list);
            return;
        }
        // shows all incorrect guesses if any exist
        if !incorrect_guesses.is_empty() {
            let wrong_guesses: String = incorrect_guesses.join(", ");
            println!("\nIncorrect Guesses: {wrong_guesses}");
        }
        // checks for loss
        if incorrect_total >= 6 {
            println!("\nYou lose!\nIt was {hidden_word}");
            play_again(words_list);
            return;
        } else {
            // show errors if they exist then resets error
            if !error.is_empty() {
                println!("{error}");
            }
            error = "".to_string();
            // gets guess
            println!("\nType a letter or a full guess:");
            let guess: String = input().to_lowercase();
            if incorrect_guesses.contains(&guess) {
                error = format!("\nYou already guessed '{guess}' incorrectly.").to_string();
            // letter guess
            } else if guess.len() == 1 {
                let guess_char: char = guess.chars().next().expect("string is empty");
                // guessed letter was already chosen
                if known_letters.contains(&guess_char) {
                    error = "\nYou already guessed that correctly.".to_string();
                } else if hidden_word.to_lowercase().contains(&guess) {
                    known_letters.push(guess_char);
                // blank response causes a new prompt for a guess again
                } else if guess.is_empty() {
                    error = "\nPlease type in a valid guess.".to_string();
                // adds incorrect guess to incorrect_guesses and increments total wrong answers
                } else {
                    incorrect_guesses.push(guess.trim().to_string());
                }
            // full guess
            } else if guess == hidden_word.to_lowercase() {
                println!("\nYou win!");
                play_again(words_list);
                return;
            } else {
                incorrect_guesses.push(guess.trim().to_string());
            }
        }
    }
}

fn main() {
    let words_list: Vec<String> = load_words();
    play(words_list);
}

#[cfg(test)]
mod hangman_tests {
    use super::*;

    #[test]
    fn read_file_works() {
        let words_list_path = path::Path::new("../words_list.txt");
        let words_list: Vec<String> = read_file(words_list_path);
        assert!(words_list.contains(&"Rust".to_string()));
        assert!(!words_list.is_empty());
    }

    #[test]
    fn load_words_works() {
        let words_list = load_words();
        assert!(words_list.len() > 10);
    }

    #[test]
    fn random_choice_works() {
        let perm_words_list: [String; 2] = ["Rust".to_string(), "Linux".to_string()];
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            let word = words_list.random_choice();
            // confirms word is no longer in words_list
            assert!(!words_list.contains(&word));
            // confirms
            let result = perm_words_list.contains(&word);
            assert!(result);
        }
    }

    #[test]
    fn random_choice_empties_vector() {
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..2 {
            let word = words_list.random_choice();
            // confirms word is no longer in words_list
            assert!(!words_list.contains(&word));
        }
        assert!(words_list.is_empty());
    }

    #[test]
    fn censor_hidden_word_no_win_yet() {
        let known_letters: Vec<char> = vec!['t', 'e'];
        let (string, win) = censor_hidden_word("Test this", &known_letters);
        // tests string
        assert_eq!(string, "T e _ t   t _ _ _".to_string());
        // tests win
        assert!(!win);
    }

    #[test]
    fn censor_hidden_word_win() {
        let known_letters: Vec<char> = vec!['t', 'e', 's'];
        let (string, win) = censor_hidden_word("Test", &known_letters);
        // tests string
        assert_eq!(string, "T e s t".to_string());
        // tests win
        assert!(win);
    }
}
