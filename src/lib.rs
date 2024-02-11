use std::collections::{BTreeMap, HashSet};

use chrono::naive;
use frozenset::Freeze;

pub mod md;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Log {
    pub start: naive::NaiveDateTime,
    end: naive::NaiveDateTime,
    kinds: Kinds,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Kinds {
    paths: frozenset::FrozenSet<Vec<String>>,
}

impl Kinds {
    fn new(paths: HashSet<Vec<String>>) -> Kinds {
        Kinds {
            paths: paths.freeze(),
        }
    }
}

impl std::fmt::Display for Kinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut paths = self.paths.iter().map(|p| p.join(" / ")).collect::<Vec<_>>();
        paths.sort();
        write!(f, "{}", paths.join(" // "))
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {}", self.start, self.end.time(), self.kinds)
    }
}

pub struct Logs {
    logs: HashSet<Log>,
}

impl Logs {
    fn sorted_logs(&self) -> Vec<Log> {
        let mut logs = self.logs.iter().cloned().collect::<Vec<_>>();
        logs.sort_by_key(|l| l.start);
        logs
    }

    pub fn to_plain_text(&self) -> String {
        self.sorted_logs()
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join("\n")
            .to_string()
    }

    pub fn total_by_day(&self) -> BTreeMap<naive::NaiveDate, chrono::Duration> {
        let mut days_to_total: BTreeMap<naive::NaiveDate, chrono::Duration> = BTreeMap::new();
        self.logs.iter().for_each(|l| {
            let day = &l.start.date();
            let previous_duration = *days_to_total.get(day).unwrap_or(&chrono::Duration::zero());
            days_to_total.insert(*day, previous_duration + (l.end - l.start));
        });
        days_to_total
    }

    pub fn accumulated_vs_target(
        &self,
        target: chrono::Duration,
    ) -> Vec<(naive::NaiveDate, chrono::Duration, chrono::Duration)> {
        self.total_by_day()
            .iter()
            .scan(
                (chrono::Duration::zero(), chrono::Duration::zero()),
                |(running_total, running_target), (&date, &total)| {
                    *running_total += total;
                    *running_target += target;
                    let vs_target = *running_total - *running_target;
                    Some((date, total, vs_target))
                },
            )
            .collect::<Vec<_>>()
    }
}

///
/// ```
/// let source = std::fs::read_to_string("example.md").unwrap();
/// let (logs, errors) = djot_log::parse_log(&source);
/// assert!(errors.is_empty());
/// assert_eq!(
///     logs.to_plain_text(),
///     "2023-12-03 09:00:00-13:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-03 14:00:00-15:00:00 Meeting // Work / MyOrg / MyDept
/// 2023-12-03 15:00:00-18:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-04 09:00:00-13:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-04 14:00:00-18:00:00 Coding // Work / MyOrg / MyDept / MyProj"
/// )
/// ```
pub fn parse_log(s: &str) -> (Logs, Vec<String>) {
    let mut current_day: Option<naive::NaiveDate> = None;
    let mut start_time: Option<naive::NaiveDateTime> = None;
    let mut errors: Vec<String> = vec![];
    let mut kinds = HashSet::new();
    let mut logs = HashSet::new();
    for n in md::parse_log_nodes(&md::parse_markdown(s)) {
        match n {
            md::LogNode::DayHeader(md::DayHeader { date }) => {
                current_day = Some(date);
            }
            md::LogNode::TimeHeader(md::TimeHeader { time }) => match start_time {
                None => match current_day {
                    Some(current_day) => {
                        start_time = Some(naive::NaiveDateTime::new(current_day, time));
                    }
                    None => {
                        errors.push(format!("Unexpected {:?} without preceding day header", n));
                    }
                },
                Some(start_time_) => {
                    let end = naive::NaiveDateTime::new(current_day.unwrap(), time);
                    if !kinds.is_empty() {
                        logs.insert(Log {
                            start: start_time_,
                            end,
                            kinds: Kinds::new(kinds),
                        });
                        kinds = HashSet::new();
                    }
                    start_time = Some(end);
                }
            },
            md::LogNode::KindHeader(md::KindHeader { ref path }) => match start_time {
                Some(_) => {
                    kinds.insert(path.clone());
                }
                None => {
                    errors.push(format!("Unexpected {:?} without start time set", n));
                }
            },
        }
    }
    (Logs { logs }, errors)
}
