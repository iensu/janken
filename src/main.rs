fn main() {
    println!("Hello, world!");
}

fn is_winner(p1: &str, p2: &str) -> bool {
    if p1 == "rock" && p2 == "scissors" {
        return true;
    } else if p1 == "scissors" && p2 == "paper" {
        return true;
    } else if p1 == "paper" && p2 == "rock" {
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

    #[test]
    fn scissors_wins_over_paper() {
        assert_eq!(true, is_winner("scissors", "paper"))
    }

    #[test]
    fn paper_wins_over_rock() {
        assert_eq!(true, is_winner("paper", "rock"))
    }
}
