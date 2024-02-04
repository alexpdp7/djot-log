use std::collections::HashSet;

use chrono::naive;
use frozenset::Freeze;
use markdown::mdast;

#[derive(Clone, Debug)]
pub enum LogNode {
    DayHeader(DayHeader),
    TimeHeader(TimeHeader),
    KindHeader(KindHeader),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DayHeader {
    pub date: naive::NaiveDate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeHeader {
    pub time: naive::NaiveTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KindHeader {
    pub path: Vec<String>,
}

pub trait NodeExt {
    fn expect_root(self) -> mdast::Root;
    fn to_day_header(&self) -> Option<DayHeader>;
    fn to_time_header(&self) -> Option<TimeHeader>;
    fn to_kind_header(&self) -> Option<KindHeader>;
    fn to_log_node(&self) -> Option<LogNode>;
}

impl NodeExt for mdast::Node {
    /// ```
    /// use djot_log::NodeExt;
    /// assert_eq!(
    ///     djot_log::parse_markdown("# 2024-12-03\n").children[0].to_day_header(),
    ///     Some(djot_log::DayHeader {
    ///         date: chrono::naive::NaiveDate::parse_from_str("2024-12-03", "%Y-%m-%d").unwrap()
    ///     })
    /// );
    /// ```
    fn to_day_header(&self) -> Option<DayHeader> {
        match self {
            mdast::Node::Heading(mdast::Heading {
                children,
                position: _,
                depth: 1,
            }) => {
                if let [mdast::Node::Text(mdast::Text { value, .. }), ..] = children.as_slice() {
                    if let Ok(date) = naive::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                        Some(DayHeader { date })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// ```
    /// use djot_log::NodeExt;
    /// assert_eq!(
    ///     djot_log::parse_markdown("## 08:00\n").children[0].to_time_header(),
    ///     Some(djot_log::TimeHeader {
    ///         time: chrono::naive::NaiveTime::parse_from_str("08:00", "%H:%M").unwrap()
    ///     })
    /// );
    /// ```
    fn to_time_header(&self) -> Option<TimeHeader> {
        match self {
            mdast::Node::Heading(mdast::Heading {
                children,
                position: _,
                depth: 2,
            }) => {
                if let [mdast::Node::Text(mdast::Text { value, .. }), ..] = children.as_slice() {
                    if let Ok(time) = naive::NaiveTime::parse_from_str(value, "%H:%M") {
                        Some(TimeHeader { time })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// ```
    /// use djot_log::NodeExt;
    /// assert_eq!(
    ///     djot_log::parse_markdown("### Foo / Bar/Baz / Qux\n").children[0].to_kind_header(),
    ///     Some(djot_log::KindHeader {
    ///         path: vec!["Foo".into(), "Bar/Baz".into(), "Qux".into()]
    ///     })
    /// );
    /// ```
    fn to_kind_header(&self) -> Option<KindHeader> {
        match self {
            mdast::Node::Heading(mdast::Heading {
                children,
                position: _,
                depth: 3,
            }) => {
                if let [mdast::Node::Text(mdast::Text { value, .. }), ..] = children.as_slice() {
                    Some(KindHeader {
                        path: value.split(" / ").map(|x| x.to_string()).collect(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn to_log_node(&self) -> Option<LogNode> {
        [
            self.to_day_header().map(LogNode::DayHeader),
            self.to_time_header().map(LogNode::TimeHeader),
            self.to_kind_header().map(LogNode::KindHeader),
        ]
        .iter()
        .flatten()
        .next()
        .cloned()
    }

    fn expect_root(self) -> mdast::Root {
        match self {
            mdast::Node::Root(root) => root,
            _ => {
                panic!("Expected root {:?}", self);
            }
        }
    }
}

pub fn parse_markdown(s: &str) -> mdast::Root {
    markdown::to_mdast(s, &markdown::ParseOptions::default())
        .unwrap()
        .expect_root()
}

/// ```
/// let source = std::fs::read_to_string("example.md").unwrap();
/// let debug = format!("{:?}", djot_log::parse_log_nodes(&source))
///     .strip_prefix("[")
///     .unwrap()
///     .strip_suffix("]")
///     .unwrap()
///     .replace("), ", ")\n");
/// assert_eq!(debug, r##"DayHeader(DayHeader { date: 2023-12-03 })
/// TimeHeader(TimeHeader { time: 09:00:00 })
/// KindHeader(KindHeader { path: ["Work", "MyOrg", "MyDept", "MyProj"] })
/// KindHeader(KindHeader { path: ["Coding"] })
/// TimeHeader(TimeHeader { time: 13:00:00 })
/// TimeHeader(TimeHeader { time: 14:00:00 })
/// KindHeader(KindHeader { path: ["Work", "MyOrg", "MyDept"] })
/// KindHeader(KindHeader { path: ["Meeting"] })
/// TimeHeader(TimeHeader { time: 15:00:00 })
/// KindHeader(KindHeader { path: ["Work", "MyOrg", "MyDept", "MyProj"] })
/// KindHeader(KindHeader { path: ["Coding"] })
/// TimeHeader(TimeHeader { time: 18:00:00 })
/// DayHeader(DayHeader { date: 2023-12-04 })
/// TimeHeader(TimeHeader { time: 09:00:00 })
/// KindHeader(KindHeader { path: ["Work", "MyOrg", "MyDept", "MyProj"] })
/// KindHeader(KindHeader { path: ["Coding"] })
/// TimeHeader(TimeHeader { time: 13:00:00 })
/// TimeHeader(TimeHeader { time: 14:00:00 })
/// KindHeader(KindHeader { path: ["Work", "MyOrg", "MyDept", "MyProj"] })
/// KindHeader(KindHeader { path: ["Coding"] })
/// TimeHeader(TimeHeader { time: 18:00:00 })"##)
/// ```
pub fn parse_log_nodes(s: &str) -> Vec<LogNode> {
    parse_markdown(s)
        .children
        .iter()
        .flat_map(NodeExt::to_log_node)
        .collect::<Vec<_>>()
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Log {
    pub start: naive::NaiveDateTime,
    end: naive::NaiveDateTime,
    kinds: Kinds,
}

#[derive(PartialEq, Eq, Hash, Debug)]
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
        fn fmt_path(path: &Vec<String>) -> String {
            path.join(" / ")
        }

        let mut paths = self.paths.iter().map(fmt_path).collect::<Vec<_>>();
        paths.sort();
        write!(f, "{}", paths.join(" // "))
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {}", self.start, self.end.time(), self.kinds)
    }
}

pub fn parse_log(s: &str) -> (HashSet<Log>, Vec<String>) {
    let mut current_day: Option<naive::NaiveDate> = None;
    let mut start_time: Option<naive::NaiveDateTime> = None;
    let mut errors: Vec<String> = vec![];
    let mut kinds = HashSet::new();
    let mut logs = HashSet::new();
    for n in parse_log_nodes(s) {
        match n {
            LogNode::DayHeader(DayHeader { date }) => {
                current_day = Some(date);
            }
            LogNode::TimeHeader(TimeHeader { time }) => match start_time {
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
            LogNode::KindHeader(KindHeader { ref path }) => match start_time {
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
