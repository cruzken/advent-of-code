use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    let state = initial_state(input);
    let patterns: HashMap<String, String> =
        input.lines().skip(2).map(|x| parse_pattern(x)).collect();
    println!("{:?}", &state);
    let mut next_gen = String::new();
    for (i, val) in state.clone().into_iter().enumerate() {
        let mut base = String::new();
        match i {
            0 => {
                base.push_str("..");
                base.push(state[i]);
                base.push(state[i + 1]);
                base.push(state[i + 2]);
            }
            1 => {
                base.push_str(".");
                base.push(state[i - 1]);
                base.push(state[i]);
                base.push(state[i + 1]);
                base.push(state[i + 2]);
            }
            x if x == state.len() - 1 => {
                base.push(state[i - 2]);
                base.push(state[i - 1]);
                base.push(state[i]);
                base.push_str("..");
            }
            x if x == state.len() - 2 => {
                base.push(state[i - 2]);
                base.push(state[i - 1]);
                base.push(state[i]);
                base.push(state[i + 1]);
                base.push_str(".");
            }
            _ => {
                base.push(state[i - 2]);
                base.push(state[i - 1]);
                base.push(state[i]);
                base.push(state[i + 1]);
                base.push(state[i + 2]);
            }
        }

        println!("{}: {}", i, base);

        match patterns.get(&base) {
            Some(x) => {
                println!("Has match");
                next_gen.push('#');
            },
            None => {
                println!("No match");
                next_gen.push('.');
            },
        }
    }

    println!("{}", next_gen);
    0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn parse_pattern(input: &str) -> (String, String) {
    let parsed: Vec<String> = input
        .replace(" => ", " ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    (parsed[0].clone(), parsed[1].clone())
}

fn initial_state(input: &str) -> Vec<char> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .replace("initial state: ", "")
        .chars()
        .collect::<Vec<char>>()
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
