mod exercises;

use std::io;
const DIFFICULTY: usize = 4;

/// BITCOIN MINER
/// This is a simple program that reads a string from the terminal, converts it into byte array, and attemps to find a nonce such that sha256(arr, nonce) has at least DIFFICULTY zeros
/// You may find the guessing game example from the rust book helpful when it comes to reading input: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
/// You'll need to use the `sha2` crate for sha256. Its documentation can be found at https://docs.rs/sha2/0.9.8/sha2/. An example of how to import a crate can be found in the guessing game tutorial.
fn main() {
    println!("Please input a string to mine.");
    let mut string_to_mine = String::new();
    io::stdin()
        .read_line(&mut string_to_mine)
        .expect("Failed to read line");

    let buf = string_to_mine.as_bytes();
    loop {
        // TODO: mine buf with sha256!
        unimplemented!()
    }
}
