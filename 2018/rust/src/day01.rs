use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> i32 {
    let mut sum = 0;

    for el in input.lines() {
        sum += el.parse::<i32>().unwrap();
    }
    sum
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> i32 {
    let mut sum = 0;
    let mut history = HashSet::new();
    history.insert(sum);
    for el in input.lines().cycle() {
        sum += el.parse::<i32>().unwrap();
        if !history.insert(sum) {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("+1\n-2\n+3\n+1"), 3);
        assert_eq!(star_one("+1\n+1\n+1"), 3);
        assert_eq!(star_one("+1\n+1\n-2"), 0);
        assert_eq!(star_one("-1\n-2\n-3"), -6);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("+1\n-1"), 0);
        assert_eq!(star_two("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(star_two("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(star_two("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
