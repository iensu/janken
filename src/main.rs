use std::io;

use janken::HandShape;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    stdin.read_line(&mut buffer)?;

    match HandShape::try_from(buffer) {
        Ok(shape) => println!("Got {:?}", shape),
        Err(err) => println!("Failed to get hand shape: {}", err),
    }

    Ok(())
}
