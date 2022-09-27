use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello Worlds!");
    let time = Duration::from_secs(1);

    // sleep
    sleep(time);
}
