fn print_hidden_word(word: &str, known_letters: Vec<char>) -> bool {
    let mut final_string_vec: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in word.chars() {
        // set c to lower case .to_lowercase()
        if known_letters.contains(&c) {
            final_string_vec.push(c);
        } else {
            final_string_vec.push('_');
            missing_count = &missing_count + 1;
        }
    }
    let final_string = final_string_vec
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", final_string);
    return missing_count == 0;
}

fn main() {
    // let mut losses = 0;
    // while losses < 6 {
    //     println!("Welcome to the game of Hangman\n");
    // }
    let mut known_letters = Vec::new();
    known_letters.push('t');
    known_letters.push('e');
    let win = print_hidden_word("This is a test", known_letters);
    println!("{}", win);
}
