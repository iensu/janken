use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    stdin
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    println!("You wrote: {}", buffer);
}
