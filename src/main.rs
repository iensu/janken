fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_meaning_of_life() {
        let meaning_of_life = 42;

        assert_eq!(42, meaning_of_life);
    }
}
