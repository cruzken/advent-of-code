use std::collections::HashMap;

trait PasswordCheck {
    fn not_decreasing(self) -> bool;
    fn double_adjacent(self) -> bool;
    fn one_adjacent_pair(self) -> bool;
}

impl PasswordCheck for u32 {
    fn not_decreasing(self) -> bool {
        let digits: String = self.to_string();
        let mut d = digits.chars();
        let mut last = d.next().unwrap();

        for i in d {
            if last.to_digit(10).unwrap() > i.to_digit(10).unwrap() {
                return false;
            }
            last = i;
        }
        true
    }

    fn double_adjacent(self) -> bool {
        let digits: String = self.to_string();
        let mut d = digits.chars();
        let mut last = d.next().unwrap();

        for i in d {
            if last == i {
                return true;
            }
            last = i;
        }
        false
    }

    fn one_adjacent_pair(self) -> bool {
        let mut adjacent: HashMap<char, u32> = HashMap::new();
        let digits: String = self.to_string();
        let mut d = digits.chars();
        let mut last = d.next().unwrap();

        for i in d {
            if last == i {
                let count = adjacent.entry(i).or_insert(0);
                *count += 1;
            }
            last = i;
        }

        adjacent.values().any(|x| *x == 1)
    }
}

pub fn star_one(input: &str) -> Option<usize> {
    let mut nums = input
        .split('-')
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap());

    let (min, max) = (nums.next()?, nums.next()?);

    Some(
        (min..=max)
            .filter(|x| x.not_decreasing() && x.double_adjacent())
            .count(),
    )
}

pub fn star_two(input: &str) -> Option<usize> {
    let mut nums = input
        .split('-')
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap());

    let (min, max) = (nums.next()?, nums.next()?);

    Some(
        (min..=max)
            .filter(|x| x.not_decreasing() && x.one_adjacent_pair())
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("156218-652527"), Some(1694));
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("156218-652527"), Some(1148))
    }
}
