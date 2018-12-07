use std::collections::HashMap;

#[allow(dead_code)]
pub fn star_one(input: &str) -> u32 {
    let mut twice = 0;
    let mut thrice = 0;
    for line in input.lines() {
        let mut uniques: HashMap<char, u32> = HashMap::new();
        for el in line.chars() {
            if uniques.contains_key(&el) {
                let increment: u32 = uniques.get(&el).unwrap() + 1;
                uniques.insert(el, increment);
            } else {
                uniques.insert(el, 1);
            }
        }
        let mut twice_found = false;
        let mut thrice_found = false;
        for (.., value) in uniques.iter() {
            if *value == 2 && !twice_found {
                twice = twice + 1;
                twice_found = true;
            }
            if *value == 3 && !thrice_found {
                thrice = thrice + 1;
                thrice_found = true;
            }
            if twice_found && thrice_found {
                break;
            }
        }
    }
    twice * thrice
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> String {
    let mut box_id_list: Vec<&str> = Vec::new();
    for line in input.lines() {
        box_id_list.push(line);
    }
    for box1 in 0..box_id_list.len() {
        for box2 in (box1 + 1)..box_id_list.len() {
            let mut mismatch = 0;
            for (id1, id2) in box_id_list[box1].chars().zip(box_id_list[box2].chars()) {
                if id1 != id2 {
                    mismatch = mismatch + 1;
                    if mismatch > 1 {
                        break;
                    }
                }
            }
            if mismatch == 1 {
                let mut found: String = String::new();
                for (id1, id2) in box_id_list[box1].chars().zip(box_id_list[box2].chars()) {
                    if id1 == id2 {
                        found.push(id1);
                    }
                }
                return found;
            }
        }
    }
    return String::new();
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"),
            String::from("fgij")
        );
    }
}
