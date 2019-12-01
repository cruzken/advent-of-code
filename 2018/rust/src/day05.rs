use std::mem;

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let v = input.chars().collect::<Vec<_>>();
    reduction_len(v)
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> usize {
    (b'a'..b'z' + 1)
        .map(|c| c as char)
        .filter(|c| input.contains(*c))
        .map(|x| {
            let v = input
                .chars()
                .filter(|c| *c != x && *c != flip_case(x))
                .collect::<Vec<_>>();
            reduction_len(v)
        })
        .min()
        .unwrap()
}

fn reduction_len(mut v: Vec<char>) -> usize {
    let first_count = v.len();
    let mut removed_count = 0;
    let mut reduced: Vec<char> = Vec::new();
    loop {
        let mut i = 0;
        let mut reacted = false;
        while i < v.len() - 1 {
            if v[i] == flip_case(v[i + 1]) {
                reacted = true;
                i += 2;
                removed_count += 2;
                continue;
            }
            reduced.push(v[i]);
            i += 1;
        }
        mem::swap(&mut v, &mut reduced);
        reduced.clear();
        if !reacted {
            break;
        }
    }
    first_count - removed_count
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
