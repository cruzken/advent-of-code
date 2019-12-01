pub fn star_one(input: &str) -> i32 {
    input.lines().map(|x| x.parse::<i32>().unwrap() / 3 - 2).fold(0, |sum, x| sum + x)
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(star_one("12"), 2);
        // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
        assert_eq!(star_one("14"), 2);
        // For a mass of 1969, the fuel required is 654.
        assert_eq!(star_one("1969"), 654);
        // For a mass of 100756, the fuel required is 33583.
        assert_eq!(star_one("100756"), 33583);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 0)
    }
}
