#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("10 players; last marble is worth 1618 points"), 8317);
        assert_eq!(star_one("13 players; last marble is worth 7999 points"), 146373);
        assert_eq!(star_one("17 players; last marble is worth 1104 points"), 2764);
        assert_eq!(star_one("21 players; last marble is worth 6111 points"), 54718);
        assert_eq!(star_one("30 players; last marble is worth 5807 points"), 37305);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
