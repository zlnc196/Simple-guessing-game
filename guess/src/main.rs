use std::io;
use rand::{rng, Rng};


fn main() {
    
    let mut rng = rand::rng();

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
        } else if userGuessInt > answer {
            println!("Answer is smaller");
        } else if answer > userGuessInt {
            println!("Answer is larger");
        }

    }
}

