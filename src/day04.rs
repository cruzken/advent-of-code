use std::cmp::Ordering;

pub fn main() {}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    let mut logs: Vec<LogEntry> = Vec::new();
    for line in input.lines() {
        logs.push(build_entry(&parse_entry(line)));
    }
    logs[..].sort();
    
    for entry in &logs {
        println!("{:?}", entry);
    }
    0
}

fn build_entry(parsed: &[String]) -> LogEntry {
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
        .map(|x| String::from(x))
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
            1
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
