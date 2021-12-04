use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("Write something:");

    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            println!("You wrote: {}", buffer);
        }
        Err(err) => {
            println!("Failed to read user input! {:?}", err);
        }
    }
}
