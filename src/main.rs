fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
enum RoundResult {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

fn is_winner(p1: HandShape, p2: HandShape) -> RoundResult {
    match (p1, p2) {
        (x, y) if x == y => RoundResult::Draw,
        (HandShape::Rock, HandShape::Scissors) => RoundResult::Win,
        (HandShape::Scissors, HandShape::Paper) => RoundResult::Win,
        (HandShape::Paper, HandShape::Rock) => RoundResult::Win,
        _ => RoundResult::Loss,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_wins_over_scissors() {
        assert_eq!(
            RoundResult::Win,
            is_winner(HandShape::Rock, HandShape::Scissors)
        )
    }

    #[test]
    fn scissors_wins_over_paper() {
        assert_eq!(
            RoundResult::Win,
            is_winner(HandShape::Scissors, HandShape::Paper)
        )
    }

    #[test]
    fn paper_wins_over_rock() {
        assert_eq!(
            RoundResult::Win,
            is_winner(HandShape::Paper, HandShape::Rock)
        )
    }

    #[test]
    fn is_winner_handles_draws() {
        assert_eq!(
            RoundResult::Draw,
            is_winner(HandShape::Paper, HandShape::Paper),
            "Paper draw failed"
        );
        assert_eq!(
            RoundResult::Draw,
            is_winner(HandShape::Rock, HandShape::Rock),
            "Rock draw failed"
        );
        assert_eq!(
            RoundResult::Draw,
            is_winner(HandShape::Scissors, HandShape::Scissors),
            "Scissors draw failed"
        );
    }
}
