// def get_new_word(self):
//     """
//     Gets a new word for playing
//     """
//     if self.words_list:
//         new_word = random.choice(self.words_list)
//         self.words_list.pop(self.words_list.index(new_word))
//         self.current_word = new_word
//         return self.current_word
//     else:
//         return False

// use rand::seq::SliceRandom; // 0.7.2

// fn get_new_word(words_list: &[String]) {
//     let new_word: String = words_list.choose(&mut rand::thread_rng());
// }

fn join_vector(vec: Vec<char>) -> String {
    return vec
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");
}

fn print_hidden_word(word: &str, known_letters: Vec<char>) -> bool {
    let mut final_string_vec: Vec<char> = Vec::new();
    let mut missing_count: u8 = 0;
    for c in word.chars() {
        if known_letters.contains(&c.to_ascii_lowercase()) {
            final_string_vec.push(c);
        } else {
            final_string_vec.push('_');
            missing_count = missing_count + 1;
        }
    }
    let final_string: String = join_vector(final_string_vec);
    println!("{}", final_string);
    // returns true if no more underscores
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
