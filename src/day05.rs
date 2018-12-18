use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let v = input.chars().collect::<Vec<_>>();
    reduction_len(v)
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut reduced: HashSet<usize> = HashSet::new();
    for letter in alphabet.chars() {
        if input.contains(letter) {
            let mut v = input.chars().collect::<Vec<_>>();

            let mut i = 0;
            while i <= v.len() - 1 {
                if v[i] == letter || v[i] == flip_case(&letter) {
                    v.remove(i);
                    if i == 0 {
                        continue;
                    }
                    i -= 1;
                }
                i += 1;
            }
            reduced.insert(reduction_len(v));
        }
    }
    *reduced.iter().min().unwrap()
}

fn reduction_len(mut v: Vec<char>) -> usize {

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
        assert_eq!(star_two("dabAcCaCBAcCcaDA"), 4)
    }
}
