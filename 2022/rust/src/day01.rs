fn sum_calories_per_elf(input: &str) -> Vec<i64> {
    input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|c| match c.parse::<i64>() {
                    Ok(n) => n,
                    Err(e) => {
                        panic!("error while parsing {}: {}", c, e);
                    }
                })
                .sum()
        })
        .collect()
}

pub fn star_one(input: &str) -> Option<i64> {
    sum_calories_per_elf(input).into_iter().max()
}

pub fn star_two(input: &str) -> i64 {
    let mut calories_per_elf = sum_calories_per_elf(input);
    calories_per_elf.sort();

    return calories_per_elf.into_iter().rev().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), Some(24000))
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 45000)
    }
}
