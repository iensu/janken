use rand::prelude::*;

#[derive(PartialEq, Debug)]
pub enum RoundResult {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Debug, Clone)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for HandShape {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_lowercase().as_str() {
            "rock" => Ok(HandShape::Rock),
            "paper" => Ok(HandShape::Paper),
            "scissors" => Ok(HandShape::Scissors),
            x => Err(format!("Can't turn '{}' into a hand shape", x)),
        }
    }
}

impl HandShape {
    /// Return a random `HandShape`
    pub fn random() -> HandShape {
        let mut rng = thread_rng();
        let selection = vec![HandShape::Rock, HandShape::Paper, HandShape::Scissors];

        selection.choose(&mut rng).unwrap().to_owned()
    }
}

/// Executes a round of Rock, Paper, Scissors
///
/// The result is from the perspective of `p1`, so a result of
/// `RoundResult::Win` indicates that `p1` won.
///
/// ## Examples
///
/// ```
/// use janken::*;
///
/// let result = execute_round(HandShape::Paper, HandShape::Rock);
///
/// assert_eq!(RoundResult::Win, result);
/// ```
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

    #[test]
    fn handshapes_can_be_constructed_from_strings() {
        let tests = vec![
            ("rock", HandShape::Rock),
            ("paper", HandShape::Paper),
            ("scissors", HandShape::Scissors),
            ("ROCK", HandShape::Rock),
            ("PAPER", HandShape::Paper),
            ("SCISSORS", HandShape::Scissors),
            ("  rock", HandShape::Rock),
            ("paper  ", HandShape::Paper),
            ("  scissors  ", HandShape::Scissors),
        ];

        for (input, expected) in tests {
            let result = HandShape::try_from(input);
            assert_eq!(Ok(expected), result);
        }
    }
}
