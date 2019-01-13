use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> String {
    // insert first character mention in the dependency pool
    // build BTreeMap of chars with BTreeSet of dependencies
    // iterate through the map
    // insert key to pool if dependencies are fufilled
    // exit the loop if there is no missing dependencies
    let mut result = String::new();
    let mut depended_by: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for line in input.lines() {
        if line.chars().count() > 0 {
            let parsed = line.split_whitespace().collect::<Vec<_>>();
            let entry = depended_by.entry(parsed[7]).or_insert_with(BTreeSet::new);
            entry.insert(parsed[1]);
        }
    }
    for dependencies in depended_by.iter() {
        println!("{:?} needs {:?}", dependencies.0, dependencies.1);
    }

    loop {
        let mut missing_dependencies = false;
        for dependencies in depended_by.iter() {
            if result.is_empty() {
                for entry in dependencies.1 {
                    result.push_str(entry);
                }
            }
            if result[..].contains(dependencies.0) {
                continue;
            }
            if has_dependencies(&result, &dependencies.1) {
                result.push_str(dependencies.0);
            } else {
                missing_dependencies = true;
            }
        }
        if !missing_dependencies {
            break;
        }
    }
    result
}

fn has_dependencies(result: &str, required: &BTreeSet<&str>) -> bool {
    let mut check = true;
    for entry in required {
        if !result[..].contains(entry) {
            check = false;
            break;
        }
    }
    check
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
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
