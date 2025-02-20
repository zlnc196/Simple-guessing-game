use std::io;
use rand::{rng, Rng};


fn main() {
    
    let mut playAgain = true;
    let mut rng = rand::rng();
   


    while playAgain {

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

                let mut validInp = false;

                while !(validInp) {
                    println!("Would you like to play again (y/n)");
                    let mut response: String = String::new();
                    io::stdin().read_line(&mut response).expect("Error in entering message");
                    match response.trim() {
                        "n" => {playAgain = false; validInp=true; println!("Thanks for playing!")},
                        "y" => {validInp =true},
                        _ => {},

                    }

                }

            } else if userGuessInt > answer {
                println!("Answer is smaller");
                guessNumber += 1;
            } else if answer > userGuessInt {
                println!("Answer is larger");
                guessNumber += 1;
            }

        }

    }


}

