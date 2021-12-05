use std::io;

use janken::HandShape;

fn to_handshape(value: String) -> Result<HandShape, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "rock" => Ok(HandShape::Rock),
        "paper" => Ok(HandShape::Paper),
        "scissors" => Ok(HandShape::Scissors),
        x => Err(format!("Can't turn '{}' into a hand shape", x)),
    }
}

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    stdin.read_line(&mut buffer)?;

    match to_handshape(buffer) {
        Ok(shape) => println!("Got {:?}", shape),
        Err(err) => println!("Failed to get hand shape: {}", err),
    }

    Ok(())
}
