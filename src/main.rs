fn main() {
    println!("Hello, world!");
}

fn is_winner(p1: &str, p2: &str) -> bool {
    if p1 == "rock" && p2 == "scissors" {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_wins_over_scissors() {
        assert_eq!(true, is_winner("rock", "scissors"))
    }
}
