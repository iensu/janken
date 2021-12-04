fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

fn is_winner(p1: HandShape, p2: HandShape) -> bool {
    match (p1, p2) {
        (HandShape::Rock, HandShape::Scissors) => true,
        (HandShape::Scissors, HandShape::Paper) => true,
        (HandShape::Paper, HandShape::Rock) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_wins_over_scissors() {
        assert_eq!(true, is_winner(HandShape::Rock, HandShape::Scissors))
    }

    #[test]
    fn scissors_wins_over_paper() {
        assert_eq!(true, is_winner(HandShape::Scissors, HandShape::Paper))
    }

    #[test]
    fn paper_wins_over_rock() {
        assert_eq!(true, is_winner(HandShape::Paper, HandShape::Rock))
    }
}
