use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> String {
    let mut order = String::new();
    let mut depended_by = dep_builder(input);
    let mut available: BTreeSet<&str> = BTreeSet::new();

    // 1. list all dependencies that start first and store in the available pool
    for (.., set) in depended_by.iter() {
        for entry in set.iter() {
            if !depended_by.contains_key(entry) {
                available.insert(entry);
            }
        }
    }

    // 2. repeat 3-6 until available pool is empty
    while !available.is_empty() {
        // 3. remove the lowest character in the available pool
        let clone = available.clone();
        let lowest = clone.iter().next().unwrap();
        available.remove(lowest);

        // 4. store that character in the order string.
        order.push_str(lowest);

        // 5. remove that character from all characters dependencies
        for (.., set) in &mut depended_by {
            set.remove(lowest);
        }

        // 6. remove all characters that don't have dependencies and store them into available pool
        for (key, set) in depended_by.clone() {
            if set.is_empty() {
                depended_by.remove(key);
                available.insert(key);
            }
        }
    }

    // 7. output the order string
    order
}

fn dep_builder(input: &str) -> BTreeMap<&str, HashSet<&str>> {
    let mut depended_by: BTreeMap<&str, HashSet<&str>> = BTreeMap::new();
    for line in input.lines() {
        if line.chars().count() > 0 {
            let parsed = line.split_whitespace().collect::<Vec<_>>();
            let entry = depended_by.entry(parsed[7]).or_insert_with(HashSet::new);
            entry.insert(parsed[1]);
        }
    }
    depended_by
}

trait Checker {
    fn contains_set(&self, set: &HashSet<&str>) -> bool;
}

impl Checker for String {
    fn contains_set(&self, set: &HashSet<&str>) -> bool {
        let mut check = true;
        for entry in set {
            if !self.contains(entry) {
                check = false;
                break;
            }
        }
        check
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    // 1. update tick
    // 2. subtract 1 second from all current workers working
    // 3. if worker is finished a step, place step to DONE pool and worker is idle
    // 4. remove the latest finished characters that are dependencies for all characters
    // 5. get all available chars and place in pool.
    // 6. push lowest character from pool to an available worker. Repeat until pool is empty or all workers busy.
    // 7. break loop when available pool is empty and all workers are idle
    // 8. output seconds ticked
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(
                "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            ),
            String::from("CABDFE")
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
