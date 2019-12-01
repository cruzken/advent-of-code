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
        assert_eq!(star_one("5"), 0124515891);
        assert_eq!(star_one("18"), 9251071085);
        assert_eq!(star_one("2018"), 5941429882);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
