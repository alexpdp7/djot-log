use chrono::naive;
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
            self.to_day_header().map(|dh| LogNode::DayHeader(dh)),
            self.to_time_header().map(|th| LogNode::TimeHeader(th)),
            self.to_kind_header().map(|kh| LogNode::KindHeader(kh)),
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
