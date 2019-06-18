use std::collections::HashMap;

#[allow(dead_code)]
pub fn star_one(input: &str) -> i64 {
    let mut old_gen = initial_state(input);
    let mut sum = 0;
    let patterns: HashMap<String, char> = input.lines().skip(2).map(|x| parse_pattern(x)).collect();

    let mut next_gen: String;

    println!("{} pots: {}", old_gen, sum);
    let mut origin_index = 0;
    for i in 0..20 {
        next_gen = proceed_generation(&old_gen, &patterns, &mut origin_index);
        old_gen = next_gen.clone();
        sum = 0;
        for (i, item) in old_gen.chars().enumerate() {
            if item == '#' {
                sum += i as i64 - origin_index;
            }
        }
        println!("{}: {} pots: {}", i + 1, next_gen, sum);
    }
    sum
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    let mut old_gen = initial_state(input);
    let mut sum: i64 = 0;
    let patterns: HashMap<String, char> = input.lines().skip(2).map(|x| parse_pattern(x)).collect();

    let mut next_gen: String;

    println!("{} pots: {}", old_gen, sum);
    let mut origin_index = 0;

    let mut old_sum = 0;
    let mut new_sum_diff: i64 = 0;
    let mut old_sum_diff = -1;

    for i in 0_i64..500000000 {
        if new_sum_diff != old_sum_diff || new_sum_diff == 1 {
            old_sum_diff = new_sum_diff;
            next_gen = proceed_generation(&old_gen, &patterns, &mut origin_index);
            old_gen = next_gen.clone();
            sum = 0;
            for (i, item) in old_gen.chars().enumerate() {
                if item == '#' {
                    sum += i as i64 - origin_index;
                }
            }
            new_sum_diff = sum - old_sum;
            old_sum = sum;
            println!("{}: pots: {} sum difference: {}", i + 1, sum, new_sum_diff);
        } else {
            sum += new_sum_diff * (50000000000 - i);
            break;
        }
    }
    sum
}

fn padded_index_check(input: &Vec<char>) -> i64 {
    let mut prefix = 4;
    for item in input.into_iter() {
        if *item == '#' || prefix == 0 {
            break;
        }
        prefix -= 1;
    }
    prefix
}

fn padded(input: &Vec<char>) -> Vec<char> {
    let mut prefix = 4;
    for item in input.into_iter() {
        if *item == '#' || prefix == 0 {
            break;
        }
        prefix -= 1;
    }

    let mut suffix = 4;
    for item in input.into_iter().rev() {
        if *item == '#' || suffix == 0 {
            break;
        }
        suffix -= 1;
    }

    let mut output: Vec<char> = Vec::new();
    let mut pre_vec = vec!['.'; prefix];
    let mut suf_vec = vec!['.'; suffix];

    output.append(&mut pre_vec);
    output.append(&mut input.clone());
    output.append(&mut suf_vec);
    output
}

fn proceed_generation(
    input: &str,
    patterns: &HashMap<String, char>,
    origin_index: &mut i64,
) -> String {
    *origin_index += padded_index_check(&input.chars().collect::<Vec<char>>());
    let padded_gen = padded(&input.chars().collect::<Vec<char>>());
    println!("origin_index is {}", origin_index);
    let mut next_gen = String::new();
    for i in 0..padded_gen.len() {
        let mut base = String::new();
        match i {
            0 => {
                base.push_str("..");
                base.push(padded_gen[i]);
                base.push(padded_gen[i + 1]);
                base.push(padded_gen[i + 2]);
            }
            1 => {
                base.push_str(".");
                base.push(padded_gen[i - 1]);
                base.push(padded_gen[i]);
                base.push(padded_gen[i + 1]);
                base.push(padded_gen[i + 2]);
            }
            x if x == padded_gen.len() - 1 => {
                base.push(padded_gen[i - 2]);
                base.push(padded_gen[i - 1]);
                base.push(padded_gen[i]);
                base.push_str("..");
            }
            x if x == padded_gen.len() - 2 => {
                base.push(padded_gen[i - 2]);
                base.push(padded_gen[i - 1]);
                base.push(padded_gen[i]);
                base.push(padded_gen[i + 1]);
                base.push_str(".");
            }
            _ => {
                base.push(padded_gen[i - 2]);
                base.push(padded_gen[i - 1]);
                base.push(padded_gen[i]);
                base.push(padded_gen[i + 1]);
                base.push(padded_gen[i + 2]);
            }
        }

        match patterns.get(&base) {
            Some(x) => {
                next_gen.push(*x);
            }
            None => {
                next_gen.push('.');
            }
        }
    }

    next_gen
}

fn parse_pattern(input: &str) -> (String, char) {
    let parsed: Vec<String> = input
        .replace(" => ", " ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    (parsed[0].clone(), parsed[1].clone().chars().nth(0).unwrap())
}

fn initial_state(input: &str) -> String {
    input
        .lines()
        .nth(0)
        .unwrap()
        .replace("initial state: ", "")
        .chars()
        .collect()
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
        assert_eq!(
            star_two(
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
            999999999374
        )
    }
}
