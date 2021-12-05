use std::io;

use janken::{execute_round, HandShape, RoundResult};

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();

    loop {
        println!("Rock, paper or scissors?");
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        if let Ok(shape) = HandShape::try_from(buffer) {
            match execute_round(shape, HandShape::Paper) {
                RoundResult::Win => {
                    println!("Hooray! You won! \\(^O^)/");
                    break;
                }
                RoundResult::Loss => {
                    println!("Boohoo, you lost! (T T)");
                    break;
                }
                RoundResult::Draw => {
                    println!("It was a draw!");
                }
            }
        } else {
            println!("Sorry, I didn't quite get that...");
        }
    }

    Ok(())
}
