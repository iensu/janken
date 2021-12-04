use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    stdin.read_line(&mut buffer).unwrap();

    println!("You wrote: {}", buffer);
}
