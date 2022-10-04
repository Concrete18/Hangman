use rand::Rng;
use std::io;

/// asks for input after printing a msg
fn get_input(msg: &str) -> String {
    println!("{}", msg);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    return response;
}

fn play_again() {
    let mut response = get_input("\nDo you want to play again?\n");
    response = response.to_lowercase();
    if response == "yes" || response == "y" {
        main()
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
            "\n    |-------|\n    |       |\n    |        \n    |      \n    |      \n    |________________"
        ),
        1 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      \n    |      \n    |________________"
        ),
        2 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |       |  \n    |      \n    |________________"
        ),
        3 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /| \n    |      \n    |________________"
        ),
        4 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      \n    |________________"
        ),
        5 => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      /  \n    |________________"
        ),
        _ => println!(
            "\n    |-------|\n    |       |\n    |       O\n    |      /|\\\n    |      / \\\n    |________________"
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
    println!("{}", final_string);
    // returns true if no more underscores and the printed string
    return (missing_count == 0, final_string);
}

fn guess(hidden_word: String, known_letters: Vec<char>, incorrect_guesses: &Vec<String>) {
    // TODO finish function
    let mut guess = get_input("\nType a letter or a full guess:\n");
    guess = guess.to_lowercase();
    // full guess
    if guess == hidden_word.to_lowercase() {
        println!("\nYou win!");
        play_again()
    // letter guess
    } else if guess.len() == 1 {
        // guessed letter was already chosen
        let guess_char = guess.chars().next().expect("string is empty");
        if known_letters.contains(&guess_char) {}
        // guessed letter or word was already used incorrectly
        if incorrect_guesses.contains(&guess) {}
        // guessed letter is in the current word
        if hidden_word.contains(&guess) {}
    // blank response causes a new prompt for a guess again
    } else if guess == "" {
        // TODO update error to "\nPlease type in a valid guess."
    }
}

fn main() {
    let mut words_list = vec![
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

    let hidden_word = words_list.take_random_item();
    let mut known_letters: Vec<char> = vec![' '];
    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut losses: u8 = 0;
    while losses < 6 {
        // TODO clear terminal
        println!("Welcome to the game of Hangman\n");
        let win = print_hidden_word(&hidden_word, &known_letters).0;
        display_stick_man(losses);
        if win {
            println!("\nYou Win!");
            play_again()
        }
        if incorrect_guesses.len() > 0 {
            let wrong_guesses = incorrect_guesses.join(", ");
            println!("Wrong Guesses: {}", wrong_guesses);
        }
        // TODO show errors
        guess(hidden_word, known_letters, &incorrect_guesses)
    }
    // TODO display win text if no words are left
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
