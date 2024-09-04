// you'll need these two imports to read input form the user
use std::io;
use std::io::Write;

fn check_letter_in_word(word: &str, guess: &char) -> bool {
    for c in word.chars() {
        if c == *guess {
            return true;
        }
    }

    return false;
}

fn edit_guess(answer: &str, word: &mut String, guess: &char) {
    let mut index = 0;
    let mut characters: Vec<char> = word.chars().collect();

    for c in answer.chars() {
        if c == *guess {
            characters[index] = *guess;
        }
        index += 1;
    }

    *word = characters.into_iter().collect();
}

fn main() {
    let s = "austin";
    let mut guess: String = String::from("------");
    let num_letters = s.chars().count();
    let mut matched: usize = 0;
    let mut num_guesses = 5;

    println!("Welcome to cis1905 Hangman!");
    println!("The word so far is {guess}");
    println!("You have {} guesses left", num_guesses);
    println!("Please guess a letter");

    while num_guesses > 0 && matched < num_letters {
        io::stdout().flush().expect("Error flushing stdout.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line.");

        match input.chars().nth(0) {
            Some(c) => {
                if !check_letter_in_word(&guess, &c) {
                    // check if letter should be in the word
                    if check_letter_in_word(&s, &c) {
                        edit_guess(&s, &mut guess, &c);
                        matched += 1;
                        num_guesses += 1;
                    }
                }
            }
            None => {
                println!("The string is empty");
            }
        }
        num_guesses -= 1;
        if matched == num_letters || num_guesses == 0 {
            break;
        }
        println!("The word so far is {guess}");
        println!("You have {} guesses left", num_guesses);
        println!("Please guess a letter");
    }

    if matched == num_letters {
        println!("Congratulations, you guessed the secret word {}!", s);
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
