use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::dbs::page::Page;

lazy_static! {
    static ref FILTERING_REGEX: Regex = Regex::new(
        r"(?s)Balance Brought Forward\s(\d+\.\d{2})(.*?)Balance Carried Forward\s(\d+\.\d{2})",
    )
    .unwrap();
}

pub struct Document {
    pages: Vec<Page>,
}

impl FromStr for Document {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: FILTERING_REGEX
                .captures_iter(s)
                .map(|m| m.unwrap().get(0).unwrap().as_str())
                .map(Page::from_str)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pages.iter().map(Page::to_string).join("\n\n"))
    }
}
