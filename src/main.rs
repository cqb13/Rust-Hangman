use std::fs;
use rand::Rng;
const FILE_PATH : &str = "src/words.txt";

fn main() {
    const MAX_GUESSES : u8 = 10;

    let contents = match fs::read_to_string(FILE_PATH) {
        Ok(contents) => contents,
        Err(error) => panic!("Failed to read file {}: {}", FILE_PATH, error),
    };

    let num = rand::thread_rng().gen_range(0..contents.lines().count());

    let mut words : Vec<&str> = Vec::new();
    for line in contents.lines() {
        words.push(line);
    }

    let word = words[num];
    let length = word.len();
    
    println!("Welcome to Hangman!");
    println!("Made by: cqb13");

    let mut control = String::new();
    println!("Press enter to start");
    std::io::stdin().read_line(&mut control).expect("Failed to read line");
    
    println!("The word is {} letters long", length);
    println!("The word is: {}", hide_word(word));

    game_loop(word, MAX_GUESSES);
}

fn game_loop(word : &str, max_guesses : u8) {
    let mut guesses : u8 = 0;
    let mut guessed_letters : Vec<char> = Vec::new();
    let mut hidden_word = hide_word(word);
    let mut correct_guesses : u8 = 0;
    let mut incorrect_guesses : u8 = 0;
    let mut game_over : bool = false;

    while !game_over && guesses < max_guesses {
        let mut guess = String::new();
        println!("Guess a letter");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().to_lowercase();
        let guess = guess.chars().next().unwrap();
        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter");
            println!("You have also guessed: {:?}", guessed_letters);
        } else if hidden_word != word {
            guessed_letters.push(guess);
            if word.contains(guess) {
                for (i, c) in word.chars().enumerate() {
                    if c == guess {
                        hidden_word.remove(i);
                        hidden_word.insert(i, guess);
                        correct_guesses += 1;
                    }
                }
                println!("The word is: {}", hidden_word);
            } else {
                println!("Incorrect guess");
                println!("The word is: {}", hidden_word);
                guesses += 1;
                incorrect_guesses += 1;
                println!("You have {} guesses left", max_guesses - guesses);
            }
        }

        if hidden_word == word || guesses >= max_guesses {
            game_over = true;
            println!("");
            if guesses >= max_guesses {
                println!("You lose!");
            } else {
                println!("You win!");
            }
            println!("The word was {}", word);
            println!("You guessed the word in {} guesses", max_guesses - guesses);
            println!("You had {} guesses left", guesses);
            println!("You had {} correct guesses and {} incorrect guesses", correct_guesses, incorrect_guesses);
            println!("You guessed the following letters: {:?}", guessed_letters);
            let mut control = String::new();
            println!("Press enter to exit");
            std::io::stdin().read_line(&mut control).expect("Failed to read line");
        }
    }
}

fn hide_word(word : &str) -> String {
    let mut hidden_word = String::new();
    for _ in 0..word.len() {
        hidden_word.push('_');
    }
    hidden_word
}