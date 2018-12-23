use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let v = input.chars().collect::<Vec<_>>();
    reduction_len(v)
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> usize {
    let alphabet = (b'a'..=b'z' + 1)
        .map(|c| c as char)
        .filter(|c| input.contains(*c));

    let mut reduced: HashSet<usize> = HashSet::new();
    for letter in alphabet {
        let v = input
            .chars()
            .filter(|c| *c != letter && *c != flip_case(letter))
            .collect::<Vec<_>>();
        reduced.insert(reduction_len(v));
    }
    *reduced.iter().min().unwrap()
}

fn reduction_len(mut v: Vec<char>) -> usize {
    let mut i = 0;
    while i < v.len() - 1 {
        if v[i] == flip_case(v[i + 1]) {
            v.remove(i + 1);
            v.remove(i);
            i = 0;
        } else {
            i += 1;
        }
    }
    v.len()
}

fn flip_case(letter: char) -> char {
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
        assert_eq!(star_two("dabAcCaCBAcCcaDA"), 4)
    }
}
