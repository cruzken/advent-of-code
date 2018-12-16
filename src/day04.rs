use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::repeat;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> u32 {
    let mut logs: Vec<LogEntry> = Vec::new();
    for line in input.lines() {
        logs.push(build_entry(line));
    }
    logs[..].sort();

    let sleep_table = build_sleep_table(&logs);

    let mut sleep_sums: HashMap<u32, u32> = HashMap::new();

    for minute in sleep_table.iter() {
        for (guard, freq) in minute.iter() {
            let minutes_sleep = sleep_sums.entry(*guard).or_insert(0);
            *minutes_sleep += freq;
        }
    }

    let mut sleepy_guard: Vec<_> = sleep_sums.iter().collect::<Vec<_>>();
    sleepy_guard.sort_by(|a, b| b.1.cmp(a.1));
    println!(
        "guard {} is the sleepiest with {} minutes",
        sleepy_guard[0].0, sleepy_guard[0].1
    );

    let mut opportunity_time: Option<(u32, u32)> = None;
    for i in 0..sleep_table.len() {
        if sleep_table[i].get(sleepy_guard[0].0) != None {
            let sleep_freq = sleep_table[i].get(sleepy_guard[0].0).unwrap();
            match opportunity_time {
                None => opportunity_time = Some((i as u32, *sleep_freq)),
                Some((x, y)) if *sleep_freq > y => opportunity_time = Some((i as u32, *sleep_freq)),
                Some(_) => {}
            }
        }
    }
    println!("{:?}", opportunity_time.unwrap());
    sleepy_guard[0].0 * opportunity_time.unwrap().0
}

fn build_sleep_table(logs: &Vec<LogEntry>) -> Vec<HashMap<u32, u32>> {
    let mut sleep_table = repeat(HashMap::<u32, u32>::new())
        .take(60)
        .collect::<Vec<_>>();

    let mut guard_state: Option<u32> = None;
    let mut fall_sleep_at: Option<u32> = None;

    for entry in logs {
        match entry.event {
            LogEvent::BeginShift(guard_num) => {
                guard_state = Some(guard_num);
            }
            LogEvent::FallsAsleep => {
                fall_sleep_at = Some(entry.date.minute);
            }
            LogEvent::WakesUp => {
                for item in sleep_table
                    .iter_mut()
                    .take(entry.date.minute as usize)
                    .skip(fall_sleep_at.unwrap() as usize)
                {
                    let counter = item.entry(guard_state.unwrap()).or_insert(0);
                    *counter += 1;
                }
            }
            _ => {}
        }
    }
    sleep_table
}

fn build_entry(input: &str) -> LogEntry {
    let parsed = parse_entry(input);
    LogEntry {
        date: LogDate {
            year: parsed[0].parse::<u32>().unwrap(),
            month: parsed[1].parse::<u32>().unwrap(),
            day: parsed[2].parse::<u32>().unwrap(),
            hour: parsed[3].parse::<u32>().unwrap(),
            minute: parsed[4].parse::<u32>().unwrap(),
        },
        event: match &parsed[5][..] {
            "Guard" => LogEvent::BeginShift(parsed[6].parse::<u32>().unwrap()),
            "falls" => LogEvent::FallsAsleep,
            "wakes" => LogEvent::WakesUp,
            _ => LogEvent::Unknown,
        },
    }
}
fn parse_entry(input: &str) -> Vec<String> {
    input
        .replace("[", "")
        .replace("-", " ")
        .replace(":", " ")
        .replace("]", "")
        .replace("#", "")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

#[derive(Debug)]
enum LogEvent {
    FallsAsleep,
    WakesUp,
    BeginShift(u32),
    Unknown,
}

#[derive(Debug)]
struct LogEntry {
    date: LogDate,
    event: LogEvent,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.date == other.date
    }
}

impl Eq for LogEntry {}

#[derive(Eq, Debug)]
struct LogDate {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Ord for LogDate {
    fn cmp(&self, other: &LogDate) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => match self.day.cmp(&other.day) {
                    Ordering::Equal => match self.hour.cmp(&other.hour) {
                        Ordering::Equal => self.minute.cmp(&other.minute),
                        _ => self.hour.cmp(&other.hour),
                    },
                    _ => self.day.cmp(&other.day),
                },
                _ => self.month.cmp(&other.month),
            },
            _ => self.year.cmp(&other.year),
        }
    }
}

impl PartialOrd for LogDate {
    fn partial_cmp(&self, other: &LogDate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogDate {
    fn eq(&self, other: &LogDate) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
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
                "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            ),
            240
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
