use std::io;

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    stdin.read_line(&mut buffer)?;

    println!("You wrote: {}", buffer);

    Ok(())
}
