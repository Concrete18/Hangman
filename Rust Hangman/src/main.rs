use rand::Rng;
use std::{fs, io};

/// loads words list from text file
fn load_words_list() -> Vec<String> {
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

    // let file_path = "words_list.txt";
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let words_list = contents.trim().split('\n').collect::<Vec<String>>();

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
fn display_stick_man(parts: u8) {
    match parts {
        0 => println!(
            "\n    |-------|\n    |       |\n    |        \n    |      \n    |      \n    |        \n    |________________"
        ),
        1 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      \n    |      \n    |        \n    |________________"
        ),
        2 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |       |  \n    |      \n    |        \n    |________________"
        ),
        3 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /| \n    |      \n    |        \n    |________________"
        ),
        4 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      \n    |        \n    |________________"
        ),
        5 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      /  \n    |        \n    |________________"
        ),
        _ => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      / \\\n    |        \n    |________________"
        ),
    }
}

/// Prints `hidden_word` with all letters not in `known_letters` replaced with _
fn print_hidden_word(hidden_word: &String, known_letters: &Vec<char>) -> (bool, String) {
    let mut final_string_vec: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in hidden_word.chars() {
        if known_letters.contains(&c.to_ascii_lowercase()) {
            final_string_vec.push(c);
        } else {
            final_string_vec.push('_');
            missing_count = missing_count + 1;
        }
    }
    let final_string: String = join_vector(final_string_vec, " ".to_string());
    println!("{final_string}");
    // returns true if no more underscores and the printed string
    (missing_count == 0, final_string)
}

fn play(mut words_list: Vec<String>) {
    if words_list.len() < 1 {
        println!("\nYou win!\nThere are no more words left.");
        input();
        return;
    }
    let hidden_word = words_list.take_random_item();
    let mut known_letters: Vec<char> = vec![' '];
    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut losses: u8 = 0;
    let mut error = String::new();
    while losses < 6 {
        // TODO add proper terminal clear
        print!("{esc}c", esc = 27 as char);
        // new game rewrite of for each loop
        println!("Welcome to the game of Hangman\n");
        println!("\n{hidden_word}"); // TODO remove when done testing
        let win = print_hidden_word(&hidden_word, &known_letters).0;
        display_stick_man(losses);
        if win {
            println!("\nYou Win!");
            play_again(words_list);
            return;
        }
        if incorrect_guesses.len() > 0 {
            let wrong_guesses = incorrect_guesses.join(", ");
            println!("\nWrong Guesses: {wrong_guesses}");
        }
        // show errors if they exist
        println!("{error}");
        // gets guess5
        println!("Type a letter or a full guess:");
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
            }
        }
    }
    println!("\nYou lose!");
    play_again(words_list);
}

fn main() {
    let words_list = load_words_list();

    play(words_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_random_item_works() {
        let perm_words_list: [String; 2] = ["Rust".to_string(), "Linux".to_string()];
        let mut words_list = vec!["Rust".to_string(), "Linux".to_string()];
        for _ in 0..1 {
            let word = words_list.take_random_item();
            let result = perm_words_list.contains(&word);
            assert_eq!(result, true);
        }
    }

    // TODO test empty words_list

    #[test]
    fn hidden_word_printing_no_win() {
        let known_letters: Vec<char> = vec!['t', 'e'];
        let result = print_hidden_word(&"This is a test".to_string(), &known_letters);
        let answer = false;
        assert_eq!(result.0, answer);
    }

    #[test]
    fn hidden_word_printing_win() {
        let known_letters: Vec<char> = vec!['t', 'e', 's'];
        let result = print_hidden_word(&"test".to_string(), &known_letters);
        let answer = true;
        assert_eq!(result.0, answer);
    }

    #[test]
    fn hidden_word_printing_value() {
        let known_letters: Vec<char> = vec!['t', 'e'];
        let result = print_hidden_word(&"This is a test".to_string(), &known_letters);
        let answer = "T _ _ _ _ _ _ _ _ _ t e _ t".to_string();
        assert_eq!(result.1, answer);
    }
}
