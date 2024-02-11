use std::collections::HashSet;

use chrono::naive;
use frozenset::Freeze;
use itertools::Itertools;

pub mod md;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Log {
    pub start: naive::NaiveDateTime,
    end: naive::NaiveDateTime,
    kinds: Kinds,
}

impl Log {
    fn duration(&self) -> chrono::TimeDelta {
        self.end - self.start
    }
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

pub fn total_by_day<'a>(
    logs: impl Iterator<Item = &'a Log>,
) -> Vec<(naive::NaiveDate, chrono::TimeDelta)> {
    logs.group_by(|l| l.start.date())
        .into_iter()
        .map(|(d, ls)| (d, ls.map(|l| l.duration()).sum()))
        .collect()
}

pub fn add_running_total<'a>(
    logs: impl Iterator<Item = &'a (naive::NaiveDate, chrono::TimeDelta)> + 'a,
) -> impl Iterator<Item = (naive::NaiveDate, chrono::TimeDelta, chrono::TimeDelta)> + 'a {
    logs.scan(chrono::TimeDelta::zero(), |state, (date, duration)| {
        *state += *duration;
        Some((*date, *duration, *state))
    })
}

pub fn target(incr: chrono::TimeDelta) -> impl Iterator<Item = chrono::TimeDelta> {
    std::ops::RangeFrom { start: 1 }.map(move |i| incr * i)
}

pub fn running_total_vs_target(
    logs: impl Iterator<Item = (naive::NaiveDate, chrono::TimeDelta, chrono::TimeDelta)>,
    target: impl Iterator<Item = chrono::TimeDelta>,
) -> impl Iterator<Item = (naive::NaiveDate, chrono::TimeDelta, chrono::TimeDelta)> {
    logs.zip(target)
        .map(|((date, total, running), target)| (date, total, running - target))
}

///
/// ```
/// let source = std::fs::read_to_string("example.md").unwrap();
/// let (logs, errors) = djot_log::parse_log(&source);
/// assert!(errors.is_empty());
/// assert_eq!(
///     logs.iter()
///         .map(|l| format!("{}", l))
///         .collect::<Vec<_>>()
///         .join("\n"),
///     "2023-12-03 09:00:00-13:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-03 14:00:00-15:00:00 Meeting // Work / MyOrg / MyDept
/// 2023-12-03 15:00:00-18:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-04 09:00:00-13:00:00 Coding // Work / MyOrg / MyDept / MyProj
/// 2023-12-04 14:00:00-18:00:00 Coding // Work / MyOrg / MyDept / MyProj"
/// )
/// ```
pub fn parse_log(s: &str) -> (Vec<Log>, Vec<String>) {
    let mut current_day: Option<naive::NaiveDate> = None;
    let mut start_time: Option<naive::NaiveDateTime> = None;
    let mut errors: Vec<String> = vec![];
    let mut kinds = HashSet::new();
    let mut logs = Vec::new();
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
                        logs.push(Log {
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
    (logs, errors)
}
