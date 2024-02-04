use chrono::naive;
use markdown::mdast;

#[derive(Debug, Eq, PartialEq)]
pub struct DayHeader {
    pub date: naive::NaiveDate,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TimeHeader {
    pub time: naive::NaiveTime,
}

#[derive(Debug, Eq, PartialEq)]
pub struct KindHeader {
    pub path: Vec<String>,
}

pub trait NodeExt {
    fn expect_root(self) -> mdast::Root;
    fn to_day_header(&self) -> Option<DayHeader>;
    fn to_time_header(&self) -> Option<TimeHeader>;
    fn to_kind_header(&self) -> Option<KindHeader>;
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
