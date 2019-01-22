use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
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

#[allow(dead_code)]
pub fn star_two(input: &str, workers: usize, time: u32) -> u32 {
    let mut order = String::new();
    let mut depended_by = dep_builder(input);
    let mut available: BTreeSet<&str> = BTreeSet::new();
    let mut ticks = 0;
    let mut workers = spawn_workers(workers);

    // 0. list all dependencies that start first and store in the available pool
    for (.., set) in depended_by.iter() {
        for entry in set.iter() {
            if !depended_by.contains_key(entry) {
                available.insert(entry);
            }
        }
    }

    loop {
        // 1. subtract 1 second from all current workers working
        for mut worker in workers.iter_mut() {
            if worker.step != None {
                worker.time -= 1;
            }
            // 2. if worker is finished a step, place step to DONE pool and worker is idle
            if worker.time == 0 && worker.step != None {
                let finished_step = worker.step.unwrap();
                order.push_str(finished_step);
                worker.set(None, 0);

                // 3. remove the finished step that are dependencies for all characters
                for (.., set) in &mut depended_by {
                    set.remove(finished_step);
                }
            }
        }
        // 4. get all available chars and place in pool.
        for (key, set) in depended_by.clone() {
            if set.is_empty() {
                depended_by.remove(key);
                available.insert(key);
            }
        }
        // 5. push lowest character from pool to an available worker. Repeat until pool is empty or all workers busy.
        for worker in workers.iter_mut().filter(|x| x.step == None) {
            if available.is_empty() {
                break;
            }
            let clone = available.clone();
            let lowest = clone.iter().next().unwrap();
            available.remove(lowest);
            worker.set(Some(lowest), calc_time(lowest, time)); // requires step time calculation function
        }
        // 6. break loop when available pool is empty and all workers are idle
        if available.is_empty()
            && workers.iter().filter(|x| x.step == None).count() == workers.len()
        {
            println!("loop is finished");
            break;
        }

        // 7. update tick
        ticks += 1;
    }
    // 8. output seconds ticked
    ticks
}

fn calc_time(step: &str, initial: u32) -> u32 {
    let points: HashMap<u8, u32> = (b'A'..=b'Z').zip(1..).collect();
    let query = step.chars().next().unwrap() as u8;
    let time = points.get(&query);
    match time {
        Some(x) => x + initial,
        None => 0,
    }
}

fn spawn_workers<'a>(num: usize) -> Vec<Worker<'a>> {
    let mut workers = Vec::new();
    while workers.len() < num {
        workers.push(Worker::new());
    }
    workers
}

struct Worker<'a> {
    step: Option<&'a str>,
    time: u32,
}

impl<'a> Worker<'a> {
    fn new() -> Worker<'a> {
        Worker {
            step: None,
            time: 0,
        }
    }

    fn set(&mut self, step: Option<&'a str>, count: u32) {
        self.step = step;
        self.time = count;
    }
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
        assert_eq!(
            star_two(
                "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
2, 0
            ),
            15
        )
    }
}
