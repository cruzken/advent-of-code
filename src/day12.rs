use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    let state = initial_state(input);
    let patterns: HashMap<String, String> = input.lines().skip(2).map(|x| parse_pattern(x)).collect();
    println!("{:?}", patterns);
    0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn parse_pattern(input: &str) -> (String, String) {
    let parsed: Vec<String> = input.replace(" => ", " ").split_whitespace().map(|x| x.to_string()).collect();

    (parsed[0].clone(), parsed[1].clone())
}

fn initial_state(input: &str) -> String {
    input.lines().nth(0).unwrap().replace("initial state: ", "")
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(
                "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            ),
            325
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
