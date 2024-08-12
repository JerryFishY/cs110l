// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let n: usize = secret_word.len();
    println!("Welcome to CS110L Hangman! ");
    let mut guessed_answer: String = String::from("-".repeat(n));
    let mut guessed_letters: String = String::from("");
    let mut cnt: i32 = 5;
    let mut stovec: Vec<Vec<i32>> =  vec![Vec::new(); 26];
    for i in 0..(n-1){
        let position = (secret_word_chars[i] as u8) - ('a' as u8)
        stovec[position].insert(i);
    }
    loop{

    println!("The word so far is {}",guessed_answer);
    println!("You have guessed the following letters: {}",guessed_letters);
    println!("You have {} guesses left", cnt);
    print!("Please guess a letter: ");
    // Make sure the prompt from the previous line gets displayed:
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
    guess = String::from(guess.trim());
    if(guessed_letters.contains(&guess)){
        println!("This letter has been guessed!");
        continue;
    }
    guessed_letters=guessed_letters+guess.as_str();
    
    }
}
