pub fn star_one(input: &str) -> i64 {
    let res = input.lines().fold(0, |acc, line| {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        acc + (digits.first().unwrap() * 10) + digits.last().unwrap()
    });

    res as i64
}

pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
