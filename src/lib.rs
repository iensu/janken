#[derive(PartialEq, Debug)]
pub enum RoundResult {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Debug)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

/// Executes a round of Rock, Paper, Scissors
///
/// The result is from the perspective of `p1`, so a result of
/// `RoundResult::Win` indicates that `p1` won.
pub fn execute_round(p1: HandShape, p2: HandShape) -> RoundResult {
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
            execute_round(HandShape::Rock, HandShape::Scissors)
        )
    }

    #[test]
    fn scissors_wins_over_paper() {
        assert_eq!(
            RoundResult::Win,
            execute_round(HandShape::Scissors, HandShape::Paper)
        )
    }

    #[test]
    fn paper_wins_over_rock() {
        assert_eq!(
            RoundResult::Win,
            execute_round(HandShape::Paper, HandShape::Rock)
        )
    }

    #[test]
    fn execute_round_handles_draws() {
        assert_eq!(
            RoundResult::Draw,
            execute_round(HandShape::Paper, HandShape::Paper),
            "Paper draw failed"
        );
        assert_eq!(
            RoundResult::Draw,
            execute_round(HandShape::Rock, HandShape::Rock),
            "Rock draw failed"
        );
        assert_eq!(
            RoundResult::Draw,
            execute_round(HandShape::Scissors, HandShape::Scissors),
            "Scissors draw failed"
        );
    }
}
