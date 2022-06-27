use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::dbs::page::Page;
use crate::dbs::transactions::Transaction;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?s)Balance Brought Forward\s(\d{1,3}(?:\,\d{3})*\.\d{2})(.*?)Balance Carried Forward\s(\d{1,3}(?:\,\d{3})*\.\d{2})",
    )
    .unwrap();
    static ref DATE_REGEX: Regex = Regex::new(r"\d{1,2} ([A-z][a-z]{2}) (\d{4}) to \d{1,2} ([A-z][a-z]{2}) (\d{4})").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pages: Vec<Page>,
}

impl FromStr for Document {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(Some(captures)) = DATE_REGEX.captures(s) {
            let start = (&captures[1], &captures[2]);
            let end = (&captures[3], &captures[4]);
            Ok(Self {
                pages: RE
                    .captures_iter(s)
                    .filter_map(Result::ok)
                    .map(|cap| Page::from_strs(start, end, &cap[1], &cap[3], &cap[2]))
                    .collect(),
            })
        } else {
            Err(())
        }
    }
}

impl Document {
    pub fn transactions(&self) -> Vec<&Transaction> {
        self.pages
            .iter()
            .flat_map(|page| page.transactions().into_iter())
            .collect()
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_else(|_| "".to_string())
        )
    }
}
