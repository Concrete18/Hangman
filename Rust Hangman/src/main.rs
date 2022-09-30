fn print_hidden_word(word: &str, known_letters: Vec<char>) {
    let mut final_string_list: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in word.chars() {
        // set c to lower case .to_lowercase()
        if known_letters.contains(&c) {
            final_string_list.push(c);
        } else {
            final_string_list.push('_');
            missing_count = &missing_count + 1;
        }
        let final_string: String = final_string_list.into_iter().collect();
        println!("{}", final_string);
    }

    // TODO invert value
    return missing_count;
}

fn main() {
    // let mut losses = 0;
    // while losses < 6 {
    //     println!("Welcome to the game of Hangman\n");
    // }
    let mut known_letters = Vec::new();
    known_letters.push('t');
    known_letters.push('e');
    print_hidden_word("This is a test", known_letters)
}
