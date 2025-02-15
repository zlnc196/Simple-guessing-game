use std::io;
use rand::{rng, Rng};


fn main() {
    
    let mut rng = rand::rng();
    let mut guessNumber: u32 = 0;

    let answer = rng.random_range(1..=100);

    let mut isCorrect = false;

    while !(isCorrect) {
        println!("Guess the number");
        let mut userGuessStr = String::new();
        io::stdin().read_line(&mut userGuessStr).expect("Error in entering answer");

        let mut userGuessInt;
        match userGuessStr.trim().parse() {
            Ok(num) => userGuessInt = num,
            Err(e) => continue,
        }

        if userGuessInt == answer {
            isCorrect = true;
            println!("{} is the correct answer", userGuessInt);
            println!("It took you {} guesses to reach the correct answer", guessNumber);
        } else if userGuessInt > answer {
            println!("Answer is smaller");
            guessNumber += 1;
        } else if answer > userGuessInt {
            println!("Answer is larger");
            guessNumber += 1;
        }

    }


}

