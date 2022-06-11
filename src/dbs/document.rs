use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::dbs::page::Page;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?s)Balance Brought Forward\s(\d+\.\d{2})(.*?)Balance Carried Forward\s(\d+\.\d{2})",
    )
    .unwrap();
}

pub struct Document<'a> {
    pages: Vec<Page<'a>>,
}

impl FromStr for Document<'_> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: RE
                .captures_iter(s)
                .map(Result::unwrap)
                .map(|cap| Page::from_strs(&cap[1], &cap[3], &cap[2]))
                .collect(),
        })
    }
}

impl Display for Document<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pages.iter().map(Page::to_string).join("\n\n"))
    }
}
