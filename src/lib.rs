use chrono::naive::NaiveDate;
use markdown::mdast;

#[derive(Debug, Eq, PartialEq)]
pub struct DayHeader {
    pub date: NaiveDate,
}

pub trait NodeExt {
    fn expect_root(self) -> mdast::Root;
    fn to_day_header(&self) -> Option<DayHeader>;
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
                    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
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
