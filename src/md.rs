use chrono::naive;
use markdown::mdast;

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
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
    fn get_first_text_value_of_header_of_depth(&self, header_depth: u8) -> Option<String>;
}

impl NodeExt for mdast::Node {
    /// ```
    /// use djot_log::md::NodeExt;
    /// assert_eq!(
    ///     djot_log::md::parse_markdown("# 2024-12-03\n").children[0].to_day_header(),
    ///     Some(djot_log::md::DayHeader {
    ///         date: chrono::naive::NaiveDate::parse_from_str("2024-12-03", "%Y-%m-%d").unwrap()
    ///     })
    /// );
    /// ```
    fn to_day_header(&self) -> Option<DayHeader> {
        Some(DayHeader {
            date: naive::NaiveDate::parse_from_str(
                &(self.get_first_text_value_of_header_of_depth(1)?),
                "%Y-%m-%d",
            )
            .ok()?,
        })
    }

    /// ```
    /// use djot_log::md::NodeExt;
    /// assert_eq!(
    ///     djot_log::md::parse_markdown("## 08:00\n").children[0].to_time_header(),
    ///     Some(djot_log::md::TimeHeader {
    ///         time: chrono::naive::NaiveTime::parse_from_str("08:00", "%H:%M").unwrap()
    ///     })
    /// );
    /// ```
    fn to_time_header(&self) -> Option<TimeHeader> {
        Some(TimeHeader {
            time: naive::NaiveTime::parse_from_str(
                &(self.get_first_text_value_of_header_of_depth(2)?),
                "%H:%M",
            )
            .ok()?,
        })
    }

    /// ```
    /// use djot_log::md::NodeExt;
    /// assert_eq!(
    ///     djot_log::md::parse_markdown("### Foo / Bar/Baz / Qux\n").children[0].to_kind_header(),
    ///     Some(djot_log::md::KindHeader {
    ///         path: vec!["Foo".into(), "Bar/Baz".into(), "Qux".into()]
    ///     })
    /// );
    /// ```
    fn to_kind_header(&self) -> Option<KindHeader> {
        Some(KindHeader {
            path: (self.get_first_text_value_of_header_of_depth(3))?
                .split(" / ")
                .map(std::string::ToString::to_string)
                .collect(),
        })
    }

    fn get_first_text_value_of_header_of_depth(&self, header_depth: u8) -> Option<String> {
        match self {
            mdast::Node::Heading(mdast::Heading {
                children,
                position: _,
                depth,
            }) if *depth == header_depth => {
                if let mdast::Node::Text(mdast::Text { value, .. }) = children.first()? {
                    Some(value.to_string())
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
                panic!("Expected root {self:?}");
            }
        }
    }
}

/// # Panics
///
/// If parsing as Markdown fails.
#[allow(clippy::must_use_candidate)]
pub fn parse_markdown(s: &str) -> mdast::Root {
    markdown::to_mdast(s, &markdown::ParseOptions::default())
        .unwrap()
        .expect_root()
}

/// ```
/// let source = std::fs::read_to_string("example.md").unwrap();
/// let debug = format!(
///     "{:?}",
///     djot_log::md::parse_log_nodes(&djot_log::md::parse_markdown(&source)).collect::<Vec<_>>()
/// )
/// .strip_prefix("[")
/// .unwrap()
/// .strip_suffix("]")
/// .unwrap()
/// .replace("), ", ")\n");
/// assert_eq!(
///     debug,
///     r##"DayHeader(DayHeader { date: 2023-12-03 })
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
/// TimeHeader(TimeHeader { time: 18:00:00 })"##
/// )
/// ```
pub fn parse_log_nodes(md: &mdast::Root) -> impl Iterator<Item = LogNode> + '_ {
    md.children.iter().filter_map(NodeExt::to_log_node)
}
