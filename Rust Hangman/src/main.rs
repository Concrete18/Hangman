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
    return vec
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(&sep);
}

/// Prints `hidden_word` with all letters not in `known_letters`
/// replaced with _
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

fn guess() {
    let guess = get_input("\nType a letter or a full guess:\n");
    println!("{}", guess);
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
    let mut known_letters: Vec<char> = Vec::new();
    let mut losses: u8 = 0;
    let mut incorrect_guesses: u8 = 0;
    while losses < 6 {
        println!("Welcome to the game of Hangman\n");
        let win = print_hidden_word(&hidden_word, &known_letters).0;
        // TODO display stick man
        if win {
            println!("\nYou Win!");
            // TODO set up play again func
        }
        if incorrect_guesses > 0 {
            // TODO show incorrect guesses
            println!("Wrong Guesses: {}", incorrect_guesses);
        }
        // TODO show errors
        // TODO create guess func
        guess()
    }
    // TODO display win text if no words are left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_random_item_works() {
        let perm_words_list: [String; 3] = [
            "Rust".to_string(),
            "Linux".to_string(),
            "Programming".to_string(),
        ];
        let mut words_list = vec![
            "Rust".to_string(),
            "Linux".to_string(),
            "Programming".to_string(),
        ];
        for _ in 0..2 {
            let word = words_list.take_random_item();
            let result = perm_words_list.contains(&word);
            assert_eq!(result, true);
        }
    }

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
