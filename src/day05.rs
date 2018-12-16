#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> usize {
    let mut v = input.chars().collect::<Vec<_>>();

    let mut i = 0;
    while i < v.len() - 1 {
        if v[i] == flip_case(&v[i + 1]) {
            v.remove(i + 1);
            v.remove(i);
            i = 0;
        } else {
            i += 1;
        }
    }
    v.len()
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn flip_case(letter: &char) -> char {
    if letter.is_ascii_lowercase() {
        return letter.to_ascii_uppercase();
    }
    letter.to_ascii_lowercase()
}
#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("dabAcCaCBAcCcaDA"), 10)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
